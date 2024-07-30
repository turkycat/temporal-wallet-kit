import { useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

enum DescriptorResponse {
  None = "None",
  Invalid = "Invalid",
  Testnet = "Testnet",
  Mainnet = "Mainnet",
}

function App() {
  const [message, setMessage] = useState("");
  const [receive, setReceive] = useState("");
  const [isReceiveValid, setIsReceiveValid] = useState(false);
  const [change, setChange] = useState("");
  const [isChangeValid, setIsChangeValid] = useState(false);
  const receiveForm = useRef<HTMLFormElement>(null);
  const changeForm = useRef<HTMLFormElement>(null);

  let receiveNetwork = DescriptorResponse.None;

  async function fetchBalance() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setMessage(await invoke("fetch_balance", { receive, change }));
  }

  async function reset() {
    setIsReceiveValid(false);
    setIsChangeValid(false);
    if (receiveForm.current) {
      receiveForm.current.reset();
    }
    if (changeForm.current) {
      changeForm.current.reset();
    }
  }

  async function validateReceive() {
    const res = await invoke("set_receive", { receive });

    let msg: string | undefined = undefined;
    switch (res) {
      case DescriptorResponse.None:
        setIsReceiveValid(false);
        setMessage(
          "Something unexpected went wrong with the descriptor, verify the value and try again."
        );
        break;
      case DescriptorResponse.Invalid:
        setIsReceiveValid(false);
        setMessage("The receive descriptor you've entered is not valid.");
        break;

      // @ts-ignore(7029)
      case DescriptorResponse.Testnet:
        msg =
          "The receive descriptor you've entered valid for the test network.";
      case DescriptorResponse.Mainnet:
        msg = msg || "The receive descriptor you've entered is valid!";
        receiveNetwork = res;
        setMessage(msg);
        setIsReceiveValid(true);
        break;
    }
  }

  async function validateChange() {
    const res = await invoke("set_change", { change });
    switch (res) {
      case DescriptorResponse.None:
        setIsChangeValid(false);
        setMessage(
          "Something unexpected went wrong with the descriptor, verify the value and try again."
        );
        break;
      case DescriptorResponse.Invalid:
        setIsReceiveValid(false);
        setMessage("The change descriptor you've entered is not valid.");
        break;
      case DescriptorResponse.Testnet:
      case DescriptorResponse.Mainnet:
        receiveNetwork = res;
        setIsChangeValid(true);
        break;
    }
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
        ref={receiveForm}
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          validateReceive();
        }}
      >
        <input
          id="receive-input"
          onChange={(e) => setReceive(e.currentTarget.value)}
          placeholder="Receive descriptor:"
        />
        <button type="submit" disabled={isReceiveValid}>
          {isReceiveValid ? "Locked In" : "Check"}
        </button>
      </form>

      {isReceiveValid ? (
        <form
          ref={changeForm}
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            validateChange();
          }}
        >
          <input
            id="change-input"
            onChange={(e) => setChange(e.currentTarget.value)}
            placeholder="Change descriptor (optional):"
          />
          <button type="submit" disabled={isChangeValid}>
            {isChangeValid ? "Locked In" : "Check"}
          </button>
        </form>
      ) : null}

      <p>{message}</p>

      {isReceiveValid ? (
        <div>
          {/* <button className="action" type="button" onClick={fetchBalance}>
            Get Balance
          </button> */}
          <button className="action" type="button" onClick={reset}>
            Reset
          </button>
        </div>
      ) : null}
    </div>
  );
}

export default App;
