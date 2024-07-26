import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [balanceMessage, setBalanceMessage] = useState("");
  const [receive, setReceive] = useState("");
  const [change, setChange] = useState("");
  const [isTestnet, setIsTestnet] = useState(false);

  async function fetchBalance() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setBalanceMessage(
      await invoke("fetch_balance", { receive, change, isTestnet })
    );
  }

  return (
    <div className="container">
      <h1>Temporal Wallet Kit</h1>
      <p>
        This is a simple React app that uses Tauri to interact with a Rust
        backend. You can easily check balances with this app. It could be easily
        extended to list transactions, and a simple sweep recovery feature that
        could require users to type their mnemonics for simple empergency
        recovery.
      </p>

      <div className="row">
        <img src="/bitcoin.png" className="logo bitcoin" alt="bitcoin" />
      </div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          fetchBalance();
        }}
      >
        <input
          id="receive-input"
          onChange={(e) => setReceive(e.currentTarget.value)}
          placeholder="Receive descriptor:"
        />
        <input
          id="change-input"
          onChange={(e) => setChange(e.currentTarget.value)}
          placeholder="Change descriptor:"
        />
        <div>
          <input
            type="checkbox"
            id="testnet-checkbox"
            checked={isTestnet}
            onChange={(e) => setIsTestnet(e.currentTarget.checked)}
          />
          <label htmlFor="testnet-checkbox">is testnet</label>
        </div>
        <button type="submit">Submit</button>
      </form>

      <p>{balanceMessage}</p>
    </div>
  );
}

export default App;
