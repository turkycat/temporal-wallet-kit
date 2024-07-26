// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bdk;
use bdk::blockchain::Blockchain;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::{miniscript, KeychainKind, SyncOptions, Wallet};
use std::sync::Mutex;

static CLIENT: Mutex<Option<Client>> = Mutex::new(None);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn fetch_balance(receive: &str, change: &str, isTestnet: bool) -> String {
    let receive: String = receive.to_string();
    let change: String = change.to_string();
    let network: bdk::bitcoin::Network = match isTestnet {
        true => bdk::bitcoin::Network::Testnet,
        false => bdk::bitcoin::Network::Bitcoin,
    };
    let wallet = bdk::Wallet::new(
        &receive,
        Some(&change),
        network,
        bdk::database::MemoryDatabase::default(),
    )
    .unwrap();
    let connection = match isTestnet {
        true => "electrum.blockstream.info:60001",
        false => "blockstream.info:110",
    };
    let client = Client::new(connection).unwrap();
    let blockchain: ElectrumBlockchain = ElectrumBlockchain::from(client);
    wallet.sync(&blockchain, SyncOptions::default()).unwrap();
    let balance = wallet.get_balance().unwrap();
    let transactions = wallet.list_transactions(false).unwrap();
    // {
    //     let client_guard = CLIENT.lock().unwrap();
    //     if let Some(client) = &*client_guard {
    //         let blockchain: ElectrumBlockchain = ElectrumBlockchain::from(client.);

    //         wallet.sync(&client, SyncOptions::default()).unwrap();
    //         client.do_something();
    //     }
    // }
    format!(
        "This wallet has a confirmed balance of {} satoshis with {} transactions",
        balance.confirmed,
        transactions.len()
    )
}

fn main() {
    // {
    //     let mut client_guard = CLIENT.lock().unwrap();
    //     *client_guard = Some(Client::new("blockstream.info:110").unwrap());
    // }

    // client = Some(Client::new("blockstream.info:110").unwrap());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_balance])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
