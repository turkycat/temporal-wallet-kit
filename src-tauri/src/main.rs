// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::descriptor::{ExtendedDescriptor, IntoWalletDescriptor};
use std::sync::{Arc, Mutex};
use tauri::State;

use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::SyncOptions;
use bdk::{self, wallet};

#[derive(Default, serde::Serialize)]
enum DescriptorResponse {
    #[default]
    None,
    Testnet,
    Mainnet,
}

// TODO: add a way to manually set the network
#[derive(Default)]
struct AppState {
    wallet: Option<bdk::Wallet<MemoryDatabase>>,
    network: Option<Network>,
}

#[tauri::command]
fn reset(state: State<Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.wallet = None;
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

#[tauri::command]
fn set_wallet(
    state: State<Mutex<AppState>>,
    receive: String,
    change: String,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();

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

    state.wallet = Some(wallet);
    state.network = Some(network);
    Ok(())
}

#[tauri::command]
fn fetch_balance(state: State<Mutex<AppState>>) -> Result<String, String> {
    let mut state = state.lock().unwrap();

    let network = state.network.as_ref().ok_or("No network set")?.to_owned();
    let wallet = state.wallet.as_ref().ok_or("No wallet set")?;

    let connection = match network {
        Network::Testnet => "electrum.blockstream.info:60001",
        Network::Bitcoin => "blockstream.info:110",
        _ => return Err("Unsupported network".to_string()),
    };

    let client = Client::new(connection).unwrap();
    let blockchain: ElectrumBlockchain = ElectrumBlockchain::from(client);
    wallet.sync(&blockchain, SyncOptions::default()).unwrap();
    let balance = wallet.get_balance().unwrap();
    let transactions = wallet.list_transactions(false).unwrap();

    Ok(format!(
        "This {} wallet has a confirmed balance of {} satoshis with {} transactions",
        if network == Network::Testnet {
            "testnet"
        } else {
            "mainnet"
        },
        balance.confirmed,
        transactions.len()
    ))
}

fn main() {
    // let app_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::default()));
    let app_state: Mutex<AppState> = Mutex::new(AppState::default());
    // let app_state = AppState::default();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            fetch_balance,
            reset,
            set_wallet,
            verify_descriptor
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
