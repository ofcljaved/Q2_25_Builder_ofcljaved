import bs58 from "bs58";

function base58ToWallet(b58Address: string): Uint8Array {
  const wallet = bs58.decode(b58Address);
  console.log(`Uint8Array wallet: ${wallet}`);
  return wallet;
}

function walletToBase58(wallet: number[] | Uint8Array): string {
  const bytes = Uint8Array.from(wallet);
  const walletAddress = bs58.encode(bytes);
  console.log(`base58 wallet: ${walletAddress}`);
  return walletAddress;
}

export { base58ToWallet, walletToBase58 };
