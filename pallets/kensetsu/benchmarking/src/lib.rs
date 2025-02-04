// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg(feature = "runtime-benchmarks")]

use assets::AssetIdOf;
use codec::Decode;
use common::{balance, AssetId32, Balance, DEXId, PredefinedAssetId, DAI, KUSD, XOR};
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;
use hex_literal::hex;
use kensetsu::CdpId;
use sp_arithmetic::{Perbill, Percent};
use sp_core::Get;
use sp_runtime::traits::One;
use sp_runtime::FixedU128;

pub struct Pallet<T: Config>(kensetsu::Pallet<T>);
pub trait Config:
    kensetsu::Config + pool_xyk::Config + trading_pair::Config + pallet_timestamp::Config
{
}

/// Client account id
fn caller<T: Config>() -> T::AccountId {
    let bytes = hex!("92c4ff71ae7492a1e6fef5d80546ea16307c560ac1063ffaa5e0e084df1e2b7e");
    T::AccountId::decode(&mut &bytes[..]).expect("Failed to decode account ID")
}

/// Risk manager account id
fn risk_manager<T: Config>() -> T::AccountId {
    let bytes = hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
    T::AccountId::decode(&mut &bytes[..]).expect("Failed to decode account ID")
}

/// Sets XOR as collateral type with default risk parameters
fn set_xor_as_collateral_type<T: Config>() {
    kensetsu::CollateralInfos::<T>::set::<AssetIdOf<T>>(
        XOR.into(),
        Some(kensetsu::CollateralInfo {
            risk_parameters: kensetsu::CollateralRiskParameters {
                hard_cap: Balance::MAX,
                liquidation_ratio: Perbill::from_percent(50),
                max_liquidation_lot: balance!(100),
                stability_fee_rate: FixedU128::from_perbill(Perbill::from_percent(10)),
            },
            kusd_supply: balance!(0),
            last_fee_update_time: Default::default(),
            interest_coefficient: FixedU128::one(),
        }),
    );
}

/// Creates CDP with XOR as collateral
fn create_cdp_with_xor<T: Config>() -> CdpId {
    kensetsu::Pallet::<T>::create_cdp(
        RawOrigin::Signed(caller::<T>()).into(),
        XOR.into(),
        balance!(0),
        balance!(0),
    )
    .expect("Shall create CDP");
    kensetsu::NextCDPId::<T>::get()
}

/// Mints XOR and deposited as collateral to CDP
fn deposit_xor_collateral<T: Config>(cdp_id: CdpId, amount: Balance) {
    assets::Pallet::<T>::update_balance(
        RawOrigin::Root.into(),
        caller::<T>(),
        XOR.into(),
        amount.try_into().unwrap(),
    )
    .expect("Shall mint XOR");
    kensetsu::Pallet::<T>::deposit_collateral(
        RawOrigin::Signed(caller::<T>()).into(),
        cdp_id,
        amount,
    )
    .expect("Shall deposit");
}

/// Sets liquidation ratio too low, making CDPs unsafe
fn make_cdps_unsafe<T: Config>() {
    kensetsu::CollateralInfos::<T>::set::<AssetIdOf<T>>(
        XOR.into(),
        Some(kensetsu::CollateralInfo {
            risk_parameters: kensetsu::CollateralRiskParameters {
                hard_cap: Balance::MAX,
                liquidation_ratio: Perbill::from_percent(10),
                max_liquidation_lot: balance!(100),
                stability_fee_rate: FixedU128::from_perbill(Perbill::from_percent(10)),
            },
            kusd_supply: balance!(0),
            last_fee_update_time: Default::default(),
            interest_coefficient: FixedU128::one(),
        }),
    );
}

/// Initializes and adds liquidity to XYK pool XOR/asset_id.
fn intialize_xyk_pool<T: Config>(asset_id: AssetIdOf<T>) {
    let amount = balance!(1000000);
    assets::Pallet::<T>::update_balance(
        RawOrigin::Root.into(),
        caller::<T>(),
        XOR.into(),
        amount.try_into().unwrap(),
    )
    .expect("Shall mint XOR");
    assets::Pallet::<T>::update_balance(
        RawOrigin::Root.into(),
        caller::<T>(),
        asset_id,
        amount.try_into().unwrap(),
    )
    .expect("Shall mint token");
    pool_xyk::Pallet::<T>::initialize_pool(
        RawOrigin::Signed(caller::<T>()).into(),
        DEXId::Polkaswap.into(),
        XOR.into(),
        asset_id,
    )
    .expect("Must init init pool");
    pool_xyk::Pallet::<T>::deposit_liquidity(
        RawOrigin::Signed(caller::<T>()).into(),
        DEXId::Polkaswap.into(),
        XOR.into(),
        asset_id,
        amount,
        amount,
        amount,
        amount,
    )
    .expect("Must deposit liquidity to pool");
}

/// Initializes pools with:
/// - XOR/DAI for collateral assessment
/// - XOR/KUSD for liquidation
fn initialize_liquidity_sources<T: Config>() {
    intialize_xyk_pool::<T>(DAI.into());
    trading_pair::Pallet::<T>::register(
        RawOrigin::Signed(caller::<T>()).into(),
        DEXId::Polkaswap.into(),
        XOR.into(),
        KUSD.into(),
    )
    .expect("Must register trading pair");
    intialize_xyk_pool::<T>(KUSD.into());
}

benchmarks! {
    where_clause {
        where T::AssetId: From<AssetId32<PredefinedAssetId>>
    }

    create_cdp {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), risk_manager::<T>())
            .expect("Must set risk manager");
        kensetsu::Pallet::<T>::update_hard_cap_total_supply(
            RawOrigin::Signed(risk_manager::<T>()).into(),
            Balance::MAX,
        ).expect("Shall update hard cap");
        initialize_liquidity_sources::<T>();
        set_xor_as_collateral_type::<T>();
        let collateral = balance!(10);
        let debt = balance!(1);
        assets::Pallet::<T>::update_balance(
            RawOrigin::Root.into(),
            caller::<T>(),
            XOR.into(),
            collateral.try_into().unwrap(),
        )
        .expect("Shall mint XOR");
    }: {
        kensetsu::Pallet::<T>::create_cdp(
            RawOrigin::Signed(caller::<T>()).into(),
            XOR.into(),
            collateral,
            debt
        ).unwrap();
    }

    close_cdp {
        set_xor_as_collateral_type::<T>();
        let cdp_id = create_cdp_with_xor::<T>();
    }: {
        kensetsu::Pallet::<T>::close_cdp(RawOrigin::Signed(caller::<T>()).into(), cdp_id).unwrap();
    }

    deposit_collateral {
        set_xor_as_collateral_type::<T>();
        let cdp_id = create_cdp_with_xor::<T>();
        let amount = balance!(10);
        assets::Pallet::<T>::update_balance(
            RawOrigin::Root.into(),
            caller::<T>(),
            XOR.into(),
            amount.try_into().unwrap()
        ).expect("Shall mint XOR");
    }: {
        kensetsu::Pallet::<T>::deposit_collateral(
            RawOrigin::Signed(caller::<T>()).into(),
            cdp_id,
            amount
        ).unwrap();
    }

    withdraw_collateral {
        initialize_liquidity_sources::<T>();
        set_xor_as_collateral_type::<T>();
        let cdp_id = create_cdp_with_xor::<T>();
        let amount = balance!(10);
        deposit_xor_collateral::<T>(cdp_id, amount);
    }: {
        kensetsu::Pallet::<T>::withdraw_collateral(
            RawOrigin::Signed(caller::<T>()).into(),
            cdp_id,
            amount
        ).unwrap();
    }

    borrow {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), risk_manager::<T>())
            .expect("Must set risk manager");
        initialize_liquidity_sources::<T>();
        set_xor_as_collateral_type::<T>();
        let cdp_id = create_cdp_with_xor::<T>();
        let amount = balance!(10);
        deposit_xor_collateral::<T>(cdp_id, amount);
        let debt = balance!(1);
        kensetsu::Pallet::<T>::update_hard_cap_total_supply(
            RawOrigin::Signed(risk_manager::<T>()).into(),
            Balance::MAX,
        ).expect("Shall update hard cap");
    }: {
        kensetsu::Pallet::<T>::borrow(
            RawOrigin::Signed(caller::<T>()).into(),
            cdp_id,
            debt
        ).unwrap();
    }

    repay_debt {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), risk_manager::<T>())
            .expect("Must set risk manager");
        initialize_liquidity_sources::<T>();
        set_xor_as_collateral_type::<T>();
        let cdp_id = create_cdp_with_xor::<T>();
        let amount = balance!(10);
        deposit_xor_collateral::<T>(cdp_id, amount);
        let debt = balance!(1);
        kensetsu::Pallet::<T>::update_hard_cap_total_supply(
            RawOrigin::Signed(risk_manager::<T>()).into(),
            Balance::MAX,
        ).expect("Shall update hard cap");
        kensetsu::Pallet::<T>::borrow(RawOrigin::Signed(caller::<T>()).into(), cdp_id, debt)
            .expect("Shall borrow");
    }: {
        kensetsu::Pallet::<T>::repay_debt(
            RawOrigin::Signed(caller::<T>()).into(),
            cdp_id,
            debt
        ).unwrap();
    }

    liquidate {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), risk_manager::<T>())
            .expect("Must set risk manager");
        initialize_liquidity_sources::<T>();
        set_xor_as_collateral_type::<T>();
        let cdp_id = create_cdp_with_xor::<T>();
        let amount = balance!(100);
        deposit_xor_collateral::<T>(cdp_id, amount);
        let debt = balance!(10);
        kensetsu::Pallet::<T>::update_hard_cap_total_supply(
            RawOrigin::Signed(risk_manager::<T>()).into(),
            Balance::MAX,
        ).expect("Shall update hard cap");
        kensetsu::Pallet::<T>::borrow(RawOrigin::Signed(caller::<T>()).into(), cdp_id, debt)
            .expect("Shall borrow");
        make_cdps_unsafe::<T>();
    }: {
        kensetsu::Pallet::<T>::liquidate(RawOrigin::Signed(caller::<T>()).into(), cdp_id).unwrap();
    }

    accrue {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), risk_manager::<T>())
            .expect("Must set risk manager");
        initialize_liquidity_sources::<T>();
        set_xor_as_collateral_type::<T>();
        let cdp_id = create_cdp_with_xor::<T>();
        let amount = balance!(10);
        deposit_xor_collateral::<T>(cdp_id, amount);
        let debt = balance!(1);
        kensetsu::Pallet::<T>::update_hard_cap_total_supply(
            RawOrigin::Signed(risk_manager::<T>()).into(),
            Balance::MAX,
        ).expect("Shall update hard cap");
        kensetsu::Pallet::<T>::borrow(
            RawOrigin::Signed(caller::<T>()).into(),
            cdp_id,
            debt,
        ).expect("Shall borrow");
    }: {
        kensetsu::Pallet::<T>::accrue(RawOrigin::Signed(caller::<T>()).into(), cdp_id).unwrap();
    }

    update_collateral_risk_parameters {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), risk_manager::<T>())
            .expect("Must set risk manager");
    }: {
        kensetsu::Pallet::<T>::update_collateral_risk_parameters(
            RawOrigin::Signed(risk_manager::<T>()).into(),
            XOR.into(),
            kensetsu::CollateralRiskParameters {
                hard_cap: balance!(1000),
                liquidation_ratio: Perbill::from_percent(50),
                max_liquidation_lot: balance!(100),
                stability_fee_rate: Default::default(),
            }
        ).unwrap();
    }

    update_hard_cap_total_supply {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), risk_manager::<T>())
            .expect("Must set risk manager");
    }: {
        kensetsu::Pallet::<T>::update_hard_cap_total_supply(
            RawOrigin::Signed(risk_manager::<T>()).into(),
            balance!(1000)
        ).unwrap();
    }

    update_liquidation_penalty {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), risk_manager::<T>())
            .expect("Must set risk manager");
    }:{
        kensetsu::Pallet::<T>::update_liquidation_penalty(
            RawOrigin::Signed(risk_manager::<T>()).into(),
            Percent::from_percent(10)
        ).unwrap();
    }

    withdraw_profit {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), risk_manager::<T>())
            .expect("Must set risk manager");
        let technical_account_id = technical::Pallet::<T>::tech_account_id_to_account_id(
            &T::TreasuryTechAccount::get(),
        ).expect("Shall resolve tech account id");
        let amount = balance!(10);
        assets::Pallet::<T>::update_balance(
            RawOrigin::Root.into(),
            technical_account_id,
            KUSD.into(),
            amount.try_into().unwrap(),
        )
        .expect("Shall mint KUSD");
    }:{
        kensetsu::Pallet::<T>::withdraw_profit(
            RawOrigin::Signed(risk_manager::<T>()).into(),
            amount
        ).unwrap();
    }

    donate {
        let amount = balance!(10);
        assets::Pallet::<T>::update_balance(
            RawOrigin::Root.into(),
            caller::<T>(),
            KUSD.into(),
            amount.try_into().unwrap(),
        )
        .expect("Shall mint KUSD");
        kensetsu::BadDebt::<T>::set(balance!(5));
    }: {
        kensetsu::Pallet::<T>::donate(RawOrigin::Signed(caller::<T>()).into(), amount).unwrap();
    }

    add_risk_manager {}: {
        kensetsu::Pallet::<T>::add_risk_manager(RawOrigin::Root.into(), caller::<T>()).unwrap();
    }

    remove_risk_manager {}: {
        kensetsu::Pallet::<T>::remove_risk_manager(RawOrigin::Root.into(), caller::<T>()).unwrap();
    }
}
