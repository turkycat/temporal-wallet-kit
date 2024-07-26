import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [balanceMessage, setBalanceMessage] = useState("");
  const [receive, setReceive] = useState("");
  const [change, setChange] = useState("");

  async function fetchBalance() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setBalanceMessage(await invoke("fetch_balance", { receive, change }));
  }

  return (
    <div className="container">
      <h1>Temporal Wallet Kit</h1>
      <p>
        This is a native desktop application with a React frontend that uses
        Tauri to interact with a Rust backend. It persists no data. You can
        easily check balances with this app. It could be easily extended to list
        transactions, and a simple sweep recovery feature that could require
        users to type their mnemonics for simple empergency recovery,
        potentially adding hardware device support. This took me about 2 hours
        to implement.
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
        <button type="submit">Submit</button>
      </form>

      <p>{balanceMessage}</p>
    </div>
  );
}

export default App;
