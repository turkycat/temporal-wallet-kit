# Temporal Wallet Kit

This is a native desktop application with a React frontend that uses
Tauri to interact with a Rust backend. It persists no data. You can
easily check balances with this app. It could be easily extended to list
transactions, and a simple sweep recovery feature that could require
users to type their mnemonics for simple empergency recovery,
potentially adding hardware device support.

## Setup Instructions

1. Verify you have rust installed

```bash
$ rustc --version
rustc 1.80.0 (051478957 2024-07-21)
```

to install:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

to update:

```bash
rustup update
```

2. checkout this repo and cd into it
3. `pnpm install`
4. `pnpm tauri dev`

## usage instructions

paste both descriptors in and hit submit. app will probably crash if you leave one blank or the descriptor is invalid. app will probably crash if you mix testnet and mainnet descriptors. this is a proof of concept.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
