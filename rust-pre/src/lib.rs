mod programs;

#[cfg(test)]
mod tests {
    use crate::programs::Turbin3_prereq::{CompleteArgs, TurbinePrereqProgram, UpdateArgs};
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
        transaction::Transaction,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet:{}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        let kp = read_keypair_file("dev-wallet.json").expect("Could not find dev-wallet.json");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&kp.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s);
            }
            Err(e) => println!("Oops! Error: {}", e),
        }
    }
    //https://explorer.solana.com/tx/2xLg49p2XUXDBTLxAk7mEUh4WGMCsZvzxodVEedTcsnoVfqzW4kbFAKmsoXPcFSPuWNSRLv6BQtLjNRZeJVqkjzx?cluster=devnet

    #[test]
    fn transfer_sol() {
        let client = RpcClient::new(RPC_URL);
        let kp = read_keypair_file("dev-wallet.json").expect("Could not find dev-wallet.json");
        let pubkey = kp.pubkey();

        let balance = client.get_balance(&pubkey).expect("Failed to get balance");

        let message_bytes = b"I verify my solana keypair";
        let sig = kp.sign_message(message_bytes);
        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature is valid"),
            false => println!("Signature is invalid"),
        }

        let to_pubkey = Pubkey::from_str("7vbtBJLj54N5j33vV6wnU2CzrNuk7a2CWBGVUAZSGrpK").unwrap();
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(&kp.pubkey(), &to_pubkey, balance)],
            Some(&kp.pubkey()),
            &recent_blockhash,
        );

        let fee = client
            .get_fee_for_message(&message)
            .expect("Failed to get fee");

        let tx = Transaction::new_signed_with_payer(
            &[transfer(&kp.pubkey(), &to_pubkey, balance - fee)],
            Some(&kp.pubkey()),
            &vec![&kp],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&tx)
            .expect("Failed to send transaction");
        println!("Success! Check out your TX here:");
        println!(
            "https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }
    //First: https://explorer.solana.com/tx/nE7m4n3tUQoocD5NYnzK5owBLgPRahiu5wUn7drDpemE4XMnTbug9xta1wJXWErRUnqcXFVoLEeqjgPuGkYEQKg?cluster=devnet
    //Full : https://explorer.solana.com/tx/tkeAbyR8jE2R7JzB8Wy3gmMzNva8YhQ5gwQkeUqUhUxBgibAig9HdipWBMiLVRgHgVryUnEwn2GhhLkxQXwye44?cluster=devnet

    #[test]
    fn enroll() {
        let client = RpcClient::new(RPC_URL);
        let signer =
            read_keypair_file("turbin3-wallet.json").expect("Could not find turbin3-wallet.json");

        let prereq = TurbinePrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        let args = CompleteArgs {
            github: b"ofcljaved".to_vec(),
        };

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let tx = TurbinePrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&tx)
            .expect("Failed to send transaction");
        println!("Success! Check out your TX here:");
        println!(
            "https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_wallet() {
        println!("Enter your base58 encoded private key:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Enter your wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("[")
            .trim_end_matches("]")
            .split(",")
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{}", base58);
    }
}
