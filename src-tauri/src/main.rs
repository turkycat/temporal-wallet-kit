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
    Testnet,
    Mainnet,
}

// currently unused but that'll change soon
#[derive(Default)]
struct AppState {
    receive: String,
    change: String,
}

#[tauri::command]
fn verify_descriptor(descriptor: String) -> Result<DescriptorResponse, String> {
    let secp = Secp256k1::new();

    // TODO: this isn't sufficient since some descriptors are valid for both mainnet and testnet
    // example: wpkh(03d99179113327fc2a8349b4d47d1eac3033b51cbddcb59654c894320850500d4e)
    let is_testnet = descriptor.contains("tpub") || descriptor.contains("tprv");
    let network = match is_testnet {
        true => Network::Testnet,
        false => Network::Bitcoin,
    };

    match descriptor.into_wallet_descriptor(&secp, network) {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    };

    Ok(match is_testnet {
        true => DescriptorResponse::Testnet,
        false => DescriptorResponse::Mainnet,
    })
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn fetch_balance(receive: &str, change: &str) -> Result<String, String> {
    // TODO: this isn't sufficient since some descriptors are valid for both mainnet and testnet
    // example: wpkh(03d99179113327fc2a8349b4d47d1eac3033b51cbddcb59654c894320850500d4e)
    let is_testnet = receive.contains("tpub") || receive.contains("tprv");
    let network = match is_testnet {
        true => Network::Testnet,
        false => Network::Bitcoin,
    };

    let secp = Secp256k1::new();
    let receive = match receive.into_wallet_descriptor(&secp, network) {
        Ok(_) => receive.to_string(),
        Err(e) => return Err(e.to_string()),
    };

    let binding = change.to_string();
    let change = match change.into_wallet_descriptor(&secp, network) {
        Ok(_) => Some(&binding),
        Err(_) => None,
    };
    let wallet = match bdk::Wallet::new(
        &receive,
        change,
        network,
        bdk::database::MemoryDatabase::default(),
    ) {
        Ok(wallet) => wallet,
        Err(e) => return Err(e.to_string()),
    };

    let connection = match is_testnet {
        true => "electrum.blockstream.info:60001",
        false => "blockstream.info:110",
    };

    let client = Client::new(connection).unwrap();
    let blockchain: ElectrumBlockchain = ElectrumBlockchain::from(client);
    wallet.sync(&blockchain, SyncOptions::default()).unwrap();
    let balance = wallet.get_balance().unwrap();
    let transactions = wallet.list_transactions(false).unwrap();

    Ok(format!(
        "This {} wallet has a confirmed balance of {} satoshis with {} transactions",
        if is_testnet { "testnet" } else { "mainnet" },
        balance.confirmed,
        transactions.len()
    ))
}

fn main() {
    let app_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::default()));

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![verify_descriptor, fetch_balance])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
