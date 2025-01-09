#[cfg(test)]
mod tests {
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
    };
    use std::io::{self, BufRead};

    const RPC_URL: &str = "https://api.devnet.solana.com";

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
    fn transfer_sol() {}

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
