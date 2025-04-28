import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Favourite } from "../target/types/favourite";
import { assert } from "chai";

describe("favourite", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const owner = provider.wallet.publicKey;
  const randomOwner = anchor.web3.Keypair.generate();
  const program = anchor.workspace.favourite as Program<Favourite>;
  const favouritePDA = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("favourite"), provider.publicKey.toBytes()],
    program.programId
  )[0];

  const favNumber = new anchor.BN(2);
  const favColor = 'red';
  const favHobbies = ["web", "web2", "web3", "web4"];

  it("Writes Favourite", async () => {
    const tx = await program.methods.setFavourite(
      favNumber,
      favColor,
      favHobbies
    )
      .accountsPartial({
        owner
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const data = await program.account.favourite.fetch(favouritePDA);

    assert.equal(data.number.toString(), favNumber.toString());
    assert.equal(data.color, favColor);
    assert.deepEqual(data.hobbies, favHobbies);
  });

  it("Update Favourite Hobbies", async () => {
    const newHobbies = ["dev1", "dev2", "dev3", "dev4"];
    const tx = await program.methods.setFavourite(
      favNumber,
      favColor,
      newHobbies,
    )
      .accountsPartial({ owner })
      .rpc();
    console.log("Your transaction signature", tx);
    const data = await program.account.favourite.fetch(favouritePDA);

    assert.equal(data.hobbies, newHobbies);
  });

  it("Rejects from random owner", async () => {
    const tx = await program.methods.setFavourite(
      favNumber,
      favColor,
      favHobbies,
    )
      .accountsPartial({ owner })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
