#![cfg_attr(not(feature = "std"), no_std)]

use common::{balance, Balance, PSWAP, VAL, XOR};
use frame_support::{ensure, weights::Pays};
use sp_arithmetic::traits::Saturating;

mod benchmarking;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

type Assets<T> = assets::Module<T>;
type System<T> = frame_system::Module<T>;
type Technical<T> = technical::Module<T>;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

pub const TECH_ACCOUNT_PREFIX: &[u8] = b"faucet";
pub const TECH_ACCOUNT_MAIN: &[u8] = b"main";

pub fn balance_limit() -> Balance {
    balance!(100)
}

pub fn transfer_limit_block_count<T: frame_system::Config>() -> BlockNumberOf<T> {
    14400u32.into()
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use common::AccountIdOf;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + assets::Config + technical::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Transfers the specified amount of asset to the specified account.
        /// The supported assets are: XOR, VAL, PSWAP.
        ///
        /// # Errors
        ///
        /// AssetNotSupported is returned if `asset_id` is something the function doesn't support.
        /// AmountAboveLimit is returned if `target` has already received their daily limit of `asset_id`.
        /// NotEnoughReserves is returned if `amount` is greater than the reserves
        #[pallet::weight((0, Pays::No))]
        pub fn transfer(
            _origin: OriginFor<T>,
            asset_id: T::AssetId,
            target: AccountIdOf<T>,
            amount: Balance,
        ) -> DispatchResultWithPostInfo {
            Self::ensure_asset_supported(asset_id)?;
            let block_number = System::<T>::block_number();
            let (block_number, taken_amount) =
                Self::prepare_transfer(&target, asset_id, amount, block_number)?;
            let reserves_tech_account_id = Self::reserves_account_id();
            let reserves_account_id =
                Technical::<T>::tech_account_id_to_account_id(&reserves_tech_account_id)?;
            let reserves_amount = Assets::<T>::total_balance(&asset_id, &reserves_account_id)?;
            ensure!(amount <= reserves_amount, Error::<T>::NotEnoughReserves);
            technical::Module::<T>::transfer_out(
                &asset_id,
                &reserves_tech_account_id,
                &target,
                amount,
            )?;
            Transfers::<T>::insert(target.clone(), asset_id, (block_number, taken_amount));
            Self::deposit_event(Event::Transferred(target, amount));
            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::metadata(AccountIdOf<T> = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // The amount is transferred to the account. [account, amount]
        Transferred(AccountIdOf<T>, Balance),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Asset is not supported.
        AssetNotSupported,
        /// Amount is above limit.
        AmountAboveLimit,
        /// Not enough reserves.
        NotEnoughReserves,
    }

    #[pallet::storage]
    #[pallet::getter(fn reserves_account_id)]
    pub(super) type ReservesAcc<T: Config> = StorageValue<_, T::TechAccountId, ValueQuery>;

    #[pallet::storage]
    pub(super) type Transfers<T: Config> = StorageDoubleMap<
        _,
        Identity,
        T::AccountId,
        Blake2_256,
        T::AssetId,
        (BlockNumberOf<T>, Balance),
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub reserves_account_id: T::TechAccountId,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                reserves_account_id: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            ReservesAcc::<T>::put(&self.reserves_account_id);
        }
    }
}

impl<T: Config> Pallet<T> {
    fn ensure_asset_supported(asset_id: T::AssetId) -> Result<(), Error<T>> {
        let xor = XOR.into();
        let val = VAL.into();
        let pswap = PSWAP.into();
        if asset_id == xor || asset_id == val || asset_id == pswap {
            Ok(())
        } else {
            Err(Error::AssetNotSupported)
        }
    }

    /// Checks if new transfer is allowed, considering previous transfers.
    ///
    /// If new transfer is allowed, returns content to put in `Transfers` if the transfer is succeeded
    fn prepare_transfer(
        target: &T::AccountId,
        asset_id: T::AssetId,
        amount: Balance,
        current_block_number: BlockNumberOf<T>,
    ) -> Result<(BlockNumberOf<T>, Balance), Error<T>> {
        let balance_limit = balance_limit();
        ensure!(amount <= balance_limit, Error::AmountAboveLimit);
        if let Some((initial_block_number, taken_amount)) = Transfers::<T>::get(target, asset_id) {
            let transfer_limit_block_count = transfer_limit_block_count::<T>();
            if transfer_limit_block_count
                <= current_block_number.saturating_sub(initial_block_number)
            {
                // The previous transfer has happened a long time ago
                Ok((current_block_number, amount))
            } else if amount <= balance_limit.saturating_sub(taken_amount) {
                // Use `initial_block_number` because the previous transfer has happened recently.
                Ok((initial_block_number, taken_amount + amount))
            } else {
                Err(Error::<T>::AmountAboveLimit)
            }
        } else {
            Ok((current_block_number, amount))
        }
    }
}
