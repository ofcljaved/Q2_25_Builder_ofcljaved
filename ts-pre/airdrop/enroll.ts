import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, type Turbin3Prereq } from "../programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");
const github = Buffer.from("ofcljaved", "utf8");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

const program: Program<Turbin3Prereq> = new Program(IDL, provider);

const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
const [_enrollment_key, _bump] = PublicKey.findProgramAddressSync(enrollment_seeds, program.programId);

(async () => {
  try {
    const txHash = await program.methods
      .submit(github)
      .accounts({
        signer: keypair.publicKey,
      })
      .signers([keypair])
      .rpc();

    console.log(`Success! Check out your Transaction here: https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }

})();

//https://explorer.solana.com/tx/jvTpzyNGG5wJZ3J7xRGL7bMPHkFWE9rqfXntpFYy9twktGzXmS3kXxzreLJzvD9uVYrr3zw5CJnsbEAKGDPEiCQ?cluster=devnet
