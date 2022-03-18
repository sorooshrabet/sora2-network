//! Demeter farming platform module benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use codec::Decode;
use common::{balance, FromGenericPair, CERES_ASSET_ID, XOR};
use frame_benchmarking::benchmarks;
use frame_system::{EventRecord, RawOrigin};
use hex_literal::hex;
use sp_std::prelude::*;

use crate::Pallet as DemeterFarmingPlatform;
use assets::Module as Assets;
use technical::Module as Technical;

// Support Functions
fn alice<T: Config>() -> T::AccountId {
    let bytes = hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
    T::AccountId::decode(&mut &bytes[..]).unwrap_or_default()
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Module::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

benchmarks! {
    get_rewards {
        let caller = alice::<T>();
        frame_system::Pallet::<T>::inc_providers(&caller);
        let rewards = balance!(100);
        let is_farm = true;

        let assets_and_permissions_tech_account_id =
            T::TechAccountId::from_generic_pair(b"SYSTEM_ACCOUNT".to_vec(), b"ASSETS_PERMISSIONS".to_vec());
        let assets_and_permissions_account_id =
            Technical::<T>::tech_account_id_to_account_id(
                &assets_and_permissions_tech_account_id,
            ).unwrap();

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            balance!(20000)
        );

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            XOR.into(),
            caller.clone(),
            balance!(20000)
        );

        // Add pool
        let _ = DemeterFarmingPlatform::<T>::add_pool(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            true,
            2,
            2,
            true
        );

        // Get rewards
        let _ = DemeterFarmingPlatform::<T>::get_rewards(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            is_farm
        );
    }: _(RawOrigin::Signed(caller.clone()), XOR.into(), CERES_ASSET_ID.into(), is_farm)
    verify {
        assert_last_event::<T>(Event::RewardWithdrawn(caller, rewards, XOR.into(), CERES_ASSET_ID.into(), is_farm).into());
    }

    withdraw {
        let caller = alice::<T>();
        frame_system::Pallet::<T>::inc_providers(&caller);
        let is_farm = true;

        let assets_and_permissions_tech_account_id =
            T::TechAccountId::from_generic_pair(b"SYSTEM_ACCOUNT".to_vec(), b"ASSETS_PERMISSIONS".to_vec());
        let assets_and_permissions_account_id =
            Technical::<T>::tech_account_id_to_account_id(
                &assets_and_permissions_tech_account_id,
            ).unwrap();

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            balance!(20000)
        );

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            XOR.into(),
            caller.clone(),
            balance!(20000)
        );

        //Add pool
        let _ = DemeterFarmingPlatform::<T>::add_pool(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            true,
            2,
            2,
            true
        );

        let pooled_tokens = balance!(30);

        // Deposit
        let _ = DemeterFarmingPlatform::<T>::deposit(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            is_farm,
            pooled_tokens
        );

        // Withdraw
        let _ = DemeterFarmingPlatform::<T>::withdraw(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            pooled_tokens,
            is_farm
        );
    }: _(RawOrigin::Signed(caller.clone()), XOR.into(), CERES_ASSET_ID.into(), pooled_tokens, is_farm)
    verify {
        assert_last_event::<T>(Event::Withdrawn(caller, pooled_tokens, XOR.into(), CERES_ASSET_ID.into(), is_farm).into());
        }

    remove_pool{
        let caller = AuthorityAccount::<T>::get();
        frame_system::Pallet::<T>::inc_providers(&caller);
        let is_farm = true;

        let assets_and_permissions_tech_account_id =
            T::TechAccountId::from_generic_pair(b"SYSTEM_ACCOUNT".to_vec(), b"ASSETS_PERMISSIONS".to_vec());
        let assets_and_permissions_account_id =
            Technical::<T>::tech_account_id_to_account_id(
                &assets_and_permissions_tech_account_id,
            ).unwrap();

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            balance!(20000)
        );

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            XOR.into(),
            caller.clone(),
            balance!(20000)
        );

        //Add pool
        let _ = DemeterFarmingPlatform::<T>::add_pool(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            true,
            2,
            2,
            true
        );

        // Remove pool
        let _ = DemeterFarmingPlatform::<T>::remove_pool(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            is_farm
        );
    }: _(RawOrigin::Signed(caller.clone()), XOR.into(), CERES_ASSET_ID.into(), is_farm)
    verify {
        assert_last_event::<T>(Event::PoolRemoved(caller, XOR.into(), CERES_ASSET_ID.into(), is_farm).into());
        }

    change_pool_multiplier {
        let caller = AuthorityAccount::<T>::get();
        frame_system::Pallet::<T>::inc_providers(&caller);
        let is_farm = true;

        let assets_and_permissions_tech_account_id =
            T::TechAccountId::from_generic_pair(b"SYSTEM_ACCOUNT".to_vec(), b"ASSETS_PERMISSIONS".to_vec());
        let assets_and_permissions_account_id =
            Technical::<T>::tech_account_id_to_account_id(
                &assets_and_permissions_tech_account_id,
            ).unwrap();

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            balance!(20000)
        );

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            XOR.into(),
            caller.clone(),
            balance!(20000)
        );

        // Register token
        let _ = DemeterFarmingPlatform::<T>::register_token(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            balance!(1),
            balance!(0.4),
            balance!(0.2),
            balance!(0.2)
        );

        // Add pool
        let _ = DemeterFarmingPlatform::<T>::add_pool(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            true,
            2,
            2,
            true
        );

        let new_multiplier = 2;

        // Change pool multiplier
        let _ = DemeterFarmingPlatform::<T>::change_pool_multiplier(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            is_farm,
            new_multiplier

        );
    }: _(RawOrigin::Signed(caller.clone()), XOR.into(), CERES_ASSET_ID.into(), is_farm, new_multiplier)
    verify {
        assert_last_event::<T>(Event::MultiplierChanged(caller, XOR.into(), CERES_ASSET_ID.into(), is_farm, new_multiplier).into());
        }

    change_pool_deposit_fee {
        let caller = AuthorityAccount::<T>::get();
        frame_system::Pallet::<T>::inc_providers(&caller);
        let is_farm = true;

        let assets_and_permissions_tech_account_id =
            T::TechAccountId::from_generic_pair(b"SYSTEM_ACCOUNT".to_vec(), b"ASSETS_PERMISSIONS".to_vec());
        let assets_and_permissions_account_id =
            Technical::<T>::tech_account_id_to_account_id(
                &assets_and_permissions_tech_account_id,
            ).unwrap();

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            balance!(20000)
        );

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            XOR.into(),
            caller.clone(),
            balance!(20000)
        );

        // Add pool
        let _ = DemeterFarmingPlatform::<T>::add_pool(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            true,
            2,
            2,
            true
        );

        let deposit_fee = balance!(1);

        // Change pool deposit fee
        let _ = DemeterFarmingPlatform::<T>::change_pool_deposit_fee(
            RawOrigin::Signed(caller.clone()).into(),
            XOR.into(),
            CERES_ASSET_ID.into(),
            is_farm,
            deposit_fee
        );
    }: _(RawOrigin::Signed(caller.clone()), XOR.into(), CERES_ASSET_ID.into(), is_farm, deposit_fee)
    verify {
        assert_last_event::<T>(Event::DepositFeeChanged(caller, XOR.into(), CERES_ASSET_ID.into(), is_farm, deposit_fee).into());
        }

    change_token_info {
        let caller = AuthorityAccount::<T>::get();
        frame_system::Pallet::<T>::inc_providers(&caller);
        let token_per_block = balance!(1);
        let farms_allocation = balance!(0.2);
        let staking_allocation = balance!(0.4);
        let team_allocation = balance!(0.4);

        let assets_and_permissions_tech_account_id =
            T::TechAccountId::from_generic_pair(b"SYSTEM_ACCOUNT".to_vec(), b"ASSETS_PERMISSIONS".to_vec());
        let assets_and_permissions_account_id =
            Technical::<T>::tech_account_id_to_account_id(
                &assets_and_permissions_tech_account_id,
            ).unwrap();

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            CERES_ASSET_ID.into(),
            caller.clone(),
            balance!(20000)
        );

        let _ = Assets::<T>::mint(
            RawOrigin::Signed(assets_and_permissions_account_id.clone()).into(),
            XOR.into(),
            caller.clone(),
            balance!(20000)
        );

        // Register token
        let _ = DemeterFarmingPlatform::<T>::register_token(
            RawOrigin::Signed(caller.clone()).into(),
            CERES_ASSET_ID.into(),
            token_per_block,
            farms_allocation,
            staking_allocation,
            team_allocation
        );

        let deposit_fee = balance!(1);

        // Change token info
        let _ = DemeterFarmingPlatform::<T>::change_token_info(
            RawOrigin::Signed(caller.clone()).into(),
            CERES_ASSET_ID.into(),
            token_per_block,
            farms_allocation,
            staking_allocation,
            team_allocation
        );

    }: _(RawOrigin::Signed(caller.clone()), CERES_ASSET_ID.into(), token_per_block, farms_allocation, staking_allocation, team_allocation)
    verify {
        assert_last_event::<T>(Event::TokenInfoChanged(caller, CERES_ASSET_ID.into()).into());
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{ExtBuilder, Runtime};
    use frame_support::assert_ok;

    #[test]
    #[ignore]
    fn test_benchmarks() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_get_rewards::<Runtime>());
            assert_ok!(test_benchmark_withdraw::<Runtime>());
            assert_ok!(test_benchmark_remove_pool::<Runtime>());
            assert_ok!(test_benchmark_change_pool_multiplier::<Runtime>());
            assert_ok!(test_benchmark_change_pool_deposit_fee::<Runtime>());
            assert_ok!(test_benchmark_change_token_info::<Runtime>());
        });
    }
}
