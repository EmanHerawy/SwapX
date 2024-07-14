mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const SWAPMGR_TRACKED_CONTRACT: [u8; 20] = hex!("ff34e285f8ed393e366046153e3c16484a4dd674");

fn map_swapmgr_events(blk: &eth::Block, events: &mut contract::Events) {
    events.swapmgr_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SWAPMGR_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::swapmgr_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::SwapmgrApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            id: event.id.to_string(),
                            owner: event.owner,
                            spender: event.spender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.swapmgr_initializes.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SWAPMGR_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::swapmgr_contract::events::Initialize::match_and_decode(log) {
                        return Some(contract::SwapmgrInitialize {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            currency0: event.currency0,
                            currency1: event.currency1,
                            fee: event.fee.to_u64(),
                            hooks: event.hooks,
                            id: Vec::from(event.id),
                            tick_spacing: Into::<num_bigint::BigInt>::into(event.tick_spacing).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.swapmgr_modify_liquidities.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SWAPMGR_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::swapmgr_contract::events::ModifyLiquidity::match_and_decode(log) {
                        return Some(contract::SwapmgrModifyLiquidity {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: Vec::from(event.id),
                            liquidity_delta: event.liquidity_delta.to_string(),
                            sender: event.sender,
                            tick_lower: Into::<num_bigint::BigInt>::into(event.tick_lower).to_i64().unwrap(),
                            tick_upper: Into::<num_bigint::BigInt>::into(event.tick_upper).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.swapmgr_operator_sets.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SWAPMGR_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::swapmgr_contract::events::OperatorSet::match_and_decode(log) {
                        return Some(contract::SwapmgrOperatorSet {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            operator: event.operator,
                            owner: event.owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.swapmgr_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SWAPMGR_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::swapmgr_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::SwapmgrOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.swapmgr_protocol_fee_controller_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SWAPMGR_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::swapmgr_contract::events::ProtocolFeeControllerUpdated::match_and_decode(log) {
                        return Some(contract::SwapmgrProtocolFeeControllerUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            protocol_fee_controller: event.protocol_fee_controller,
                        });
                    }

                    None
                })
        })
        .collect());
    events.swapmgr_protocol_fee_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SWAPMGR_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::swapmgr_contract::events::ProtocolFeeUpdated::match_and_decode(log) {
                        return Some(contract::SwapmgrProtocolFeeUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: Vec::from(event.id),
                            protocol_fee: event.protocol_fee.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.swapmgr_swaps.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SWAPMGR_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::swapmgr_contract::events::Swap::match_and_decode(log) {
                        return Some(contract::SwapmgrSwap {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            fee: event.fee.to_u64(),
                            id: Vec::from(event.id),
                            liquidity: event.liquidity.to_string(),
                            sender: event.sender,
                            sqrt_price_x96: event.sqrt_price_x96.to_string(),
                            tick: Into::<num_bigint::BigInt>::into(event.tick).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.swapmgr_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SWAPMGR_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::swapmgr_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::SwapmgrTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            caller: event.caller,
                            from: event.from,
                            id: event.id.to_string(),
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_swapmgr_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.swapmgr_call_approves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Approve::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Approve::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::swapmgr_contract::functions::Approve::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrApproveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                id: decoded_call.id.to_string(),
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_burns.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Burn::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Burn::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::SwapmgrBurnCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                from: decoded_call.from,
                                id: decoded_call.id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_collect_protocol_fees.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::CollectProtocolFees::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::CollectProtocolFees::decode(call) {
                        Ok(decoded_call) => {
                            let output_amount_collected = match abi::swapmgr_contract::functions::CollectProtocolFees::output(&call.return_data) {
                                Ok(output_amount_collected) => {output_amount_collected}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrCollectProtocolFeesCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                currency: decoded_call.currency,
                                output_amount_collected: output_amount_collected.to_string(),
                                recipient: decoded_call.recipient,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_donates.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Donate::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Donate::decode(call) {
                        Ok(decoded_call) => {
                            let output_delta = match abi::swapmgr_contract::functions::Donate::output(&call.return_data) {
                                Ok(output_delta) => {output_delta}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrDonateCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount0: decoded_call.amount0.to_string(),
                                amount1: decoded_call.amount1.to_string(),
                                hook_data: decoded_call.hook_data,
                                output_delta: output_delta.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_initializes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Initialize::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Initialize::decode(call) {
                        Ok(decoded_call) => {
                            let output_tick = match abi::swapmgr_contract::functions::Initialize::output(&call.return_data) {
                                Ok(output_tick) => {output_tick}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                hook_data: decoded_call.hook_data,
                                output_tick: Into::<num_bigint::BigInt>::into(output_tick).to_i64().unwrap(),
                                sqrt_price_x96: decoded_call.sqrt_price_x96.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_mints.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Mint::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Mint::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::SwapmgrMintCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                id: decoded_call.id.to_string(),
                                to: decoded_call.to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_modify_liquidities.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::ModifyLiquidity::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::ModifyLiquidity::decode(call) {
                        Ok(decoded_call) => {
                            let (output_caller_delta, output_fees_accrued) = match abi::swapmgr_contract::functions::ModifyLiquidity::output(&call.return_data) {
                                Ok((output_caller_delta, output_fees_accrued)) => {(output_caller_delta, output_fees_accrued)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrModifyLiquidityCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                hook_data: decoded_call.hook_data,
                                output_caller_delta: output_caller_delta.to_string(),
                                output_fees_accrued: output_fees_accrued.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_set_operators.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::SetOperator::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::SetOperator::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::swapmgr_contract::functions::SetOperator::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrSetOperatorCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                approved: decoded_call.approved,
                                operator: decoded_call.operator,
                                output_param0: output_param0,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_set_protocol_fees.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::SetProtocolFee::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::SetProtocolFee::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::SwapmgrSetProtocolFeeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_protocol_fee: decoded_call.new_protocol_fee.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_set_protocol_fee_controllers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::SetProtocolFeeController::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::SetProtocolFeeController::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::SwapmgrSetProtocolFeeControllerCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                controller: decoded_call.controller,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_settles.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Settle::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Settle::decode(call) {
                        Ok(decoded_call) => {
                            let output_paid = match abi::swapmgr_contract::functions::Settle::output(&call.return_data) {
                                Ok(output_paid) => {output_paid}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrSettleCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                currency: decoded_call.currency,
                                output_paid: output_paid.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_swaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Swap::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Swap::decode(call) {
                        Ok(decoded_call) => {
                            let output_swap_delta = match abi::swapmgr_contract::functions::Swap::output(&call.return_data) {
                                Ok(output_swap_delta) => {output_swap_delta}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrSwapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                hook_data: decoded_call.hook_data,
                                output_swap_delta: output_swap_delta.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_syncs.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Sync::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Sync::decode(call) {
                        Ok(decoded_call) => {
                            let output_balance = match abi::swapmgr_contract::functions::Sync::output(&call.return_data) {
                                Ok(output_balance) => {output_balance}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrSyncCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                currency: decoded_call.currency,
                                output_balance: output_balance.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_takes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Take::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Take::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::SwapmgrTakeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                currency: decoded_call.currency,
                                to: decoded_call.to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_transfers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Transfer::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Transfer::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::swapmgr_contract::functions::Transfer::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrTransferCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                id: decoded_call.id.to_string(),
                                output_param0: output_param0,
                                receiver: decoded_call.receiver,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_transfer_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::TransferFrom::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::TransferFrom::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::swapmgr_contract::functions::TransferFrom::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrTransferFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                id: decoded_call.id.to_string(),
                                output_param0: output_param0,
                                receiver: decoded_call.receiver,
                                sender: decoded_call.sender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::SwapmgrTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_unlocks.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::Unlock::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::Unlock::decode(call) {
                        Ok(decoded_call) => {
                            let output_result = match abi::swapmgr_contract::functions::Unlock::output(&call.return_data) {
                                Ok(output_result) => {output_result}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SwapmgrUnlockCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                output_result: output_result,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.swapmgr_call_update_dynamic_lp_fees.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == SWAPMGR_TRACKED_CONTRACT && abi::swapmgr_contract::functions::UpdateDynamicLpFee::match_call(call))
                .filter_map(|call| {
                    match abi::swapmgr_contract::functions::UpdateDynamicLpFee::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::SwapmgrUpdateDynamicLpFeeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_dynamic_lp_fee: decoded_call.new_dynamic_lp_fee.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

#[substreams::handlers::map]
fn zipped_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}

fn graph_swapmgr_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.swapmgr_approvals.iter().for_each(|evt| {
        tables
            .create_row("swapmgr_approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("id", BigDecimal::from_str(&evt.id).unwrap())
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string());
    });
    events.swapmgr_initializes.iter().for_each(|evt| {
        tables
            .create_row("swapmgr_initialize", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("currency0", Hex(&evt.currency0).to_string())
            .set("currency1", Hex(&evt.currency1).to_string())
            .set("fee", evt.fee)
            .set("hooks", Hex(&evt.hooks).to_string())
            .set("id", Hex(&evt.id).to_string())
            .set("tick_spacing", evt.tick_spacing);
    });
    events.swapmgr_modify_liquidities.iter().for_each(|evt| {
        tables
            .create_row("swapmgr_modify_liquidity", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", Hex(&evt.id).to_string())
            .set("liquidity_delta", BigDecimal::from_str(&evt.liquidity_delta).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("tick_lower", evt.tick_lower)
            .set("tick_upper", evt.tick_upper);
    });
    events.swapmgr_operator_sets.iter().for_each(|evt| {
        tables
            .create_row("swapmgr_operator_set", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", evt.approved)
            .set("operator", Hex(&evt.operator).to_string())
            .set("owner", Hex(&evt.owner).to_string());
    });
    events.swapmgr_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row("swapmgr_ownership_transferred", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("user", Hex(&evt.user).to_string());
    });
    events.swapmgr_protocol_fee_controller_updateds.iter().for_each(|evt| {
        tables
            .create_row("swapmgr_protocol_fee_controller_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("protocol_fee_controller", Hex(&evt.protocol_fee_controller).to_string());
    });
    events.swapmgr_protocol_fee_updateds.iter().for_each(|evt| {
        tables
            .create_row("swapmgr_protocol_fee_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", Hex(&evt.id).to_string())
            .set("protocol_fee", evt.protocol_fee);
    });
    events.swapmgr_swaps.iter().for_each(|evt| {
        tables
            .create_row("swapmgr_swap", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount0", BigDecimal::from_str(&evt.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&evt.amount1).unwrap())
            .set("fee", evt.fee)
            .set("id", Hex(&evt.id).to_string())
            .set("liquidity", BigDecimal::from_str(&evt.liquidity).unwrap())
            .set("sender", Hex(&evt.sender).to_string())
            .set("sqrt_price_x96", BigDecimal::from_str(&evt.sqrt_price_x96).unwrap())
            .set("tick", evt.tick);
    });
    events.swapmgr_transfers.iter().for_each(|evt| {
        tables
            .create_row("swapmgr_transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("caller", Hex(&evt.caller).to_string())
            .set("from", Hex(&evt.from).to_string())
            .set("id", BigDecimal::from_str(&evt.id).unwrap())
            .set("to", Hex(&evt.to).to_string());
    });
}
fn graph_swapmgr_calls_out(calls: &contract::Calls, tables: &mut EntityChangesTables) {
    // Loop over all the abis calls to create table changes
    calls.swapmgr_call_approves.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_approve", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("id", BigDecimal::from_str(&call.id).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.swapmgr_call_burns.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_burn", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("from", Hex(&call.from).to_string())
            .set("id", BigDecimal::from_str(&call.id).unwrap());
    });
    calls.swapmgr_call_collect_protocol_fees.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_collect_protocol_fees", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("currency", Hex(&call.currency).to_string())
            .set("output_amount_collected", BigDecimal::from_str(&call.output_amount_collected).unwrap())
            .set("recipient", Hex(&call.recipient).to_string());
    });
    calls.swapmgr_call_donates.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_donate", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount0", BigDecimal::from_str(&call.amount0).unwrap())
            .set("amount1", BigDecimal::from_str(&call.amount1).unwrap())
            .set("hook_data", Hex(&call.hook_data).to_string())
            .set("output_delta", BigDecimal::from_str(&call.output_delta).unwrap());
    });
    calls.swapmgr_call_initializes.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_initialize", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("hook_data", Hex(&call.hook_data).to_string())
            .set("output_tick", call.output_tick)
            .set("sqrt_price_x96", BigDecimal::from_str(&call.sqrt_price_x96).unwrap());
    });
    calls.swapmgr_call_mints.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_mint", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("id", BigDecimal::from_str(&call.id).unwrap())
            .set("to", Hex(&call.to).to_string());
    });
    calls.swapmgr_call_modify_liquidities.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_modify_liquidity", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("hook_data", Hex(&call.hook_data).to_string())
            .set("output_caller_delta", BigDecimal::from_str(&call.output_caller_delta).unwrap())
            .set("output_fees_accrued", BigDecimal::from_str(&call.output_fees_accrued).unwrap());
    });
    calls.swapmgr_call_set_operators.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_set_operator", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("approved", call.approved)
            .set("operator", Hex(&call.operator).to_string())
            .set("output_param0", call.output_param0);
    });
    calls.swapmgr_call_set_protocol_fees.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_set_protocol_fee", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("new_protocol_fee", call.new_protocol_fee);
    });
    calls.swapmgr_call_set_protocol_fee_controllers.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_set_protocol_fee_controller", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("controller", Hex(&call.controller).to_string());
    });
    calls.swapmgr_call_settles.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_settle", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("currency", Hex(&call.currency).to_string())
            .set("output_paid", BigDecimal::from_str(&call.output_paid).unwrap());
    });
    calls.swapmgr_call_swaps.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_swap", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("hook_data", Hex(&call.hook_data).to_string())
            .set("output_swap_delta", BigDecimal::from_str(&call.output_swap_delta).unwrap());
    });
    calls.swapmgr_call_syncs.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_sync", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("currency", Hex(&call.currency).to_string())
            .set("output_balance", BigDecimal::from_str(&call.output_balance).unwrap());
    });
    calls.swapmgr_call_takes.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_take", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("currency", Hex(&call.currency).to_string())
            .set("to", Hex(&call.to).to_string());
    });
    calls.swapmgr_call_transfers.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_transfer", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("id", BigDecimal::from_str(&call.id).unwrap())
            .set("output_param0", call.output_param0)
            .set("receiver", Hex(&call.receiver).to_string());
    });
    calls.swapmgr_call_transfer_froms.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_transfer_from", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("id", BigDecimal::from_str(&call.id).unwrap())
            .set("output_param0", call.output_param0)
            .set("receiver", Hex(&call.receiver).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.swapmgr_call_transfer_ownerships.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_transfer_ownership", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("new_owner", Hex(&call.new_owner).to_string());
    });
    calls.swapmgr_call_unlocks.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_unlock", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("data", Hex(&call.data).to_string())
            .set("output_result", Hex(&call.output_result).to_string());
    });
    calls.swapmgr_call_update_dynamic_lp_fees.iter().for_each(|call| {
        tables
            .create_row("swapmgr_call_update_dynamic_lp_fee", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("new_dynamic_lp_fee", call.new_dynamic_lp_fee);
    });
  }
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_swapmgr_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
let mut calls = contract::Calls::default();
    map_swapmgr_calls(&blk, &mut calls);
    Ok(calls)
}
#[substreams::handlers::map]
fn graph_out(events: contract::Events, calls: contract::Calls) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_swapmgr_out(&events, &mut tables);
    graph_swapmgr_calls_out(&calls, &mut tables);
    Ok(tables.to_entity_changes())
}

