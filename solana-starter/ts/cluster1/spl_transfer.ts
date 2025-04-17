import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "/Users/jack/.config/solana/id.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("3198CfHzLwrhVZa73LT9pNXHT2TbDyHRqg83crdTJs4Z");

// Recipient address
const to = new PublicKey("7vbtBJLj54N5j33vV6wnU2CzrNuk7a2CWBGVUAZSGrpK");

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const fromAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    // Get the token account of the toWallet address, and if it does not exist, create it
    const toAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );
    // Transfer the new token to the "toTokenAccount" we just created
    const tx = await transfer(connection, keypair, fromAccount.address, toAccount.address, keypair, 10000000n);
    console.log(tx);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`)
  }
})();
