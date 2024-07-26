// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bdk;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::SyncOptions;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn fetch_balance(receive: &str, change: &str) -> String {
    let is_testnet = receive.contains("tpub") || receive.contains("tprv");
    let receive: String = receive.to_string();
    let change: String = change.to_string();
    let network: bdk::bitcoin::Network = match is_testnet {
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
    let connection = match is_testnet {
        true => "electrum.blockstream.info:60001",
        false => "blockstream.info:110",
    };
    let client = Client::new(connection).unwrap();
    let blockchain: ElectrumBlockchain = ElectrumBlockchain::from(client);
    wallet.sync(&blockchain, SyncOptions::default()).unwrap();
    let balance = wallet.get_balance().unwrap();
    let transactions = wallet.list_transactions(false).unwrap();
    format!(
        "This {} wallet has a confirmed balance of {} satoshis with {} transactions",
        if is_testnet { "testnet" } else { "mainnet" },
        balance.confirmed,
        transactions.len()
    )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_balance])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
