import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("7vbtBJLj54N5j33vV6wnU2CzrNuk7a2CWBGVUAZSGrpK");
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    const balance = await connection.getBalance(from.publicKey);
    console.log(`Balance: ${balance} SOL`);

    const txn = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance,
      })
    );
    txn.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
    txn.feePayer = from.publicKey;

    const fee = (await connection.getFeeForMessage(txn.compileMessage(), 'confirmed')).value || 0;

    txn.instructions.pop();
    txn.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      })
    );

    const signature = await sendAndConfirmTransaction(
      connection,
      txn,
      [from]
    );
    console.log(`Success! Check out your Transaction here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);

  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();

// Initial Transfer: https://explorer.solana.com/tx/2neH5ZPBDqbq3bbz2BxeFBZ5wLe8oFxFbf1tJcoXP4wQMsJ4aXnBse3WCp7ahbRWDPWDFMEXxB4f1kNPQcKKtG6m?cluster=devnet
// Complete Transfer: https://explorer.solana.com/tx/2QsBmP5JvFQorK8P5pFSCPqZjeWZmGMbMmRafJyLb2AygJ5KFqP6aEWLd7oQdcQks2A1U7eXe2bb4MKn8cMCKw6D?cluster=devnet
