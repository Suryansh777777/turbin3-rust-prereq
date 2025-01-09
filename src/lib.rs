#[cfg(test)]
mod tests {
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    };

    use std::io::{self, BufRead};
    const RPC_URL: &str = "https://api.devnet.solana.com";
    use std::str::FromStr;
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }
    //a new Solana wallet: 8E1jyG4kcxi3ZyQ1ZhB7nsAh1nA2QCmdy4F1Nx59RsbT
    #[test]
    fn airdop() {
        let keypair = read_keypair_file("src/dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => {
                println!("Oops, something went wrong: {}", e.to_string());
            }
        }
    }
    //https://explorer.solana.com/tx/5hqG8tdFw8i5WLotHu3ZMMhohxCWqPWPJTzutShLYi5SkewkzJyetsMyki8oKC3UfJkeNn8Ex8E8GEDTmtXhqsa6?cluster=devnet
    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("src/dev-wallet.json").expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("6DKBzE6PyUBNBnBfqqdjj9AmFgZ6GY8iaGeD8f5HYYod").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        // a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        // let transaction = Transaction::new_signed_with_payer(
        //     &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        //     Some(&keypair.pubkey()),
        //     &vec![&keypair],
        //     recent_blockhash,
        // );
        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
    //First transfer(0.1sol)
    //https://explorer.solana.com/tx/5NNhGJyZYD5rrjftqTYpKcTnBkKo97E5zupBKfz1uEkUmsHhxD8UfYAPqknKDoHHAvrRpLXQUh54jVprghwo9fFb?cluster=devnet

    //Full Transfer
    //https://explorer.solana.com/tx/3sHha2fotGhVrSDYKEDt3ToFLmQdSdHYvyAZ4xuodW7jTzitXc8GYX6A1d99aGXsbcJ1EHvRG9i2Lz6GeyD2oq9z?cluster=devnet

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin(); // Read the input from stdin
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:"); // Print the input
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet); // Pprint the wallet conversion
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>(); // Print the input
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string(); // Print the base58 conversion
        println!("{:?}", base58);
    }
}
