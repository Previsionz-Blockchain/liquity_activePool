mod abi;
mod pb;
mod util;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::store::{
    StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigDecimal, StoreGetBigInt, StoreNew,
};
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

use util::to_big_decimal;
extern crate hex;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::ops::Mul;
use std::str::FromStr;
use substreams::scalar::BigDecimal;
use substreams::scalar::BigInt;

use substreams::pb::substreams::Clock;

substreams_ethereum::init!();

const ACTIVEPOOL_TRACKED_CONTRACT: [u8; 20] = hex!("df9eb223bafbe5c5271415c75aecd68c21fe3d7f");

fn map_activepool_events(
    blk: &eth::Block,
    events: &mut contract::Events,
    chainlink_prices: StoreGetBigDecimal,
    clock: Clock,
) {
    let eth_price = get_eth_price(chainlink_prices);
    let mut contract_name = contract::ContractName::default();

    events.activepool_active_pool_address_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::ActivePoolAddressChanged::match_and_decode(log) {
                        return Some(contract::ActivepoolActivePoolAddressChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_active_pool_address: event.u_new_active_pool_address,
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_active_pool_eth_balance_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::ActivePoolEthBalanceUpdated::match_and_decode(log) {
                        return Some(contract::ActivepoolActivePoolEthBalanceUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            eth: event.u_eth.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_active_pool_lusd_debt_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::ActivePoolLusdDebtUpdated::match_and_decode(log) {
                        return Some(contract::ActivepoolActivePoolLusdDebtUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            lusd_debt: event.u_lusd_debt.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_borrower_operations_address_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::BorrowerOperationsAddressChanged::match_and_decode(log) {
                        return Some(contract::ActivepoolBorrowerOperationsAddressChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_borrower_operations_address: event.u_new_borrower_operations_address,
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_default_pool_address_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::DefaultPoolAddressChanged::match_and_decode(log) {
                        return Some(contract::ActivepoolDefaultPoolAddressChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_default_pool_address: event.u_new_default_pool_address,
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_eth_balance_updateds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::activepool_contract::events::EthBalanceUpdated::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::ActivepoolEthBalanceUpdated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_balance: event.u_new_balance.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.activepool_ether_sents.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT) // Check if the log is from the targeted contract
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::activepool_contract::events::EtherSent::match_and_decode(log)
                        {
                            // Check if u_amount is not zero
                            if event.u_amount != BigInt::from(0) {
                                let convert_amount = BigInt::from_str(&event.u_amount.to_string())
                                    .unwrap_or(BigInt::from(0))
                                    .to_decimal(BigInt::from(18).to_u64());

                                return Some(contract::ActivepoolEtherSent {
                                    evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                    evt_index: log.block_index,
                                    evt_block_time: Some(blk.timestamp().to_owned()),
                                    evt_block_number: blk.number,
                                    amount: event.u_amount.to_string(),
                                    to: event.u_to,
                                    usd_value: eth_price.clone().mul(convert_amount).to_string(),
                                });
                            }
                        }
                        None
                    })
            })
            .collect(),
    );
    events.activepool_lusd_balance_updateds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::activepool_contract::events::LusdBalanceUpdated::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::ActivepoolLusdBalanceUpdated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_balance: event.u_new_balance.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.activepool_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::ActivepoolOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_stability_pool_address_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::StabilityPoolAddressChanged::match_and_decode(log) {
                        return Some(contract::ActivepoolStabilityPoolAddressChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_stability_pool_address: event.u_new_stability_pool_address,
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_trove_manager_address_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::TroveManagerAddressChanged::match_and_decode(log) {
                        return Some(contract::ActivepoolTroveManagerAddressChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_trove_manager_address: event.u_new_trove_manager_address,
                        });
                    }

                    None
                })
        })
        .collect());

    if clock.number == 12178562 {
        contract_name.name = get_name().unwrap_or("".to_string());
        events.contract_name = Some(contract_name);
    }
}

//Using chainlink substream to get ETH/USD price data
fn get_eth_price(chainlink_prices: StoreGetBigDecimal) -> BigDecimal {
    let eth_price = chainlink_prices
        .get_last("price_by_symbol:ETH:USD")
        .map_or(BigDecimal::from(0), |price| price);
    substreams::log::println(format!("{:?}", eth_price));
    eth_price
}

// RPC call to get the name of the contract
fn get_name() -> Option<String> {
    let name = abi::activepool_contract::functions::Name {};
    let name_options = name.call(ACTIVEPOOL_TRACKED_CONTRACT.to_vec());

    if let Some(name_value) = name_options {
        Some(name_value)
    } else {
        substreams::log::info!("Failed to get contract name, returned None.");
        None
    }
}

// Addig up the amount of total ETH sent and storing it
#[substreams::handlers::store]
fn eth_sent_store(events: contract::Events, o: StoreAddBigInt) {
    for update in events.activepool_ether_sents.into_iter() {
        o.add(
            0,
            format!("Update"),
            BigInt::from_str(&update.amount).unwrap(),
        );
    }
}

fn graph_activepool_out(
    events: &contract::Events,
    tables: &mut EntityChangesTables,
    clock: Clock,
    eth_sent_store: StoreGetBigInt,
) {
    let bigdecimal0 = BigDecimal::zero();
    let contract_address = hex::encode(ACTIVEPOOL_TRACKED_CONTRACT);
    let contract_address = format!("0x{}", contract_address);
    let name = events
        .contract_name
        .as_ref()
        .map_or("", |name| name.name.as_str());

    if clock.number == 12178562 {
        tables
            .create_row("Pool", contract_address.clone())
            .set("name", name)
            .set("trove_manager_address", "")
            .set("borrower_operations_address", "")
            .set("stability_pool_address", "")
            .set("default_pool_address", "")
            .set("eth", &bigdecimal0)
            .set("lusd_debt", &bigdecimal0);
    }

    events
        .activepool_active_pool_eth_balance_updateds
        .iter()
        .for_each(|evt| {
            tables
                .update_row("Pool", contract_address.clone())
                .set("eth", BigDecimal::from_str(&evt.eth).unwrap());
        });
    events
        .activepool_active_pool_lusd_debt_updateds
        .iter()
        .for_each(|evt| {
            tables
                .update_row("Pool", contract_address.clone())
                .set("lusd_debt", BigDecimal::from_str(&evt.lusd_debt).unwrap());
        });
    events
        .activepool_borrower_operations_address_changeds
        .iter()
        .for_each(|evt| {
            tables.update_row("Pool", contract_address.clone()).set(
                "borrower_operations_address",
                Hex(&evt.new_borrower_operations_address).to_string(),
            );
        });
    events
        .activepool_default_pool_address_changeds
        .iter()
        .for_each(|evt| {
            tables.update_row("Pool", contract_address.clone()).set(
                "default_pool_address",
                Hex(&evt.new_default_pool_address).to_string(),
            );
        });

    events.activepool_ether_sents.iter().for_each(|evt| {
        tables
            .create_row("EthSent", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", to_big_decimal(&evt.amount, 18).unwrap())
            .set("to", Hex(&evt.to).to_string())
            .set("usd_value", BigDecimal::from_str(&evt.usd_value).unwrap())
            .set("pool", contract_address.clone());

        if let Some(total) = eth_sent_store.get_last(format!("Update")) {
            tables.update_row("Pool", contract_address.clone()).set(
                "total_eth_sent",
                to_big_decimal(total.to_string().as_str(), 18).unwrap(),
            );
        }
    });
    events
        .activepool_ownership_transferreds
        .iter()
        .for_each(|evt| {
            tables
                .update_row("Pool", contract_address.clone())
                .set("current_owner", Hex(&evt.new_owner).to_string())
                .set("previous_owner", Hex(&evt.previous_owner).to_string());
        });
    events
        .activepool_stability_pool_address_changeds
        .iter()
        .for_each(|evt| {
            tables.update_row("Pool", contract_address.clone()).set(
                "stability_pool_address",
                Hex(&evt.new_stability_pool_address).to_string(),
            );
        });
    events
        .activepool_trove_manager_address_changeds
        .iter()
        .for_each(|evt| {
            tables.update_row("Pool", contract_address.clone()).set(
                "trove_manager_address",
                Hex(&evt.new_trove_manager_address).to_string(),
            );
        });
}

#[substreams::handlers::map]
fn map_events(
    blk: eth::Block,
    chainlink_prices: StoreGetBigDecimal,
    clock: Clock,
) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_activepool_events(&blk, &mut events, chainlink_prices, clock);
    Ok(events)
}

#[substreams::handlers::map]
fn graph_out(
    events: contract::Events,
    clock: Clock,
    eth_sent_store: StoreGetBigInt,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = EntityChangesTables::new();
    graph_activepool_out(&events, &mut tables, clock, eth_sent_store);
    Ok(tables.to_entity_changes())
}
