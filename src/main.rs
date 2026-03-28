mod serialize;
use crate::serialize::{EvmKeyExport, generate_account};
use std::{collections::BTreeMap, env, fs};

// ===========================================================

/// Parses command line arguments for account count.
/// 
/// saves generated accounts to the JSON file `accounts.json`.
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut num_accounts = 1;

    if let Some(pos) = args.iter().position(|a| a == "accounts") {
        if let Some(val) = args.get(pos + 1) {
            if let Ok(n) = val.parse::<usize>() {
                num_accounts = n;
            }
        }
    }
    if num_accounts == 0 {
        panic!("Error: accounts must be a positive number");
    }

    let mut accounts: BTreeMap<String, EvmKeyExport> = BTreeMap::new();
    for i in 1..=num_accounts {
        let account = generate_account();
        let entry = format!("account{}", i);
        accounts.insert(entry, account);
    }
    let path = "accounts.json";
    let contents = serde_json::to_string_pretty(&accounts)?;

    fs::write(path, contents)?;
    println!("Generated {} account(s) in {}", num_accounts, path);
    Ok(())
}

// ===========================================================
