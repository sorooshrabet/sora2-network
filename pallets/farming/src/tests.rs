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

use crate::mock::*;
use crate::{Farmers, VestedRewards};
use common::prelude::Balance;
use common::{balance, AssetName, AssetSymbol, DOT, XOR};
use frame_support::assert_ok;
use frame_support::traits::{OnFinalize, OnInitialize};

fn run_to_block(n: u64) {
    while System::block_number() < n {
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        Farming::on_initialize(System::block_number());
    }
}

// Checks that accounts that have more than 1 XOR are automatically added to farming each REFRESH_FREQUENCY blocks. Also, checks that accounts that no longer has 1 XOR are removed from farming.
// Checks that farming
#[test]
fn test() {
    let dex_id = DEX_A_ID;
    let gt: crate::mock::AssetId = XOR;
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(assets::Module::<Runtime>::register_asset_id(
            ALICE(),
            XOR,
            AssetSymbol(b"XOR".to_vec()),
            AssetName(b"SORA".to_vec()),
            18,
            Balance::from(0u32),
            true,
        ));

        assert_ok!(assets::Module::<Runtime>::register_asset_id(
            ALICE(),
            DOT,
            AssetSymbol(b"DOT".to_vec()),
            AssetName(b"Polkadot".to_vec()),
            18,
            Balance::from(0u32),
            true,
        ));

        assert_ok!(assets::Module::<Runtime>::mint_to(
            &gt,
            &ALICE(),
            &ALICE(),
            balance!(2900000)
        ));

        assert_ok!(assets::Module::<Runtime>::mint_to(
            &gt,
            &ALICE(),
            &BOB(),
            balance!(2900000)
        ));

        assert_ok!(trading_pair::Module::<Runtime>::register(
            Origin::signed(BOB()),
            dex_id.clone(),
            XOR,
            DOT
        ));

        assert_ok!(pool_xyk::Module::<Runtime>::initialize_pool(
            Origin::signed(BOB()),
            dex_id.clone(),
            XOR,
            DOT,
        ));

        assert_ok!(pool_xyk::Module::<Runtime>::deposit_liquidity(
            Origin::signed(ALICE()),
            dex_id,
            XOR,
            DOT,
            balance!(1.1),
            balance!(4.4),
            balance!(1.1),
            balance!(4.4),
        ));

        assert_ok!(pool_xyk::Module::<Runtime>::deposit_liquidity(
            Origin::signed(BOB()),
            dex_id,
            XOR,
            DOT,
            balance!(1.1),
            balance!(4.4),
            balance!(1.1),
            balance!(4.4),
        ));

        run_to_block(REFRESH_FREQUENCY);

        assert_eq!(
            Farmers::<Runtime>::get(&ALICE()),
            Some((1099999999999999498, 200))
        );
        assert_eq!(
            Farmers::<Runtime>::get(&BOB()),
            Some((1099999999999999998, 200))
        );

        run_to_block(VESTING_FREQUENCY);

        assert_eq!(
            VestedRewards::<Runtime>::get(&ALICE()),
            34626038781163425878113
        );
        assert_eq!(
            VestedRewards::<Runtime>::get(&BOB()),
            34626038781163441621885
        );

        assert_ok!(pool_xyk::Module::<Runtime>::deposit_liquidity(
            Origin::signed(ALICE()),
            dex_id,
            XOR,
            DOT,
            balance!(0.5),
            balance!(2),
            balance!(0.3),
            balance!(0.5),
        ));

        assert_ok!(pool_xyk::Module::<Runtime>::withdraw_liquidity(
            Origin::signed(BOB()),
            dex_id,
            XOR,
            DOT,
            balance!(1.5),
            balance!(0.5),
            balance!(2),
        ));

        run_to_block(VESTING_FREQUENCY + REFRESH_FREQUENCY);

        assert_eq!(
            Farmers::<Runtime>::get(&ALICE()),
            Some((1599999999999999498, 200))
        );
        assert_eq!(Farmers::<Runtime>::get(&BOB()), None);

        run_to_block(VESTING_FREQUENCY + VESTING_FREQUENCY);

        // ALICE received all PSWAP
        assert_eq!(
            VestedRewards::<Runtime>::get(&ALICE()),
            103878116343490293378112
        );
        // BOB's rewards didn't change
        assert_eq!(
            VestedRewards::<Runtime>::get(&BOB()),
            34626038781163441621885
        );
    });
}
