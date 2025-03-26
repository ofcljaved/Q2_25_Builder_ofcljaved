import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    const txHash = await connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);
    console.log(`Success! Check out your Transaction here: https://explorer.solana.com/tx/${txHash}?cluster=devnet`);

    //https://explorer.solana.com/tx/NyKibmTxrABorhC3AMdtQ2SqCfRP5gFyGZvEtw3LHeAc7SfjPSeyiVgCCb9mjEb6aEByBtxAWLbiBkTctTV4fXd?cluster=devnet


  }
  catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
