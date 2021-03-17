#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use common::{prelude::Balance, FromGenericPair, SwapAction, SwapRulesValidation};
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::{ensure, weights::Weight, Parameter};
use frame_system::ensure_signed;
use sp_runtime::traits::{MaybeSerializeDeserialize, Member};
use sp_runtime::RuntimeDebug;

use common::TECH_ACCOUNT_MAGIC_PREFIX;
use sp_core::H256;
use sp_std::convert::TryFrom;

mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type TechAccountIdOf<T> = <T as Config>::TechAccountId;
type AssetIdOf<T> = <T as assets::Config>::AssetId;
type TechAssetIdOf<T> = <T as Config>::TechAssetId;
type DEXIdOf<T> = <T as common::Config>::DEXId;

/// Pending atomic swap operation.
#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct PendingSwap<T: Config> {
    /// Source of the swap.
    pub source: T::AccountId,
    /// Action of this swap.
    pub action: T::SwapAction,
    /// Condition is time or block number, or something logical.
    pub condition: T::Condition,
}

pub trait WeightInfo {
    fn create_swap() -> Weight;
}

pub fn tech_account_id_encoded_to_account_id_32(tech_account_id: &[u8]) -> H256 {
    use ::core::hash::Hasher;
    let mut h0 = twox_hash::XxHash::with_seed(0);
    let mut h1 = twox_hash::XxHash::with_seed(1);
    h0.write(tech_account_id);
    h1.write(tech_account_id);
    let r0 = h0.finish();
    let r1 = h1.finish();
    let mut repr: H256 = H256::zero();
    repr[..16].copy_from_slice(&TECH_ACCOUNT_MAGIC_PREFIX);
    repr[16..24].copy_from_slice(&r0.to_le_bytes());
    repr[24..].copy_from_slice(&r1.to_le_bytes());
    repr
}

impl<T: Config> Pallet<T> {
    /// Perform creation of swap, version without validation
    pub fn perform_create_swap_unchecked(
        source: AccountIdOf<T>,
        action: &T::SwapAction,
    ) -> DispatchResult {
        common::with_transaction(|| {
            action.reserve(&source)?;
            if action.is_able_to_claim() {
                if action.instant_auto_claim_used() {
                    if action.claim(&source) {
                        Self::deposit_event(Event::SwapSuccess(source));
                    } else if !action.triggered_auto_claim_used() {
                        action.cancel(&source);
                    } else {
                        return Err(Error::<T>::NotImplemented)?;
                    }
                } else {
                    return Err(Error::<T>::NotImplemented)?;
                }
            } else if action.triggered_auto_claim_used() {
                return Err(Error::<T>::NotImplemented)?;
            } else {
                return Err(Error::<T>::NotImplemented)?;
            }
            Ok(())
        })
    }

    /// Perform creation of swap, may be used by extrinsic operation or other pallets.
    pub fn perform_create_swap(
        source: AccountIdOf<T>,
        action: &mut T::SwapAction,
    ) -> DispatchResult {
        ensure!(
            !action.is_abstract_checking(),
            Error::<T>::OperationWithAbstractCheckingIsImposible
        );
        action.prepare_and_validate(Some(&source))?;
        Module::<T>::perform_create_swap_unchecked(source, action)
    }

    /// Creates an `T::AccountId` based on `T::TechAccountId`.
    ///
    /// This function works under assumption that `T::AccountId` is essentially 32-byte array
    /// (e.g. `AccountId32`).
    pub fn tech_account_id_to_account_id(
        tech_account_id: &T::TechAccountId,
    ) -> Result<T::AccountId, DispatchError> {
        let repr = tech_account_id_encoded_to_account_id_32(&tech_account_id.encode());
        T::AccountId::decode(&mut &repr[..]).map_err(|_| Error::<T>::DecodeAccountIdFailed.into())
    }

    /// Lookups registered `TechAccountId` for the given `AccountId`.
    pub fn lookup_tech_account_id(
        account_id: &T::AccountId,
    ) -> Result<T::TechAccountId, DispatchError> {
        Self::tech_account(account_id).ok_or(Error::<T>::AssociatedAccountIdNotFound.into())
    }

    /// Check `TechAccountId` for registration in storage map.
    pub fn ensure_account_registered(
        account_id: &T::AccountId,
    ) -> Result<T::TechAccountId, DispatchError> {
        Self::lookup_tech_account_id(account_id)
            .map_err(|_| Error::<T>::TechAccountIdIsNotRegistered.into())
    }

    /// Check `TechAccountId` for registration in storage map.
    pub fn ensure_tech_account_registered(tech_account_id: &T::TechAccountId) -> DispatchResult {
        let account_id = Self::tech_account_id_to_account_id(tech_account_id)?;
        Self::ensure_account_registered(&account_id).map(|_| ())
    }

    /// Register `TechAccountId` in storate map.
    pub fn register_tech_account_id(tech_account_id: T::TechAccountId) -> DispatchResult {
        let account_id = Self::tech_account_id_to_account_id(&tech_account_id)?;
        TechAccounts::<T>::insert(account_id, tech_account_id);
        Ok(())
    }

    /// Register `TechAccountId` in storate map if it not exist.
    pub fn register_tech_account_id_if_not_exist(
        tech_account_id: &T::TechAccountId,
    ) -> DispatchResult {
        let account_id = Self::tech_account_id_to_account_id(tech_account_id)?;
        match Self::lookup_tech_account_id(&account_id) {
            Err(_) => {
                TechAccounts::<T>::insert(account_id, tech_account_id.clone());
            }
            _ => (),
        }
        Ok(())
    }

    /// Set storage changes in assets to transfer specific asset from regular `AccountId` into pure `TechAccountId`.
    pub fn transfer_in(
        asset: &AssetIdOf<T>,
        source: &T::AccountId,
        tech_dest: &T::TechAccountId,
        amount: Balance,
    ) -> DispatchResult {
        let to = Self::tech_account_id_to_account_id(tech_dest)?;
        Self::ensure_account_registered(&to)?;
        assets::Module::<T>::transfer_from(asset, source, &to, amount)?;
        Ok(())
    }

    /// Set storage changes in assets to transfer specific asset from pure `TechAccountId` into pure `AccountId`.
    pub fn transfer_out(
        asset: &AssetIdOf<T>,
        tech_source: &T::TechAccountId,
        to: &<T as frame_system::Config>::AccountId,
        amount: Balance,
    ) -> DispatchResult {
        let from = Self::tech_account_id_to_account_id(tech_source)?;
        Self::ensure_account_registered(&from)?;
        assets::Module::<T>::transfer_from(asset, &from, to, amount)?;
        Ok(())
    }

    /// Transfer specific asset from pure `TechAccountId` into pure `TechAccountId`.
    pub fn transfer(
        asset: &AssetIdOf<T>,
        tech_source: &T::TechAccountId,
        tech_dest: &T::TechAccountId,
        amount: Balance,
    ) -> DispatchResult {
        let from = Self::tech_account_id_to_account_id(tech_source)?;
        Self::ensure_account_registered(&from)?;
        let to = Self::tech_account_id_to_account_id(tech_dest)?;
        Self::ensure_account_registered(&to)?;
        assets::Module::<T>::transfer_from(asset, &from, &to, amount)
    }

    /// Mint specific asset to the given `TechAccountId`.
    pub fn mint(
        asset: &AssetIdOf<T>,
        tech_dest: &T::TechAccountId,
        amount: Balance,
    ) -> DispatchResult {
        let account_id = Self::tech_account_id_to_account_id(tech_dest)?;
        Self::ensure_account_registered(&account_id)?;
        assets::Module::<T>::mint_to(asset, &account_id, &account_id, amount)
    }

    /// Burn specific asset from the given `TechAccountId`.
    pub fn burn(
        asset: &AssetIdOf<T>,
        tech_dest: &T::TechAccountId,
        amount: Balance,
    ) -> DispatchResult {
        let account_id = Self::tech_account_id_to_account_id(tech_dest)?;
        Self::ensure_account_registered(&account_id)?;
        assets::Module::<T>::burn_from(asset, &account_id, &account_id, amount)
    }

    /// Burn specific asset from the given `TechAccountId`.
    pub fn total_balance(
        asset_id: &T::AssetId,
        tech_id: &T::TechAccountId,
    ) -> Result<Balance, DispatchError> {
        let account_id = Self::tech_account_id_to_account_id(tech_id)?;
        Self::ensure_account_registered(&account_id)?;
        assets::Module::<T>::total_balance(asset_id, &account_id)
    }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + common::Config + assets::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Like Asset but deterministically maked from purpose.
        type TechAssetId: Copy
            + Ord
            + Member
            + Parameter
            + Into<AssetIdOf<Self>>
            + TryFrom<AssetIdOf<Self>>;

        /// Like AccountId but controlled by consensus, not signing by user.
        /// This extra traits exist here bacause no way to do it by constraints, problem exist with
        /// constraints and substrate macroses, and place this traits here is solution.
        type TechAccountId: Ord
            + Member
            + Parameter
            + Default
            + FromGenericPair
            + MaybeSerializeDeserialize
            + common::ToFeeAccount
            + common::ToTechUnitFromDEXAndTradingPair<
                DEXIdOf<Self>,
                common::TradingPair<TechAssetIdOf<Self>>,
            > + Into<common::TechAccountId<Self::AccountId, Self::TechAssetId, Self::DEXId>>;

        /// Trigger for auto claim.
        type Trigger: Default + Copy + Member + Parameter;

        /// Condition for auto claim.
        type Condition: Default + Copy + Member + Parameter;

        /// Swap action.
        type SwapAction: common::SwapRulesValidation<Self::AccountId, Self::TechAccountId, Self>
            + Parameter;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(<T as Config>::WeightInfo::create_swap())]
        pub(crate) fn create_swap(
            origin: OriginFor<T>,
            action: T::SwapAction,
        ) -> DispatchResultWithPostInfo {
            let source = ensure_signed(origin)?;
            let mut action_mut = action;
            Module::<T>::perform_create_swap(source, &mut action_mut)?;
            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::metadata(AccountIdOf<T> = "AccountId", TechAssetIdOf<T> = "TechAssetId", TechAccountIdOf<T> = "TechAccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Some pure technical assets were minted. [asset, owner, minted_amount, total_exist].
        /// This is not only for pure TechAccountId.
        /// TechAccountId can be just wrapped AccountId.
        Minted(TechAssetIdOf<T>, TechAccountIdOf<T>, Balance, Balance),

        /// Some pure technical assets were burned. [asset, owner, burned_amount, total_exist].
        /// For full kind of accounts like in Minted.
        Burned(TechAssetIdOf<T>, TechAccountIdOf<T>, Balance, Balance),

        /// Some assets were transferred out. [asset, from, to, amount].
        /// TechAccountId is only pure TechAccountId.
        OutputTransferred(
            TechAssetIdOf<T>,
            TechAccountIdOf<T>,
            AccountIdOf<T>,
            Balance,
        ),

        /// Some assets were transferred in. [asset, from, to, amount].
        /// TechAccountId is only pure TechAccountId.
        InputTransferred(
            TechAssetIdOf<T>,
            AccountIdOf<T>,
            TechAccountIdOf<T>,
            Balance,
        ),

        /// Swap operaction is finalised [initiator, finaliser].
        /// TechAccountId is only pure TechAccountId.
        SwapSuccess(AccountIdOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        /// Balance too low to send value.
        InsufficientBalance,
        /// Swap already exists.
        AlreadyExist,
        /// Swap proof is invalid.
        InvalidProof,
        /// Source does not match.
        SourceMismatch,
        /// Swap has already been claimed.
        AlreadyClaimed,
        /// Claim action mismatch.
        ClaimActionMismatch,
        /// Duration has not yet passed for the swap to be cancelled.
        DurationNotPassed,
        /// If argument must be technical, and only regular values inside it is allowed
        OnlyRegularAsset,
        /// If argument must be technical, and only regular values inside it is allowed
        OnlyRegularAccount,
        /// If argument must be technical, and only regular values inside it is allowed
        OnlyRegularBalance,
        /// If argument must be technical, and only regular values inside it is allowed
        OnlyPureTechnicalAccount,
        /// Got an overflow after adding.
        Overflow,
        /// If argument must be technical, and only pure technical value is allowed
        TechAccountIdMustBePure,
        /// It is not posible to extract code from `AccountId32` as representation
        /// or find it in storage.
        UnableToGetReprFromTechAccountId,
        /// Type must sport mapping from hash to special subset of `AccountId32`
        RepresentativeMustBeSupported,
        /// It is not posible to find record in storage map about `AccountId32` representation for
        /// technical account.
        TechAccountIdIsNotRegistered,
        /// This function or ablility is still not implemented.
        NotImplemented,
        /// Failed to decode `AccountId` from a hash.
        DecodeAccountIdFailed,
        /// Associated `AccountId` not found with a given `TechnicalAccountId`.
        AssociatedAccountIdNotFound,
        /// Operation with abstract checking is impossible.
        OperationWithAbstractCheckingIsImposible,
    }

    /// Registered technical account identifiers. Map from repr `AccountId` into pure `TechAccountId`.
    #[pallet::storage]
    #[pallet::getter(fn tech_account)]
    pub(super) type TechAccounts<T: Config> =
        StorageMap<_, Blake2_128Concat, AccountIdOf<T>, TechAccountIdOf<T>>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// Registered technical account identifiers. Map from repr `AccountId` into pure `TechAccountId`.
        pub account_ids_to_tech_account_ids: Vec<(AccountIdOf<T>, TechAccountIdOf<T>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                account_ids_to_tech_account_ids: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.account_ids_to_tech_account_ids
                .iter()
                .for_each(|(k, v)| {
                    TechAccounts::<T>::insert(k, v);
                });
        }
    }
}
