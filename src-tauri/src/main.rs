// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::Network;
use bdk::descriptor::{ExtendedDescriptor, IntoWalletDescriptor};
use std::sync::{Arc, Mutex};
use tauri::State;

use bdk;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::SyncOptions;

#[derive(Default, serde::Serialize)]
enum DescriptorResponse {
    #[default]
    None,
    Invalid,
    Testnet,
    Mainnet,
}

#[derive(Default)]
struct AppState {
    receive: String,
    change: String,
}

#[tauri::command]
fn set_receive(state: State<Arc<Mutex<AppState>>>, receive: String) -> DescriptorResponse {
    let mut app_state = match state.lock() {
        Ok(state) => state,
        Err(_) => return DescriptorResponse::None,
    };

    let secp = Secp256k1::new();

    let is_testnet = receive.contains("tpub") || receive.contains("tprv");
    let network = match is_testnet {
        true => Network::Testnet,
        false => Network::Bitcoin,
    };

    match receive.into_wallet_descriptor(&secp, network) {
        Ok(_) => {}
        Err(_) => return DescriptorResponse::Invalid,
    };

    app_state.receive = receive;
    match is_testnet {
        true => DescriptorResponse::Testnet,
        false => DescriptorResponse::Mainnet,
    }
}

#[tauri::command]
fn set_change(state: State<Arc<Mutex<AppState>>>, change: String) {
    let mut app_state = state.lock().unwrap();
    app_state.change = change;
}

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
    let app_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::default()));

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            set_receive,
            set_change,
            fetch_balance
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
