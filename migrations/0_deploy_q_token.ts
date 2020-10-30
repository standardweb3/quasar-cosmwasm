const {
  MnemonicKey,
  LCDClient,
  MsgStoreCode,
  MsgInstantiateContract,
  Coins,
  MsgExecuteContract,
  Coin,
  MsgSend,
} = require("@terra-money/terra.js");

async function deploy() {
  console.log("Setting up accounts and contracts...");
  // Setup an account with key from mnemonic phrases
  const mk1 = new MnemonicKey({
    mnemonic:
      "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius",
  });
  // Setup a network provider to connect
  const terra = new LCDClient({
    URL: "http://localhost:1317",
    chainID: "localterra",
  });

  // Create a wallet which points to the network provider
  const test1 = terra.wallet(mk1);

  const fs = require("fs");
  // To deploy a contract, you must get an wasm file.
  const starterCode = fs.readFileSync("../wasm/q_native.wasm");

  // Create tx to sign and send to the blockchain
  const storeStarter = new MsgStoreCode(
    test1.key.accAddress,
    starterCode.toString("base64")
  );

  // Create a batch of txs to send to the blockchain
  const storeCodeTx = await test1
    .createAndSignTx({
      msgs: [storeStarter],
    })
    .catch((error: any) => {
      console.log(error);
    });

  // Get results with codeId
  const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx);
  const qTokenId = storeCodeTxResult.logs[0].events[1].attributes[1].value;

  console.log("Starter code id", qTokenId);
  // Cosmwasm smart contracts need to instantiate from the uploaded wasmer executable binary.
  const instantiateStarter = new MsgInstantiateContract(
    test1.key.accAddress,
    +qTokenId,
    {
      borrow_index: "100000000", // one in decimal of 8
      decimals: 8,
      denom: "uluna",
      initial_exchange_rate: "2000000", // 0.02 * 10^8,
      max_borrow_rate: "1000000000", // just a mock number for now,
      name: "qLuna",
      reserve_factor: "5000000", // 0.05 * 10^8,
      symbol: "qLUNA",
      total_supply: "0"
    },
    new Coins({}),
    false
  );

  // Create tx batch again
  const instantiateTx = await test1.createAndSignTx({
    msgs: [instantiateStarter],
  }).catch((error: any) => {
    console.log(error);
  });;

  // Get address from executing tx
  const instantiateTxResult = await terra.tx.broadcast(instantiateTx);

  const qLunaAddress =
    instantiateTxResult.logs[0].events[0].attributes[2].value;

  console.log("qLuna address", qLunaAddress);

  // To interact with smart contract, check generated contract's schemas in the schemas/ folder.
  // For example, to set name in starter contract

  const mintQLuna = new MsgExecuteContract(
    test1.key.accAddress,
    qLunaAddress,
    {
      mint: {},
    },
    new Coins([new Coin("uluna", "1000000000000")])
  );

  const interactTx = await test1.createAndSignTx({
    msgs: [mintQLuna],
  });

  const interactTxResult = await terra.tx.broadcast(interactTx);

  console.log(interactTxResult);
}

deploy();
