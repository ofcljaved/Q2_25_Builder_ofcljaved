import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Voting } from "../target/types/voting";
import { assert } from "chai";

describe("voting", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.voting as Program<Voting>;

  const poll_id = new anchor.BN(1);
  const poll = anchor.web3.PublicKey.findProgramAddressSync([
    Buffer.from("poll"),
    poll_id.toArrayLike(Buffer, "le", 8),
  ], program.programId)[0];
  const poll_desc = "Favourite IDE";
  const candidate_first = "Neovim";
  const candidate_second = "VsCode";
  it("Initialize Poll", async () => {
    await program.methods.initPoll(
      poll_id,
      "Favourite IDE",
      new anchor.BN(0),
      new anchor.BN(1759508293),
    ).rpc();
    const pollAccount = await program.account.poll.fetch(poll);
    console.log(pollAccount);
    assert.equal(pollAccount.pollId.toNumber(), poll_id.toNumber());
    assert.equal(pollAccount.description, poll_desc);
    assert.isTrue(pollAccount.startTime.toNumber() < pollAccount.endTime.toNumber())
  });

  it("Initialize Candidate", async () => {
    const c1 = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("candidate"),
      poll_id.toArrayLike(Buffer, "le", 8),
      Buffer.from(candidate_first)
    ], program.programId)[0];
    const c2 = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("candidate"),
      poll_id.toArrayLike(Buffer, "le", 8),
      Buffer.from(candidate_second)
    ], program.programId)[0];

    await program.methods.initCandidate(
      poll_id,
      candidate_first
    ).rpc();
    await program.methods.initCandidate(
      poll_id,
      candidate_second
    ).rpc();
    const pollAccount = await program.account.poll.fetch(poll);
    console.log(pollAccount);
    const c1Account = await program.account.candidate.fetch(c1)
    console.log(c1Account);
    const c2Account = await program.account.candidate.fetch(c2)
    console.log(c2Account);

    assert.equal(pollAccount.candidateAmount.toNumber(), 2);
    assert.equal(c1Account.name, candidate_first);
    assert.equal(c1Account.voteCount.toNumber(), 0);
    assert.equal(c2Account.name, candidate_second);
    assert.equal(c2Account.voteCount.toNumber(), 0);
  });

  it("Vote Candidate one", async () => {
    const c1 = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("candidate"),
      poll_id.toArrayLike(Buffer, "le", 8),
      Buffer.from(candidate_first)
    ], program.programId)[0];

    await program.methods.vote(
      poll_id,
      candidate_first
    ).rpc();
    const c1Account = await program.account.candidate.fetch(c1)
    console.log(c1Account);

    assert.equal(c1Account.name, candidate_first);
    assert.equal(c1Account.voteCount.toNumber(), 1);
  });

  it("Vote Candidate second", async () => {
    const c2 = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("candidate"),
      poll_id.toArrayLike(Buffer, "le", 8),
      Buffer.from(candidate_second)
    ], program.programId)[0];

    await program.methods.vote(
      poll_id,
      candidate_second
    ).rpc();
    const c2Account = await program.account.candidate.fetch(c2)
    console.log(c2Account);

    assert.equal(c2Account.name, candidate_second);
    assert.equal(c2Account.voteCount.toNumber(), 1);
  });
});
