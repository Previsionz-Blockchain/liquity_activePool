mod abi;
mod pb;
mod util;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt, StoreNew};

use util::to_big_decimal;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;
use substreams::scalar::BigInt;

use substreams::pb::substreams::{Clock};

substreams_ethereum::init!();

const ACTIVEPOOL_TRACKED_CONTRACT: [u8; 20] = hex!("df9eb223bafbe5c5271415c75aecd68c21fe3d7f");

fn map_activepool_events(blk: &eth::Block, events: &mut contract::Events) {
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
                            u_new_active_pool_address: event.u_new_active_pool_address,
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
                            u_eth: event.u_eth.to_string(),
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
                            u_lusd_debt: event.u_lusd_debt.to_string(),
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
                            u_new_borrower_operations_address: event.u_new_borrower_operations_address,
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
                            u_new_default_pool_address: event.u_new_default_pool_address,
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_eth_balance_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::EthBalanceUpdated::match_and_decode(log) {
                        return Some(contract::ActivepoolEthBalanceUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_new_balance: event.u_new_balance.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_ether_sents.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::EtherSent::match_and_decode(log) {
                        return Some(contract::ActivepoolEtherSent {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_amount: event.u_amount.to_string(),
                            u_to: event.u_to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.activepool_lusd_balance_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACTIVEPOOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::activepool_contract::events::LusdBalanceUpdated::match_and_decode(log) {
                        return Some(contract::ActivepoolLusdBalanceUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_new_balance: event.u_new_balance.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
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
                            u_new_stability_pool_address: event.u_new_stability_pool_address,
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
                            u_new_trove_manager_address: event.u_new_trove_manager_address,
                        });
                    }

                    None
                })
        })
        .collect());
}

#[substreams::handlers::store]
fn eth_sent_store(events: contract::Events, o: StoreAddBigInt) {
    for update in events.activepool_ether_sents.into_iter() {
        o.add(
            0,
            format!("Update"),
            BigInt::from_str(&update.u_amount).unwrap(),
        );
        // o.add(
        //     0,
        //     format!("Account:{}", Hex(&draw.user).to_string()),
        //     BigInt::from_str(&draw.payout).unwrap(),
        // );
        // o.add(
        //     0,
        //     format!(
        //         "AccountDraw:{}:{}",
        //         Hex(&draw.user).to_string(),
        //         &draw.draw_id
        //     ),
        //     BigInt::from_str(&draw.payout).unwrap(),
        // );
        // o.add(
        //     0,
        //     format!("Aggregate"),
        //     BigInt::from_str(&draw.payout).unwrap(),
        // );
    }
}

// fn db_activepool_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
//     // Loop over all the abis events to create table changes
//     events.activepool_active_pool_address_changeds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_active_pool_address_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_new_active_pool_address", Hex(&evt.u_new_active_pool_address).to_string());
//     });
//     events.activepool_active_pool_eth_balance_updateds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_active_pool_eth_balance_updated", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_eth", BigDecimal::from_str(&evt.u_eth).unwrap());
//     });
//     events.activepool_active_pool_lusd_debt_updateds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_active_pool_lusd_debt_updated", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_lusd_debt", BigDecimal::from_str(&evt.u_lusd_debt).unwrap());
//     });
//     events.activepool_borrower_operations_address_changeds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_borrower_operations_address_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_new_borrower_operations_address", Hex(&evt.u_new_borrower_operations_address).to_string());
//     });
//     events.activepool_default_pool_address_changeds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_default_pool_address_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_new_default_pool_address", Hex(&evt.u_new_default_pool_address).to_string());
//     });
//     events.activepool_eth_balance_updateds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_eth_balance_updated", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_new_balance", BigDecimal::from_str(&evt.u_new_balance).unwrap());
//     });
//     events.activepool_ether_sents.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_ether_sent", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_amount", BigDecimal::from_str(&evt.u_amount).unwrap())
//             .set("u_to", Hex(&evt.u_to).to_string());
//     });
//     events.activepool_lusd_balance_updateds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_lusd_balance_updated", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_new_balance", BigDecimal::from_str(&evt.u_new_balance).unwrap());
//     });
//     events.activepool_ownership_transferreds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_ownership_transferred", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("new_owner", Hex(&evt.new_owner).to_string())
//             .set("previous_owner", Hex(&evt.previous_owner).to_string());
//     });
//     events.activepool_stability_pool_address_changeds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_stability_pool_address_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_new_stability_pool_address", Hex(&evt.u_new_stability_pool_address).to_string());
//     });
//     events.activepool_trove_manager_address_changeds.iter().for_each(|evt| {
//         tables
//             .create_row("activepool_trove_manager_address_changed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
//             .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
//             .set("evt_block_number", evt.evt_block_number)
//             .set("u_new_trove_manager_address", Hex(&evt.u_new_trove_manager_address).to_string());
//     });
// }


fn graph_activepool_out(events: &contract::Events, tables: &mut EntityChangesTables, clock: Clock, eth_sent_store: StoreGetBigInt) {
    // Loop over all the abis events to create table changes

    let bigdecimal0 = BigDecimal::zero();
    if clock.number == 12178562 {
        tables
            .create_row("activePool", format!("pool"))
            .set("troveManagerAddress", "")
            .set("borrowerOperationsAddress", "")
            .set("stabilityPoolAddress", "")
            .set("defaultPoolAddress", "")
            .set("ETH", &bigdecimal0)
            .set("LUSDDebt", &bigdecimal0);
    }
    
    // events.activepool_active_pool_address_changeds.iter().for_each(|evt| {
    //     tables
    //         .create_row("activepool_active_pool_address_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
    //         .set("evt_tx_hash", &evt.evt_tx_hash)
    //         .set("evt_index", evt.evt_index)
    //         .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
    //         .set("evt_block_number", evt.evt_block_number)
    //         .set("u_new_active_pool_address", Hex(&evt.u_new_active_pool_address).to_string());

    // });
    events.activepool_active_pool_eth_balance_updateds.iter().for_each(|evt| {
        tables
            .create_row("activepool_active_pool_eth_balance_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_eth", BigDecimal::from_str(&evt.u_eth).unwrap());

        tables
            .update_row("activePool", format!("pool"))
            .set("ETH", BigDecimal::from_str(&evt.u_eth).unwrap());
    });
    events.activepool_active_pool_lusd_debt_updateds.iter().for_each(|evt| {
        tables
            .create_row("activepool_active_pool_lusd_debt_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_lusd_debt", BigDecimal::from_str(&evt.u_lusd_debt).unwrap());

        tables
            .update_row("activePool", format!("pool"))
            .set("LUSDDebt", BigDecimal::from_str(&evt.u_lusd_debt).unwrap());
    });
    events.activepool_borrower_operations_address_changeds.iter().for_each(|evt| {
        tables
            .create_row("activepool_borrower_operations_address_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_new_borrower_operations_address", Hex(&evt.u_new_borrower_operations_address).to_string());

            tables
            .update_row("activePool", format!("pool"))
            .set("borrowerOperationsAddress", Hex(&evt.u_new_borrower_operations_address).to_string());
    });
    events.activepool_default_pool_address_changeds.iter().for_each(|evt| {
        tables
            .create_row("activepool_default_pool_address_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_new_default_pool_address", Hex(&evt.u_new_default_pool_address).to_string());

        tables
            .update_row("activePool", format!("pool"))
            .set("defaultPoolAddress", Hex(&evt.u_new_default_pool_address).to_string());
    });
    // events.activepool_eth_balance_updateds.iter().for_each(|evt| {
    //     tables
    //         .create_row("activepool_eth_balance_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
    //         .set("evt_tx_hash", &evt.evt_tx_hash)
    //         .set("evt_index", evt.evt_index)
    //         .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
    //         .set("evt_block_number", evt.evt_block_number)
    //         .set("u_new_balance", BigDecimal::from_str(&evt.u_new_balance).unwrap());
    // });
    events.activepool_ether_sents.iter().for_each(|evt| {
        tables
            .create_row("activepool_ether_sent", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_amount", BigDecimal::from_str(&evt.u_amount).unwrap())
            .set("u_to", Hex(&evt.u_to).to_string());
        
        if let Some(total) = eth_sent_store.get_last(format!("Update")) {
            tables
                .update_row("activePool", format!("pool"))
                .set("totalEthSent", to_big_decimal(total.to_string().as_str(), 18).unwrap());
        }
    });
    // events.activepool_lusd_balance_updateds.iter().for_each(|evt| {
    //     tables
    //         .create_row("activepool_lusd_balance_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
    //         .set("evt_tx_hash", &evt.evt_tx_hash)
    //         .set("evt_index", evt.evt_index)
    //         .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
    //         .set("evt_block_number", evt.evt_block_number)
    //         .set("u_new_balance", BigDecimal::from_str(&evt.u_new_balance).unwrap());
    // });
    events.activepool_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row("activepool_ownership_transferred", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.activepool_stability_pool_address_changeds.iter().for_each(|evt| {
        tables
            .create_row("activepool_stability_pool_address_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_new_stability_pool_address", Hex(&evt.u_new_stability_pool_address).to_string());
        
        tables
            .update_row("activePool", format!("pool"))
            .set("stabilityPoolAddress", Hex(&evt.u_new_stability_pool_address).to_string());
    });
    events.activepool_trove_manager_address_changeds.iter().for_each(|evt| {
        tables
            .create_row("activepool_trove_manager_address_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_new_trove_manager_address", Hex(&evt.u_new_trove_manager_address).to_string());

        tables
            .update_row("activePool", format!("pool"))
            .set("troveManagerAddress", Hex(&evt.u_new_trove_manager_address).to_string());
    });
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_activepool_events(&blk, &mut events);
    Ok(events)
}

// #[substreams::handlers::map]
// fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
//     // Initialize Database Changes container
//     let mut tables = DatabaseChangeTables::new();
//     db_activepool_out(&events, &mut tables);
//     Ok(tables.to_database_changes())
// }

#[substreams::handlers::map]
fn graph_out(events: contract::Events, clock: Clock, eth_sent_store: StoreGetBigInt) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_activepool_out(&events, &mut tables, clock, eth_sent_store);
    Ok(tables.to_entity_changes())
}
