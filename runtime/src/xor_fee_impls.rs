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

use crate::*;
use common::LiquidityProxyTrait;
use pallet_utility::Call as UtilityCall;
use sp_runtime::traits::Zero;

impl RuntimeCall {
    pub fn swap_count(&self) -> u32 {
        match self {
            Self::Multisig(pallet_multisig::Call::as_multi_threshold_1 { call, .. })
            | Self::Multisig(pallet_multisig::Call::as_multi { call, .. })
            | Self::Utility(UtilityCall::as_derivative { call, .. }) => call.swap_count(),
            Self::Utility(UtilityCall::batch { calls })
            | Self::Utility(UtilityCall::batch_all { calls })
            | Self::Utility(UtilityCall::force_batch { calls }) => {
                calls.iter().map(|call| call.swap_count()).sum()
            }
            Self::LiquidityProxy(liquidity_proxy::Call::swap { .. })
            | Self::LiquidityProxy(liquidity_proxy::Call::swap_transfer { .. })
            | Self::LiquidityProxy(liquidity_proxy::Call::swap_transfer_batch { .. }) => 1,
            _ => 0,
        }
    }

    pub fn is_called_by_bridge_peer(&self, who: &AccountId) -> bool {
        match self {
            RuntimeCall::BridgeMultisig(call) => match call {
                bridge_multisig::Call::as_multi {
                    id: multisig_id, ..
                }
                | bridge_multisig::Call::as_multi_threshold_1 {
                    id: multisig_id, ..
                } => bridge_multisig::Accounts::<Runtime>::get(multisig_id)
                    .map(|acc| acc.is_signatory(&who)),
                _ => None,
            },
            RuntimeCall::EthBridge(call) => match call {
                eth_bridge::Call::approve_request { network_id, .. } => {
                    Some(eth_bridge::Pallet::<Runtime>::is_peer(who, *network_id))
                }
                eth_bridge::Call::register_incoming_request { incoming_request } => {
                    let net_id = incoming_request.network_id();
                    eth_bridge::BridgeAccount::<Runtime>::get(net_id).map(|acc| acc == *who)
                }
                eth_bridge::Call::import_incoming_request {
                    load_incoming_request,
                    ..
                } => {
                    let net_id = load_incoming_request.network_id();
                    eth_bridge::BridgeAccount::<Runtime>::get(net_id).map(|acc| acc == *who)
                }
                eth_bridge::Call::finalize_incoming_request { network_id, .. }
                | eth_bridge::Call::abort_request { network_id, .. } => {
                    eth_bridge::BridgeAccount::<Runtime>::get(network_id).map(|acc| acc == *who)
                }
                _ => None,
            },
            _ => None,
        }
        .unwrap_or(false)
    }
}

pub struct CustomFees;

impl CustomFees {
    fn base_fee(call: &RuntimeCall) -> Option<Balance> {
        match call {
            RuntimeCall::LiquidityProxy(liquidity_proxy::Call::swap_transfer_batch {
                swap_batches,
                ..
            }) => Some(
                swap_batches
                    .iter()
                    .map(|x| x.receivers.len() as Balance)
                    .fold(Balance::zero(), |acc, x| acc.saturating_add(x))
                    .saturating_mul(SMALL_FEE)
                    .max(SMALL_FEE),
            ),
            RuntimeCall::Assets(assets::Call::register { .. })
            | RuntimeCall::EthBridge(eth_bridge::Call::transfer_to_sidechain { .. })
            | RuntimeCall::BridgeProxy(bridge_proxy::Call::burn { .. })
            | RuntimeCall::PoolXYK(pool_xyk::Call::withdraw_liquidity { .. })
            | RuntimeCall::Rewards(rewards::Call::claim { .. })
            | RuntimeCall::VestedRewards(vested_rewards::Call::claim_crowdloan_rewards {
                ..
            })
            | RuntimeCall::VestedRewards(vested_rewards::Call::claim_rewards { .. })
            | RuntimeCall::OrderBook(order_book::Call::update_orderbook { .. }) => Some(BIG_FEE),
            RuntimeCall::Assets(..)
            | RuntimeCall::EthBridge(..)
            | RuntimeCall::LiquidityProxy(..)
            | RuntimeCall::MulticollateralBondingCurvePool(..)
            | RuntimeCall::PoolXYK(..)
            | RuntimeCall::Rewards(..)
            | RuntimeCall::Staking(pallet_staking::Call::payout_stakers { .. })
            | RuntimeCall::TradingPair(..)
            | RuntimeCall::Band(..)
            | RuntimeCall::Referrals(..)
            | RuntimeCall::OrderBook(..) => Some(SMALL_FEE),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustomFeeDetails {
    /// Regular call with custom fee without any additional logic
    Regular(Balance),

    /// OrderBook::place_limit_order custom fee depends on limit order lifetime
    LimitOrderLifetime(Option<Moment>),
}

// Flat fees implementation for the selected extrinsics.
// Returns a value if the extrinsic is subject to manual fee
// adjustment and `None` otherwise
impl xor_fee::ApplyCustomFees<RuntimeCall, AccountId> for CustomFees {
    type FeeDetails = CustomFeeDetails;

    fn compute_fee(call: &RuntimeCall) -> Option<(Balance, CustomFeeDetails)> {
        let fee = Self::base_fee(call)?;

        let details = match call {
            RuntimeCall::OrderBook(order_book::Call::place_limit_order { lifespan, .. }) => {
                CustomFeeDetails::LimitOrderLifetime(*lifespan)
            }
            _ => CustomFeeDetails::Regular(fee),
        };

        Some((fee, details))
    }

    fn should_be_postponed(
        who: &AccountId,
        _fee_source: &AccountId,
        call: &RuntimeCall,
        fee: Balance,
    ) -> bool {
        let balance = Balances::usable_balance_for_fees(who);
        match call {
            // In case we are producing XOR, we perform exchange before fees are withdraw to allow 0-XOR accounts to trade
            RuntimeCall::LiquidityProxy(liquidity_proxy::Call::swap {
                dex_id,
                input_asset_id,
                output_asset_id,
                swap_amount,
                selected_source_types,
                filter_mode,
            }) => {
                if *output_asset_id != XOR {
                    return false;
                }
                // Check how much user has input asset
                let user_input_balance = Currencies::free_balance(*input_asset_id, who);

                // How much does the user want to spend of their input asset
                let swap_input_amount = match swap_amount {
                    SwapAmount::WithDesiredInput {
                        desired_amount_in, ..
                    } => desired_amount_in,
                    SwapAmount::WithDesiredOutput { max_amount_in, .. } => max_amount_in,
                };

                // The amount of input asset needed for this swap is more than the user has, so error
                if *swap_input_amount > user_input_balance {
                    return false;
                }

                let filter = LiquiditySourceFilter::with_mode(
                    *dex_id,
                    filter_mode.clone(),
                    selected_source_types.clone(),
                );
                let Ok(swap_result) = LiquidityProxy::quote(
                        *dex_id,
                        input_asset_id,
                        output_asset_id,
                        (*swap_amount).into(),
                        filter,
                        true,
                    ) else {
                        return false;
                    };

                let (limits_ok, output_amount) = match swap_amount {
                    SwapAmount::WithDesiredInput { min_amount_out, .. } => {
                        (swap_result.amount >= *min_amount_out, swap_result.amount)
                    }
                    SwapAmount::WithDesiredOutput {
                        desired_amount_out,
                        max_amount_in,
                        ..
                    } => (swap_result.amount <= *max_amount_in, *desired_amount_out),
                };

                if limits_ok
                    && balance
                        .saturating_add(output_amount)
                        .saturating_sub(Balances::minimum_balance())
                        >= fee
                {
                    return true;
                } else {
                    return false;
                }
            }
            RuntimeCall::LiquidityProxy(liquidity_proxy::Call::xorless_transfer {
                dex_id,
                asset_id,
                amount,
                selected_source_types,
                filter_mode,
                desired_xor_amount,
                max_amount_in,
                ..
            }) => {
                // Pay fee as usual
                if balance > fee {
                    return false;
                }

                // Check how much user has input asset
                let user_input_balance = Currencies::free_balance(*asset_id, who);

                // The amount of input asset needed for this swap is more than the user has, so error
                if amount.saturating_add(*max_amount_in) > user_input_balance {
                    return false;
                }

                let filter = LiquiditySourceFilter::with_mode(
                    *dex_id,
                    filter_mode.clone(),
                    selected_source_types.clone(),
                );
                let Ok(swap_result) = LiquidityProxy::quote(
                        *dex_id,
                        asset_id,
                        &XOR,
                        QuoteAmount::with_desired_output(*desired_xor_amount),
                        filter,
                        true,
                    ) else {
                        return false;
                    };
                if swap_result.amount <= *max_amount_in
                    && balance
                        .saturating_add(*desired_xor_amount)
                        .saturating_sub(Balances::minimum_balance())
                        >= fee
                {
                    return true;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    fn should_be_paid(who: &AccountId, call: &RuntimeCall) -> bool {
        if call.is_called_by_bridge_peer(who) {
            return false;
        }
        true
    }

    fn compute_actual_fee(
        _post_info: &sp_runtime::traits::PostDispatchInfoOf<RuntimeCall>,
        _info: &sp_runtime::traits::DispatchInfoOf<RuntimeCall>,
        _result: &sp_runtime::DispatchResult,
        fee_details: Option<CustomFeeDetails>,
    ) -> Option<Balance> {
        let fee_details = fee_details?;
        match fee_details {
            CustomFeeDetails::Regular(fee) => Some(fee),
            CustomFeeDetails::LimitOrderLifetime(lifetime) => {
                order_book::fee_calculator::FeeCalculator::<Runtime>::place_limit_order_fee(
                    lifetime,
                    _post_info.actual_weight.is_some(),
                    _result.is_err(),
                )
            }
        }
    }

    fn get_fee_source(who: &AccountId, call: &RuntimeCall, _fee: Balance) -> AccountId {
        match call {
            RuntimeCall::Referrals(referrals::Call::set_referrer { .. })
                if Referrals::can_set_referrer(who) =>
            {
                ReferralsReservesAcc::get()
            }
            _ => who.clone(),
        }
    }
}

pub struct WithdrawFee;

impl xor_fee::WithdrawFee<Runtime> for WithdrawFee {
    fn withdraw_fee(
        who: &AccountId,
        fee_source: &AccountId,
        call: &RuntimeCall,
        fee: Balance,
    ) -> Result<(AccountId, Option<NegativeImbalanceOf<Runtime>>), DispatchError> {
        match call {
            RuntimeCall::Referrals(referrals::Call::set_referrer { referrer })
                // Fee source should be set to referrer by `get_fee_source` method, if not 
                // it means that user can't set referrer
                if Referrals::can_set_referrer(who) =>
            {
                Referrals::withdraw_fee(referrer, fee)?;
            }
            _ => {

            }
        }
        Ok((
            fee_source.clone(),
            Some(Balances::withdraw(
                fee_source,
                fee,
                WithdrawReasons::TRANSACTION_PAYMENT,
                ExistenceRequirement::KeepAlive,
            )?),
        ))
    }
}

#[cfg(test)]
mod tests {
    use pallet_utility::Call as UtilityCall;
    use sp_core::H256;
    use sp_runtime::AccountId32;

    use common::{balance, VAL, XOR};

    use crate::{xor_fee_impls::CustomFees, *};
    use xor_fee::ApplyCustomFees;

    #[test]
    fn check_calls_from_bridge_peers_pays_yes() {
        let call: &<Runtime as frame_system::Config>::RuntimeCall =
            &RuntimeCall::EthBridge(eth_bridge::Call::transfer_to_sidechain {
                asset_id: XOR.into(),
                to: Default::default(),
                amount: Default::default(),
                network_id: 0,
            });

        let who = AccountId32::from([0; 32]);

        assert!(CustomFees::should_be_paid(&who, call));
    }

    #[test]
    #[ignore] // TODO: fix check_calls_from_bridge_peers_pays_no test
    fn check_calls_from_bridge_peers_pays_no() {
        framenode_chain_spec::ext().execute_with(|| {
            let call: &<Runtime as frame_system::Config>::RuntimeCall =
                &RuntimeCall::EthBridge(eth_bridge::Call::finalize_incoming_request {
                    hash: H256::zero(),
                    network_id: 0,
                });

            let who = eth_bridge::BridgeAccount::<Runtime>::get(0).unwrap();

            assert!(!CustomFees::should_be_paid(&who, call));
        });
    }

    #[test]
    fn simple_call_should_pass() {
        let call = RuntimeCall::Assets(assets::Call::transfer {
            asset_id: GetBaseAssetId::get(),
            to: From::from([1; 32]),
            amount: balance!(100),
        });

        assert_eq!(call.swap_count(), 0);
    }

    #[test]
    fn regular_batch_should_pass() {
        let batch_calls = vec![
            assets::Call::transfer {
                asset_id: GetBaseAssetId::get(),
                to: From::from([1; 32]),
                amount: balance!(100),
            }
            .into(),
            assets::Call::transfer {
                asset_id: GetBaseAssetId::get(),
                to: From::from([1; 32]),
                amount: balance!(100),
            }
            .into(),
        ];

        let call_batch = RuntimeCall::Utility(UtilityCall::batch {
            calls: batch_calls.clone(),
        });
        let call_batch_all = RuntimeCall::Utility(UtilityCall::batch_all { calls: batch_calls });

        assert_eq!(call_batch.swap_count(), 0);
        assert_eq!(call_batch_all.swap_count(), 0);
    }

    fn test_swap_in_batch(call: RuntimeCall) {
        let batch_calls = vec![
            assets::Call::transfer {
                asset_id: GetBaseAssetId::get(),
                to: From::from([1; 32]),
                amount: balance!(100),
            }
            .into(),
            call,
        ];

        let call_batch = RuntimeCall::Utility(UtilityCall::batch {
            calls: batch_calls.clone(),
        });
        let call_batch_all = RuntimeCall::Utility(UtilityCall::batch_all { calls: batch_calls });

        assert_eq!(call_batch.swap_count(), 1);
        assert_eq!(call_batch_all.swap_count(), 1);

        assert!(crate::BaseCallFilter::contains(&call_batch));
        assert!(crate::BaseCallFilter::contains(&call_batch_all));
    }

    #[test]
    fn swap_in_batch_should_fail() {
        test_swap_in_batch(
            liquidity_proxy::Call::swap {
                dex_id: 0,
                input_asset_id: VAL,
                output_asset_id: XOR,
                swap_amount: common::prelude::SwapAmount::WithDesiredInput {
                    desired_amount_in: crate::balance!(100),
                    min_amount_out: crate::balance!(100),
                },
                selected_source_types: vec![],
                filter_mode: common::FilterMode::Disabled,
            }
            .into(),
        );
    }

    #[test]
    fn swap_transfer_in_batch_should_fail() {
        test_swap_in_batch(
            liquidity_proxy::Call::swap_transfer {
                receiver: From::from([1; 32]),
                dex_id: 0,
                input_asset_id: VAL,
                output_asset_id: XOR,
                swap_amount: common::prelude::SwapAmount::WithDesiredInput {
                    desired_amount_in: crate::balance!(100),
                    min_amount_out: crate::balance!(100),
                },
                selected_source_types: vec![],
                filter_mode: common::FilterMode::Disabled,
            }
            .into(),
        );
    }
}
