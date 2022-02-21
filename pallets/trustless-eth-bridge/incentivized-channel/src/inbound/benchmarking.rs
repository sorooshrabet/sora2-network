//! IncentivizedInboundChannel pallet benchmarking

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{self, EventRecord, RawOrigin};
use hex_literal::hex;
use sp_std::convert::TryInto;
use sp_std::prelude::*;

use bridge_types::types::{ChannelId, Message, MessageId, Proof};
use bridge_types::{Header, Log};

const BASE_NETWORK_ID: EthNetworkId = 12123;

#[allow(unused_imports)]
use crate::inbound::Pallet as IncentivizedInboundChannel;

fn assert_last_event<T: Config>(system_event: <T as frame_system::Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

// This collection of benchmarks should include a benchmark for each
// call dispatched by the channel, i.e. each "app" pallet function
// that can be invoked by MessageDispatch. The most expensive call
// should be used in the `submit` benchmark.
//
// We rely on configuration via chain spec of the app pallets because
// we don't have access to their storage here.
benchmarks! {
    // Benchmark `submit` extrinsic under worst case conditions:
    // * `submit` dispatches the DotApp::unlock call
    // * `unlock` call successfully unlocks DOT
    submit {
        let caller: T::AccountId = whitelisted_caller();
        let (header, message) = dot_unlock_data();
        let envelope: envelope::Envelope<T> = rlp::decode::<Log>(&message.data)
            .map(|log| log.try_into().unwrap())
            .unwrap();
        <ChannelNonces<T>>::insert(BASE_NETWORK_ID, envelope.nonce - 1);
        <ChannelAddresses<T>>::insert(BASE_NETWORK_ID, envelope.channel);

        T::Verifier::initialize_storage(
            BASE_NETWORK_ID,
            vec![header],
            0,
            0, // forces all headers to be finalized
        )?;

    }: _(RawOrigin::Signed(caller.clone()), BASE_NETWORK_ID,message)
    verify {
        assert_eq!(envelope.nonce, <ChannelNonces<T>>::get(BASE_NETWORK_ID));

        let message_id = MessageId::new(ChannelId::Incentivized, envelope.nonce);
        if let Some(event) = T::MessageDispatch::successful_dispatch_event(message_id) {
            assert_last_event::<T>(event);
        }
    }

    // Benchmark `set_reward_fraction` under worst case conditions:
    // * The origin is authorized, i.e. equals UpdateOrigin
    set_reward_fraction {
        // Pick a value that is different from the initial RewardFraction
        let fraction = Perbill::from_percent(50);
        assert!(<RewardFraction<T>>::get() != fraction);

    }: _(RawOrigin::Root, fraction)
    verify {
        assert_eq!(<RewardFraction<T>>::get(), fraction);
    }

    #[extra]
    submit_eth_mint {
        let caller: T::AccountId = whitelisted_caller();
        let (header, message) = eth_mint_data();
        let envelope: envelope::Envelope<T> = rlp::decode::<Log>(&message.data)
            .map(|log| log.try_into().unwrap())
            .unwrap();
        <ChannelNonces<T>>::insert(BASE_NETWORK_ID, envelope.nonce - 1);
        <ChannelAddresses<T>>::insert(BASE_NETWORK_ID, envelope.channel);

        T::Verifier::initialize_storage(
            BASE_NETWORK_ID,
            vec![header],
            0,
            0, // forces all headers to be finalized
        )?;

    }: submit(RawOrigin::Signed(caller.clone()), BASE_NETWORK_ID, message)
    verify {
        assert_eq!(envelope.nonce, <ChannelNonces<T>>::get(BASE_NETWORK_ID));

        let message_id = MessageId::new(ChannelId::Incentivized, envelope.nonce);
        if let Some(event) = T::MessageDispatch::successful_dispatch_event(message_id) {
            assert_last_event::<T>(event);
        }
    }

    #[extra]
    submit_erc20_mint {
        let caller: T::AccountId = whitelisted_caller();
        let (header, message) = erc20_mint_data();
        let envelope: envelope::Envelope<T> = rlp::decode::<Log>(&message.data)
            .map(|log| log.try_into().unwrap())
            .unwrap();
        <ChannelNonces<T>>::insert(BASE_NETWORK_ID, envelope.nonce - 1);
        <ChannelAddresses<T>>::insert(BASE_NETWORK_ID, envelope.channel);

        T::Verifier::initialize_storage(
            BASE_NETWORK_ID,
            vec![header],
            0,
            0, // forces all headers to be finalized
        )?;

    }: submit(RawOrigin::Signed(caller.clone()), BASE_NETWORK_ID, message)
    verify {
        assert_eq!(envelope.nonce, <ChannelNonces<T>>::get(BASE_NETWORK_ID));

        let message_id = MessageId::new(ChannelId::Incentivized, envelope.nonce);
        if let Some(event) = T::MessageDispatch::successful_dispatch_event(message_id) {
            assert_last_event::<T>(event);
        }
    }

    register_channel {

    }: _(RawOrigin::Root, BASE_NETWORK_ID + 1, H160::repeat_byte(123))
    verify {
        assert_eq!(ChannelAddresses::<T>::get(BASE_NETWORK_ID + 1), Some(H160::repeat_byte(123)));
    }
}

// ETH mint
// Channel = 0xeda338e4dc46038493b885327842fd3e301cab39
// Fee = 10000000000
// Nonce = 3
// Source = 0x774667629726ec1fabebcec0d9139bd1c8f72a23
fn eth_mint_data() -> (Header, Message) {
    (
		Header {
			parent_hash: hex!("616872244fddc35315f7c6e0ad67171d74e200a70ee05f1e1e633f49ab0e2d08").into(),
			timestamp: 1619682198u64.into(),
			number: 74u64.into(),
			author: hex!("0000000000000000000000000000000000000000").into(),
			transactions_root: hex!("dd827279c5f79d0a55b1b014c37277e87e8e329698d71634ed5a9ecb0bad5d69").into(),
			ommers_hash: hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347").into(),
			extra_data: hex!("").into(),
			state_root: hex!("bd212b2a344f2a77b2c3097db26687b986939af84a832cb1cf9617a48c5cbed4").into(),
			receipts_root: hex!("c22e2e2d74ddd01bbb8298ccbf1c42acc21adafc5fd9f49dfbf8719e75eb9092").into(),
			logs_bloom: (&hex!("00000000000040000000000000000200000000000000400010000000010080000000000000000000000000001000000010000000000000000000000000000080000000000000000400000008000000000000000000008000000000000000000000000000020000000000000000000800000001000400000000000010000000000000020000000000000000000400000000000000000000000000040000000000000000000004000000000000000000000220000000000001000000000200080000000002000000000000000000000000000000000000000000000000000020402000000000000000000000000000000000000000000000000000000000000000")).into(),
			gas_used: 97210u64.into(),
			gas_limit: 6721975u64.into(),
			difficulty: 0u64.into(),
			seal: vec![
				hex!("a00000000000000000000000000000000000000000000000000000000000000000").to_vec(),
				hex!("880000000000000000").to_vec(),
			],
			base_fee: None,
		},
		Message {
			data: hex!("f9013a94eda338e4dc46038493b885327842fd3e301cab39e1a05e9ae1d7c484f74d554a503aa825e823725531d97e784dd9b1aacdb58d1f7076b90100000000000000000000000000774667629726ec1fabebcec0d9139bd1c8f72a2300000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000057410189b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0000c16ff2862300000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
			proof: Proof {
				block_hash: hex!("0ef7dda5ed1a551590a07bbd51c67c2f8b605e81bc5ae8e7b593603dd1706084").into(),
				tx_index: 0,
				data:
					vec![hex!("f904a8822080b904a2f9049f0183017bbab9010000000000000040000000000000000200000000000000400010000000010080000000000000000000000000001000000010000000000000000000000000000080000000000000000400000008000000000000000000008000000000000000000000000000020000000000000000000800000001000400000000000010000000000000020000000000000000000400000000000000000000000000040000000000000000000004000000000000000000000220000000000001000000000200080000000002000000000000000000000000000000000000000000000000000020402000000000000000000000000000000000000000000000000000000000000000f90394f89994774667629726ec1fabebcec0d9139bd1c8f72a23e1a0caae0f5e72020d428da73a237d1f9bf162e158dda6d4908769b8b60c095b01f4b86000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d000000000000000000000000000000000000000000000000002386f26fc10000f9011c94672a95c8928c8450b594186cf7954ec269626a2df863a0a78a9be3a7b862d26933ad85fb11d80ef66b8f972d7cbba06621d583943a4098a0000000000000000000000000b1185ede04202fe62d38f5db72f71e38ff3e8305a000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2b8a00000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f89b94672a95c8928c8450b594186cf7954ec269626a2df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000de0b6b3a7640000f9013a94eda338e4dc46038493b885327842fd3e301cab39e1a05e9ae1d7c484f74d554a503aa825e823725531d97e784dd9b1aacdb58d1f7076b90100000000000000000000000000774667629726ec1fabebcec0d9139bd1c8f72a2300000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000057410189b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0000c16ff2862300000000000000000000000000000000000000000000000000000000000000000000").to_vec()],
			},
		},
	)
}

// ERC20 mint
// Channel = 0xeda338e4dc46038493b885327842fd3e301cab39
// Fee = 10000000000
// Nonce = 2
// Source = 0x83428c7db9815f482a39a1715684dcf755021997
fn erc20_mint_data() -> (Header, Message) {
    (
		Header {
			parent_hash: hex!("47736d1a371ac88cd54239733e7d597716572724645abf92bbfb640b34efe008").into(),
			timestamp: 1619734472u64.into(),
			number: 59u64.into(),
			author: hex!("0000000000000000000000000000000000000000").into(),
			transactions_root: hex!("63b50c6ecaa5f26f112ad46cb8f159886a7d408b2ca8e6aa3d497f919cf8bbc3").into(),
			ommers_hash: hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347").into(),
			extra_data: hex!("").into(),
			state_root: hex!("4b9f175513e4f66ff469da75a69ded8bddca372388ebd92afd86bce77b2f8fb3").into(),
			receipts_root: hex!("3b6abad9d222b230dd114b9b1d3355120876488ffdc2febecafb8f564273e238").into(),
			logs_bloom: (&hex!("00000000000040000000000000000200000000000000000010000000000080000000000000000000000000001000000010000000000000000000000000200000000000000000000400000008002000000000000000008000000000000000000000000000020000000000000000000800200011000400000000000010001000000000020008000000000000000400000000000000000000000000840000000000020000000004000001000000000000000200000000000001000000000200000000000002000000000000000000000000000000000008000000800000200420400010000000000000000000000000000020000000000000000000000000000000")).into(),
			gas_used: 124834u64.into(),
			gas_limit: 6721975u64.into(),
			difficulty: 0u64.into(),
			seal: vec![
				hex!("a00000000000000000000000000000000000000000000000000000000000000000").to_vec(),
				hex!("880000000000000000").to_vec(),
			],
			base_fee: None,
		},
		Message {
			data: hex!("f9015a94eda338e4dc46038493b885327842fd3e301cab39e1a05e9ae1d7c484f74d554a503aa825e823725531d97e784dd9b1aacdb58d1f7076b9012000000000000000000000000083428c7db9815f482a39a1715684dcf75502199700000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000006b4201f8f7758fbcefd546eaeff7de24aff666b6228e7389b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27de803000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
			proof: Proof {
				block_hash: hex!("175410f9f31245f5cafe7f450df0e0240e084b2284611e51981da9ceaa38e890").into(),
				tx_index: 0,
				data:
					vec![hex!("f90622822080b9061cf90619018301e7a2b9010000000000000040000000000000000200000000000000000010000000000080000000000000000000000000001000000010000000000000000000000000200000000000000000000400000008002000000000000000008000000000000000000000000000020000000000000000000800200011000400000000000010001000000000020008000000000000000400000000000000000000000000840000000000020000000004000001000000000000000200000000000001000000000200000000000002000000000000000000000000000000000008000000800000200420400010000000000000000000000000000020000000000000000000000000000000f9050ef89b94f8f7758fbcefd546eaeff7de24aff666b6228e73f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2a000000000000000000000000083428c7db9815f482a39a1715684dcf755021997a000000000000000000000000000000000000000000000000000000000000003e8f89b94f8f7758fbcefd546eaeff7de24aff666b6228e73f863a08c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925a000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2a000000000000000000000000083428c7db9815f482a39a1715684dcf755021997a00000000000000000000000000000000000000000000000000000000000000000f8b99483428c7db9815f482a39a1715684dcf755021997e1a01e7b27577112ed83d53de87b38aee59ab80d8a9ba4acd90aad6cfee917534c79b880000000000000000000000000f8f7758fbcefd546eaeff7de24aff666b6228e7300000000000000000000000089b4ab1ef20763630df9743acf155865600daff2d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d00000000000000000000000000000000000000000000000000000000000003e8f9011c94672a95c8928c8450b594186cf7954ec269626a2df863a0a78a9be3a7b862d26933ad85fb11d80ef66b8f972d7cbba06621d583943a4098a0000000000000000000000000b1185ede04202fe62d38f5db72f71e38ff3e8305a000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2b8a00000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f89b94672a95c8928c8450b594186cf7954ec269626a2df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000de0b6b3a7640000f9015a94eda338e4dc46038493b885327842fd3e301cab39e1a05e9ae1d7c484f74d554a503aa825e823725531d97e784dd9b1aacdb58d1f7076b9012000000000000000000000000083428c7db9815f482a39a1715684dcf75502199700000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000006b4201f8f7758fbcefd546eaeff7de24aff666b6228e7389b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27de803000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").to_vec()],
			},
		}
	)
}

// DOT unlock
// Channel = 0xeda338e4dc46038493b885327842fd3e301cab39
// Fee = 10000000000
// Nonce = 1
// Source = 0xb1185ede04202fe62d38f5db72f71e38ff3e8305
fn dot_unlock_data() -> (Header, Message) {
    (
		Header {
			parent_hash: hex!("1eb95c998f6af053b87e8955d7cecada585b0304dfb5d7283947a5674e6e42f1").into(),
			timestamp: 1619678921u64.into(),
			number: 72u64.into(),
			author: hex!("0000000000000000000000000000000000000000").into(),
			transactions_root: hex!("8f407c85c74ab696364e8ff77bbdcffeb52a118c1a50dd43d848c6e37407d9e5").into(),
			ommers_hash: hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347").into(),
			extra_data: hex!("").into(),
			state_root: hex!("2a785e43bdd23e81499a062c1645879e573484a329114c4dacb1762471793116").into(),
			receipts_root: hex!("a37efc2c11999db875b5080c244999f8625c165473aa7f9a374b0dfc31f9da85").into(),
			logs_bloom: (&hex!("00000000000040000000000000000200000000000000000010000000000080000000000000000000000000001000000010000000000000000000000000000000000000000000000400000008000000000000000000008000000000000000000000000000020000000000000000000800000001000400000000000010000000000000020000000000000000000400000000000000000000000000040000000000000000000004000000000000000000000200000000000001000000000200000000000002000000000000000000000000000000000000000000000000000020400000000000000000000000000000000000000000000000000000000000000000")).into(),
			gas_used: 107966u64.into(),
			gas_limit: 6721975u64.into(),
			difficulty: 0u64.into(),
			seal: vec![
				hex!("a00000000000000000000000000000000000000000000000000000000000000000").to_vec(),
				hex!("880000000000000000").to_vec(),
			],
			base_fee: None,
		},
		Message {
			data: hex!("f9013a94eda338e4dc46038493b885327842fd3e301cab39e1a05e9ae1d7c484f74d554a503aa825e823725531d97e784dd9b1aacdb58d1f7076b90100000000000000000000000000b1185ede04202fe62d38f5db72f71e38ff3e830500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000057400189b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d000064a7b3b6e00d000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
			proof: Proof {
				block_hash: hex!("ba80aac46bc6d7f7fdc3eba2a0643e0b2baad96ca69e2709e159d92143e343b6").into(),
				tx_index: 0,
				data:
					vec![hex!("f905e9822080b905e3f905e0018301a5beb9010000000000000040000000000000000200000000000000000010000000000080000000000000000000000000001000000010000000000000000000000000000000000000000000000400000008000000000000000000008000000000000000000000000000020000000000000000000800000001000400000000000010000000000000020000000000000000000400000000000000000000000000040000000000000000000004000000000000000000000200000000000001000000000200000000000002000000000000000000000000000000000000000000000000000020400000000000000000000000000000000000000000000000000000000000000000f904d5f9013c94672a95c8928c8450b594186cf7954ec269626a2df863a0a78a9be3a7b862d26933ad85fb11d80ef66b8f972d7cbba06621d583943a4098a0000000000000000000000000b1185ede04202fe62d38f5db72f71e38ff3e8305a000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2b8c00000000000000000000000000000000000000000000000000de0b6b3a7640000000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000020d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0000000000000000000000000000000000000000000000000000000000000000f89b94672a95c8928c8450b594186cf7954ec269626a2df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000de0b6b3a7640000f9011c94672a95c8928c8450b594186cf7954ec269626a2df863a0a78a9be3a7b862d26933ad85fb11d80ef66b8f972d7cbba06621d583943a4098a0000000000000000000000000b1185ede04202fe62d38f5db72f71e38ff3e8305a000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2b8a00000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f89b94672a95c8928c8450b594186cf7954ec269626a2df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000de0b6b3a7640000f9013a94eda338e4dc46038493b885327842fd3e301cab39e1a05e9ae1d7c484f74d554a503aa825e823725531d97e784dd9b1aacdb58d1f7076b90100000000000000000000000000b1185ede04202fe62d38f5db72f71e38ff3e830500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000057400189b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d000064a7b3b6e00d000000000000000000000000000000000000000000000000000000000000000000").to_vec()],
			},
		},
	)
}

impl_benchmark_test_suite!(
    IncentivizedInboundChannel,
    crate::inbound::test::new_tester(Default::default()),
    crate::inbound::test::Test,
);
