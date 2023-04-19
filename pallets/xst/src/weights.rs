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

//! Autogenerated weights for xst
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `TRX40`, CPU: `AMD Ryzen Threadripper 3960X 24-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/framenode
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --chain=local
// --steps=50
// --repeat=20
// --extrinsic=*
// --header
// ./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=pallets/xst/src/weights.rs
// --pallet
// xst

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for xst.
pub trait WeightInfo {
	fn initialize_pool() -> Weight;
	fn set_reference_asset() -> Weight;
	fn enable_synthetic_asset() -> Weight;
	fn set_synthetic_base_asset_floor_price() -> Weight;
	fn quote() -> Weight;
	fn exchange() -> Weight;
}

/// Weights for xst using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	fn initialize_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1321`
		//  Estimated: `13204`
		// Minimum execution time: 95_365_000 picoseconds.
		Weight::from_parts(97_305_000, 13204)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: XSTPool ReferenceAssetId (r:0 w:1)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	fn set_reference_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 16_411_000 picoseconds.
		Weight::from_parts(17_061_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_synthetic_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `234`
		//  Estimated: `729`
		// Minimum execution time: 13_350_000 picoseconds.
		Weight::from_parts(13_590_000, 729)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:0 w:1)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	fn set_synthetic_base_asset_floor_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_360_000 picoseconds.
		Weight::from_parts(8_530_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:1 w:0)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool BaseFee (r:1 w:0)
	/// Proof Skipped: XSTPool BaseFee (max_values: Some(1), max_size: None, mode: Measured)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2660`
		//  Estimated: `20230`
		// Minimum execution time: 25_691_000 picoseconds.
		Weight::from_parts(25_981_000, 20230)
			.saturating_add(T::DbWeight::get().reads(6_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool PermissionedTechAccount (r:1 w:0)
	/// Proof Skipped: XSTPool PermissionedTechAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:1 w:0)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool BaseFee (r:1 w:0)
	/// Proof Skipped: XSTPool BaseFee (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:3 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4274`
		//  Estimated: `64404`
		// Minimum execution time: 98_294_000 picoseconds.
		Weight::from_parts(98_864_000, 64404)
			.saturating_add(T::DbWeight::get().reads(16_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	fn initialize_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1321`
		//  Estimated: `13204`
		// Minimum execution time: 95_365_000 picoseconds.
		Weight::from_parts(97_305_000, 13204)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: XSTPool ReferenceAssetId (r:0 w:1)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	fn set_reference_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 16_411_000 picoseconds.
		Weight::from_parts(17_061_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_synthetic_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `234`
		//  Estimated: `729`
		// Minimum execution time: 13_350_000 picoseconds.
		Weight::from_parts(13_590_000, 729)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:0 w:1)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	fn set_synthetic_base_asset_floor_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_360_000 picoseconds.
		Weight::from_parts(8_530_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:1 w:0)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool BaseFee (r:1 w:0)
	/// Proof Skipped: XSTPool BaseFee (max_values: Some(1), max_size: None, mode: Measured)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2660`
		//  Estimated: `20230`
		// Minimum execution time: 25_691_000 picoseconds.
		Weight::from_parts(25_981_000, 20230)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool PermissionedTechAccount (r:1 w:0)
	/// Proof Skipped: XSTPool PermissionedTechAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:1 w:0)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XSTPool BaseFee (r:1 w:0)
	/// Proof Skipped: XSTPool BaseFee (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:3 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4274`
		//  Estimated: `64404`
		// Minimum execution time: 98_294_000 picoseconds.
		Weight::from_parts(98_864_000, 64404)
			.saturating_add(RocksDbWeight::get().reads(16_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}
