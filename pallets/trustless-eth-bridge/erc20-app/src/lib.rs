//! # ERC20
//!
//! An application that implements bridged ERC20 token assets.
//!
//! ## Overview
//!
//! ETH balances are stored in the tightly-coupled [`asset`] runtime module. When an account holder
//! burns some of their balance, a `Transfer` event is emitted. An external relayer will listen for
//! this event and relay it to the other chain.
//!
//! ## Interface
//!
//! ### Dispatchable Calls
//!
//! - `burn`: Burn an ERC20 token balance.
#![cfg_attr(not(feature = "std"), no_std)]

pub const TRANSFER_MAX_GAS: u64 = 100_000;

extern crate alloc;

mod payload;
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::ensure;
use frame_support::traits::EnsureOrigin;
use frame_system::ensure_signed;
use sp_core::{H160, U256};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

pub use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use crate::payload::*;

    use super::*;

    use assets::AssetIdOf;
    use bridge_types::traits::{AppRegistry, EvmBridgeApp, MessageStatusNotifier, OutboundChannel};
    use bridge_types::types::{
        AppKind, AssetKind, BridgeAppInfo, BridgeAssetInfo, CallOriginOutput,
    };
    use bridge_types::{EthNetworkId, H256};
    use common::{AssetName, AssetSymbol, Balance};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_system::{ensure_root, RawOrigin};
    use traits::currency::MultiCurrency;

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> = <<T as assets::Config>::Currency as MultiCurrency<AccountIdOf<T>>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config:
        frame_system::Config + assets::Config + permissions::Config + technical::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type OutboundChannel: OutboundChannel<Self::AccountId>;

        type CallOrigin: EnsureOrigin<
            Self::Origin,
            Success = CallOriginOutput<EthNetworkId, H160, H256>,
        >;

        type MessageStatusNotifier: MessageStatusNotifier<Self::AssetId, Self::AccountId>;

        type BridgeTechAccountId: Get<Self::TechAccountId>;

        type AppRegistry: AppRegistry;

        type WeightInfo: WeightInfo;
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// [network_id, asset_id, sender, recepient, amount]
        Burned(EthNetworkId, AssetIdOf<T>, T::AccountId, H160, BalanceOf<T>),
        /// [network_id, asset_id, sender, recepient, amount]
        Minted(EthNetworkId, AssetIdOf<T>, H160, T::AccountId, BalanceOf<T>),
    }

    #[pallet::storage]
    #[pallet::getter(fn app_address)]
    pub(super) type AppAddresses<T: Config> =
        StorageDoubleMap<_, Identity, EthNetworkId, Identity, AssetKind, H160, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn asset_kind)]
    pub(super) type AssetKinds<T: Config> =
        StorageDoubleMap<_, Identity, EthNetworkId, Identity, AssetIdOf<T>, AssetKind, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn token_address)]
    pub(super) type TokenAddresses<T: Config> =
        StorageDoubleMap<_, Identity, EthNetworkId, Identity, AssetIdOf<T>, H160, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn asset_by_address)]
    pub(super) type AssetsByAddresses<T: Config> =
        StorageDoubleMap<_, Identity, EthNetworkId, Identity, H160, AssetIdOf<T>, OptionQuery>;

    #[pallet::error]
    pub enum Error<T> {
        TokenIsNotRegistered,
        AppIsNotRegistered,
        NotEnoughFunds,
        InvalidNetwork,
        TokenAlreadyRegistered,
        AppAlreadyRegistered,
        /// Call encoding failed.
        CallEncodeFailed,
        /// Amount must be > 0
        WrongAmount,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// [network_id, contract, asset_kind]
        pub apps: Vec<(EthNetworkId, H160, AssetKind)>,
        /// [network_id, asset_id, asset_contract, asset_kind]
        pub assets: Vec<(EthNetworkId, AssetIdOf<T>, H160, AssetKind)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                apps: Default::default(),
                assets: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (network_id, contract, asset_kind) in self.apps.iter() {
                AppAddresses::<T>::insert(network_id, asset_kind, contract);
            }
            for (network_id, asset_id, contract, asset_kind) in self.assets.iter() {
                Pallet::<T>::register_asset_inner(*network_id, *asset_id, *contract, *asset_kind)
                    .unwrap();
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /*
        Internal calls to be used from Ethereum side.
        DON'T CHANGE ORDER
         */

        #[pallet::weight(<T as Config>::WeightInfo::mint())]

        pub fn mint(
            origin: OriginFor<T>,
            token: H160,
            sender: H160,
            recipient: <T::Lookup as StaticLookup>::Source,
            amount: U256,
        ) -> DispatchResult {
            let CallOriginOutput {
                network_id,
                message_id,
                contract,
                timestamp,
            } = T::CallOrigin::ensure_origin(origin.clone())?;
            let asset_id = AssetsByAddresses::<T>::get(network_id, token)
                // should never return this error, because called from Ethereum
                .ok_or(Error::<T>::TokenIsNotRegistered)?;
            let asset_kind = AssetKinds::<T>::get(network_id, &asset_id)
                .ok_or(Error::<T>::TokenIsNotRegistered)?;
            let app_address = AppAddresses::<T>::get(network_id, asset_kind)
                .ok_or(Error::<T>::AppIsNotRegistered)?;

            if contract != app_address {
                return Err(DispatchError::BadOrigin.into());
            }

            let bridge_account = Self::bridge_account()?;

            let amount: BalanceOf<T> = amount.as_u128().into();
            ensure!(amount > 0, Error::<T>::WrongAmount);
            let recipient = T::Lookup::lookup(recipient)?;
            match asset_kind {
                AssetKind::Thischain => {
                    assets::Pallet::<T>::transfer_from(
                        &asset_id,
                        &bridge_account,
                        &recipient,
                        amount,
                    )?;
                }
                AssetKind::Sidechain => {
                    assets::Pallet::<T>::mint_to(&asset_id, &bridge_account, &recipient, amount)?;
                }
            }
            T::MessageStatusNotifier::inbound_request(
                network_id,
                message_id,
                sender,
                recipient.clone(),
                asset_id,
                amount,
                timestamp,
            );
            Self::deposit_event(Event::Minted(
                network_id, asset_id, sender, recipient, amount,
            ));
            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::register_asset_internal())]

        pub fn register_asset_internal(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            contract: H160,
        ) -> DispatchResult {
            let CallOriginOutput {
                network_id,
                contract: app_contract,
                ..
            } = T::CallOrigin::ensure_origin(origin)?;
            let asset_kind = AppAddresses::<T>::iter_prefix(network_id)
                .find(|(_, address)| *address == app_contract)
                .ok_or(Error::<T>::AppIsNotRegistered)?
                .0;
            Self::register_asset_inner(network_id, asset_id, contract, asset_kind)?;
            Ok(())
        }

        /*
        Common exstrinsics
         */

        #[pallet::weight(<T as Config>::WeightInfo::burn())]
        pub fn burn(
            origin: OriginFor<T>,
            network_id: EthNetworkId,
            asset_id: AssetIdOf<T>,
            recipient: H160,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::burn_inner(who, network_id, asset_id, recipient, amount)?;

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::register_erc20_asset())]

        pub fn register_erc20_asset(
            origin: OriginFor<T>,
            network_id: EthNetworkId,
            address: H160,
            symbol: AssetSymbol,
            name: AssetName,
            decimals: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !AssetsByAddresses::<T>::contains_key(network_id, address),
                Error::<T>::TokenAlreadyRegistered
            );
            let target = AppAddresses::<T>::get(network_id, AssetKind::Sidechain)
                .ok_or(Error::<T>::AppIsNotRegistered)?;
            let bridge_account = Self::bridge_account()?;

            let asset_id = assets::Pallet::<T>::register_from(
                &bridge_account,
                symbol,
                name,
                decimals,
                Balance::from(0u32),
                true,
                None,
                None,
            )?;

            Self::register_asset_inner(network_id, asset_id, address, AssetKind::Sidechain)?;

            let message = RegisterErc20AssetPayload { address };

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                target,
                2000000u64.into(),
                &message.encode().map_err(|_| Error::<T>::CallEncodeFailed)?,
            )?;
            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::register_erc20_asset())]

        pub fn register_existing_erc20_asset(
            origin: OriginFor<T>,
            network_id: EthNetworkId,
            address: H160,
            asset_id: AssetIdOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !AssetsByAddresses::<T>::contains_key(network_id, address),
                Error::<T>::TokenAlreadyRegistered
            );
            let target = AppAddresses::<T>::get(network_id, AssetKind::Sidechain)
                .ok_or(Error::<T>::AppIsNotRegistered)?;

            Self::register_asset_inner(network_id, asset_id, address, AssetKind::Sidechain)?;

            let message = RegisterErc20AssetPayload { address };

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                target,
                2000000u64.into(),
                &message.encode().map_err(|_| Error::<T>::CallEncodeFailed)?,
            )?;
            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::register_native_asset())]

        pub fn register_native_asset(
            origin: OriginFor<T>,
            network_id: EthNetworkId,
            asset_id: AssetIdOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !TokenAddresses::<T>::contains_key(network_id, asset_id),
                Error::<T>::TokenAlreadyRegistered
            );
            let target = AppAddresses::<T>::get(network_id, AssetKind::Thischain)
                .ok_or(Error::<T>::AppIsNotRegistered)?;
            let (asset_symbol, asset_name, ..) = assets::Pallet::<T>::get_asset_info(&asset_id);

            let message = RegisterNativeAssetPayload {
                asset_id: asset_id.into(),
                name: asset_name.0,
                symbol: asset_symbol.0,
            };

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                target,
                2000000u64.into(),
                &message.encode().map_err(|_| Error::<T>::CallEncodeFailed)?,
            )?;
            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::register_native_app())]

        pub fn register_native_app(
            origin: OriginFor<T>,
            network_id: EthNetworkId,
            contract: H160,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !AppAddresses::<T>::contains_key(network_id, AssetKind::Thischain),
                Error::<T>::AppAlreadyRegistered
            );
            AppAddresses::<T>::insert(network_id, AssetKind::Thischain, contract);
            T::AppRegistry::register_app(network_id, contract)?;
            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::register_erc20_app())]

        pub fn register_erc20_app(
            origin: OriginFor<T>,
            network_id: EthNetworkId,
            contract: H160,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !AppAddresses::<T>::contains_key(network_id, AssetKind::Sidechain),
                Error::<T>::AppAlreadyRegistered
            );
            AppAddresses::<T>::insert(network_id, AssetKind::Sidechain, contract);
            T::AppRegistry::register_app(network_id, contract)?;
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn register_asset_inner(
            network_id: EthNetworkId,
            asset_id: AssetIdOf<T>,
            contract: H160,
            asset_kind: AssetKind,
        ) -> DispatchResult {
            ensure!(
                AppAddresses::<T>::contains_key(network_id, asset_kind),
                Error::<T>::AppIsNotRegistered
            );
            ensure!(
                !TokenAddresses::<T>::contains_key(network_id, asset_id),
                Error::<T>::TokenAlreadyRegistered
            );
            let bridge_account = Self::bridge_account()?;
            TokenAddresses::<T>::insert(network_id, asset_id, contract);
            AssetsByAddresses::<T>::insert(network_id, contract, asset_id);
            AssetKinds::<T>::insert(network_id, asset_id, asset_kind);

            // Err when permission already exists
            for permission_id in [permissions::BURN, permissions::MINT] {
                let _ = permissions::Pallet::<T>::assign_permission(
                    bridge_account.clone(),
                    &bridge_account,
                    permission_id,
                    permissions::Scope::Limited(common::hash(&asset_id)),
                );
            }
            Ok(())
        }

        fn bridge_account() -> Result<T::AccountId, DispatchError> {
            Ok(technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::BridgeTechAccountId::get(),
            )?)
        }

        pub fn burn_inner(
            who: T::AccountId,
            network_id: EthNetworkId,
            asset_id: AssetIdOf<T>,
            recipient: H160,
            amount: BalanceOf<T>,
        ) -> Result<H256, DispatchError> {
            ensure!(amount > 0, Error::<T>::WrongAmount);
            let asset_kind = AssetKinds::<T>::get(network_id, &asset_id)
                .ok_or(Error::<T>::TokenIsNotRegistered)?;
            let target = AppAddresses::<T>::get(network_id, asset_kind)
                .ok_or(Error::<T>::AppIsNotRegistered)?;
            let bridge_account = Self::bridge_account()?;

            match asset_kind {
                AssetKind::Sidechain => {
                    assets::Pallet::<T>::burn_from(&asset_id, &bridge_account, &who, amount)?;
                }
                AssetKind::Thischain => {
                    assets::Pallet::<T>::transfer_from(&asset_id, &who, &bridge_account, amount)?;
                }
            }

            let token_address = TokenAddresses::<T>::get(network_id, &asset_id)
                .ok_or(Error::<T>::TokenIsNotRegistered)?;

            let message = MintPayload {
                token: token_address,
                sender: who.clone(),
                recipient: recipient.clone(),
                amount: amount.into(),
            };

            let message_id = T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Signed(who.clone()),
                target,
                TRANSFER_MAX_GAS.into(),
                &message.encode().map_err(|_| Error::<T>::CallEncodeFailed)?,
            )?;
            T::MessageStatusNotifier::outbound_request(
                network_id,
                message_id,
                who.clone(),
                recipient,
                asset_id,
                amount,
            );
            Self::deposit_event(Event::Burned(network_id, asset_id, who, recipient, amount));

            Ok(message_id)
        }
    }

    impl<T: Config> EvmBridgeApp<T::AccountId, T::AssetId, Balance> for Pallet<T> {
        fn is_asset_supported(network_id: EthNetworkId, asset_id: T::AssetId) -> bool {
            TokenAddresses::<T>::get(network_id, asset_id).is_some()
        }

        fn transfer(
            network_id: EthNetworkId,
            asset_id: T::AssetId,
            sender: T::AccountId,
            recipient: H160,
            amount: Balance,
        ) -> Result<H256, DispatchError> {
            Pallet::<T>::burn_inner(sender, network_id, asset_id, recipient, amount)
        }

        fn list_supported_assets(network_id: EthNetworkId) -> Vec<BridgeAssetInfo<T::AssetId>> {
            AssetKinds::<T>::iter_prefix(network_id)
                .map(|(asset_id, asset_kind)| {
                    let app_kind = match asset_kind {
                        AssetKind::Thischain => AppKind::SidechainApp,
                        AssetKind::Sidechain => AppKind::ERC20App,
                    };
                    TokenAddresses::<T>::get(network_id, &asset_id)
                        .map(|evm_address| {
                            Some(BridgeAssetInfo {
                                asset_id,
                                app_kind,
                                evm_address: Some(evm_address),
                            })
                        })
                        .unwrap_or_default()
                })
                .flatten()
                .collect()
        }

        fn list_apps(network_id: EthNetworkId) -> Vec<BridgeAppInfo> {
            AppAddresses::<T>::iter_prefix(network_id)
                .map(|(asset_kind, evm_address)| {
                    let app_kind = match asset_kind {
                        AssetKind::Thischain => AppKind::SidechainApp,
                        AssetKind::Sidechain => AppKind::ERC20App,
                    };
                    BridgeAppInfo {
                        app_kind,
                        evm_address,
                    }
                })
                .collect()
        }
    }
}
