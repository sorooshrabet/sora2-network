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

//! Autogenerated weights for order_book
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `1d7f88616439`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=order_book
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/order-book/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for order_book.
pub trait WeightInfo {
	fn create_orderbook() -> Weight;
	fn delete_orderbook() -> Weight;
	fn update_orderbook() -> Weight;
	fn change_orderbook_status() -> Weight;
	fn place_limit_order_without_cross_spread() -> Weight;
	fn cancel_limit_order_first_expiration() -> Weight;
	fn cancel_limit_order_last_expiration() -> Weight;
	fn execute_market_order() -> Weight;
	fn quote() -> Weight;
	fn exchange(e: u32, ) -> Weight;
	fn align_single_order() -> Weight;
	fn service_expiration_base() -> Weight;
	fn service_expiration_block_base() -> Weight;
	fn service_single_expiration() -> Weight;
}

/// Weights for order_book using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3079`
		//  Estimated: `38228`
		// Minimum execution time: 107_814_000 picoseconds.
		Weight::from_parts(108_768_000, 38228)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:0)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn delete_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1791`
		//  Estimated: `20825`
		// Minimum execution time: 75_451_000 picoseconds.
		Weight::from_parts(76_456_000, 20825)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: OrderBook AlignmentCursor (r:0 w:1)
	/// Proof: OrderBook AlignmentCursor (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	fn update_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1463`
		//  Estimated: `9182`
		// Minimum execution time: 50_459_000 picoseconds.
		Weight::from_parts(51_334_000, 9182)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	fn change_orderbook_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `622`
		//  Estimated: `2713`
		// Minimum execution time: 24_288_000 picoseconds.
		Weight::from_parts(24_922_000, 2713)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook Asks (r:1 w:1)
	/// Proof: OrderBook Asks (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedAsks (r:1 w:1)
	/// Proof: OrderBook AggregatedAsks (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn place_limit_order_without_cross_spread() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `123712`
		//  Estimated: `467348`
		// Minimum execution time: 557_151_000 picoseconds.
		Weight::from_parts(586_411_000, 467348)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn cancel_limit_order_first_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `158523`
		//  Estimated: `335473`
		// Minimum execution time: 962_892_000 picoseconds.
		Weight::from_parts(1_001_552_000, 335473)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn cancel_limit_order_last_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `158523`
		//  Estimated: `335473`
		// Minimum execution time: 985_468_000 picoseconds.
		Weight::from_parts(1_015_037_000, 335473)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1024 w:1024)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:4000 w:4000)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:4 w:4)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:4 w:4)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn execute_market_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1348127`
		//  Estimated: `33469598`
		// Minimum execution time: 587_975_791_000 picoseconds.
		Weight::from_parts(590_066_962_000, 33469598)
			.saturating_add(T::DbWeight::get().reads(5044_u64))
			.saturating_add(T::DbWeight::get().writes(5041_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `35797`
		//  Estimated: `78362`
		// Minimum execution time: 845_878_000 picoseconds.
		Weight::from_parts(882_782_000, 78362)
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1024 w:1024)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:4000 w:4000)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:4 w:4)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:4 w:4)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	/// The range of component `e` is `[1, 4000]`.
	fn exchange(e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `49215 + e * (330 ±0)`
		//  Estimated: `10605094 + e * (6677 ±0)`
		// Minimum execution time: 191_521_000 picoseconds.
		Weight::from_parts(192_462_000, 10605094)
			// Standard Error: 229_751
			.saturating_add(Weight::from_parts(143_475_162, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(558_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes(554_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 6677).saturating_mul(e.into()))
	}
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	fn align_single_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37814`
		//  Estimated: `85583`
		// Minimum execution time: 419_123_000 picoseconds.
		Weight::from_parts(440_863_000, 85583)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook IncompleteExpirationsSince (r:1 w:0)
	/// Proof: OrderBook IncompleteExpirationsSince (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:0)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn service_expiration_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `88996`
		// Minimum execution time: 5_648_000 picoseconds.
		Weight::from_parts(5_772_000, 88996)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:0)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn service_expiration_block_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `88497`
		// Minimum execution time: 3_647_000 picoseconds.
		Weight::from_parts(4_360_000, 88497)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:0)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:0)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:0)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn service_single_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3985`
		//  Estimated: `180935`
		// Minimum execution time: 118_628_000 picoseconds.
		Weight::from_parts(119_815_000, 180935)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3079`
		//  Estimated: `38228`
		// Minimum execution time: 107_814_000 picoseconds.
		Weight::from_parts(108_768_000, 38228)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:0)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn delete_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1791`
		//  Estimated: `20825`
		// Minimum execution time: 75_451_000 picoseconds.
		Weight::from_parts(76_456_000, 20825)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: OrderBook AlignmentCursor (r:0 w:1)
	/// Proof: OrderBook AlignmentCursor (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	fn update_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1463`
		//  Estimated: `9182`
		// Minimum execution time: 50_459_000 picoseconds.
		Weight::from_parts(51_334_000, 9182)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	fn change_orderbook_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `622`
		//  Estimated: `2713`
		// Minimum execution time: 24_288_000 picoseconds.
		Weight::from_parts(24_922_000, 2713)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook Asks (r:1 w:1)
	/// Proof: OrderBook Asks (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedAsks (r:1 w:1)
	/// Proof: OrderBook AggregatedAsks (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn place_limit_order_without_cross_spread() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `123712`
		//  Estimated: `467348`
		// Minimum execution time: 557_151_000 picoseconds.
		Weight::from_parts(586_411_000, 467348)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn cancel_limit_order_first_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `158523`
		//  Estimated: `335473`
		// Minimum execution time: 962_892_000 picoseconds.
		Weight::from_parts(1_001_552_000, 335473)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn cancel_limit_order_last_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `158523`
		//  Estimated: `335473`
		// Minimum execution time: 985_468_000 picoseconds.
		Weight::from_parts(1_015_037_000, 335473)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1024 w:1024)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:4000 w:4000)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:4 w:4)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:4 w:4)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn execute_market_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1348127`
		//  Estimated: `33469598`
		// Minimum execution time: 587_975_791_000 picoseconds.
		Weight::from_parts(590_066_962_000, 33469598)
			.saturating_add(RocksDbWeight::get().reads(5044_u64))
			.saturating_add(RocksDbWeight::get().writes(5041_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `35797`
		//  Estimated: `78362`
		// Minimum execution time: 845_878_000 picoseconds.
		Weight::from_parts(882_782_000, 78362)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1024 w:1024)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:4000 w:4000)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:4 w:4)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:4 w:4)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	/// The range of component `e` is `[1, 4000]`.
	fn exchange(e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `49215 + e * (330 ±0)`
		//  Estimated: `10605094 + e * (6677 ±0)`
		// Minimum execution time: 191_521_000 picoseconds.
		Weight::from_parts(192_462_000, 10605094)
			// Standard Error: 229_751
			.saturating_add(Weight::from_parts(143_475_162, 0).saturating_mul(e.into()))
			.saturating_add(RocksDbWeight::get().reads(558_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(e.into())))
			.saturating_add(RocksDbWeight::get().writes(554_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 6677).saturating_mul(e.into()))
	}
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	fn align_single_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37814`
		//  Estimated: `85583`
		// Minimum execution time: 419_123_000 picoseconds.
		Weight::from_parts(440_863_000, 85583)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook IncompleteExpirationsSince (r:1 w:0)
	/// Proof: OrderBook IncompleteExpirationsSince (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:0)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn service_expiration_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `88996`
		// Minimum execution time: 5_648_000 picoseconds.
		Weight::from_parts(5_772_000, 88996)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:0)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn service_expiration_block_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `88497`
		// Minimum execution time: 3_647_000 picoseconds.
		Weight::from_parts(4_360_000, 88497)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:0)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:0)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:0)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(86022), added: 88497, mode: MaxEncodedLen)
	fn service_single_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3985`
		//  Estimated: `180935`
		// Minimum execution time: 118_628_000 picoseconds.
		Weight::from_parts(119_815_000, 180935)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
