# Locket

**A minimal EVM account generator for testing and development.**

Locket generates Ethereum-compatible accounts (private keys, public keys, and addresses) using secure cryptographic standards. All generated keys are saved to a JSON file for easy access during development and testing.

---

##  **CRITICAL SECURITY WARNING** 

> **Locket Keys are from TESTING ONLY — DO NOT USE WITH REAL FUNDS**
>
> This tool writes **unencrypted private keys directly to disk** (`accounts.json`).
>
> **NEVER** use Locket-generated accounts for:
> - Real cryptocurrency transactions
> - Production environments
> - Storing actual funds
> - Mainnet operations
>
> **Intended for LOCAL TESTING ONLY.**
>
> If the user chooses to use these keys with real funds, the user does so **entirely at the user's own risk**. We **strongly recommend against it**.

---

## What Does Locket Do?

Locket generates valid Ethereum accounts by:

1. Creating a cryptographically secure private key (secp256k1)
2. Deriving the corresponding public key
3. Computing the Ethereum address from the public key hash
4. Saving all key material to `accounts.json` in a structured format

The user can mention the number of account `N`, they want to generate as a variable to the cargo run command. 
Locket will generate the JSON-formatted `N` evm accounts to the `accounts.json` file.

---

## Installation

### Prerequisites

- **Rust** (1.70 or later recommended)
  - Install from [https://rustup.rs/](https://rustup.rs/)

### Clone the Repository

```bash
git clone https://github.com/romanticNomad/Locket.git
cd Locket
```

---

## Usage

### Generate a New Account

Simply run (N must be a non-zero natural number):

```bash
cargo run -- accounts N
```
In case no variable `N` is provided, Locket will default to `N=1`. 

**What happens:**

- Locket generates a new EVM account
- The account is saved to `accounts.json` in the project directory
- If `accounts.json` doesn't exist, it will be created
- Each new account is indexed as `account1`, `account2`, `account3`, etc.

### Example Output

After running (say) `cargo run -- accounts 2`, the user will see:

```bash
Generated 2 account(s) in accounts.json
```

The `accounts.json` file will look like this:

```json
{
  "account1": {
    "pvt_key": "0x1a2b3c4d5e6f78...",
    "pub_key": "0x04a1b2c3d4e5f6...",
    "address": "0x742d35Cc6634C0..."
  },
  "account2": {
    "pvt_key": "0x9876543210fedc...",
    "pub_key": "0x04f6e5d4c3b2a1...",
    "address": "0x5aAeb6053F3E94..."
  }
}
```

---

## Output Format

Each generated account contains:

| Field      | Description                                    | Example                                                              |
| ---------- | ---------------------------------------------- | -------------------------------------------------------------------- |
| `pvt_key`  | 32-byte private key (hex with `0x` prefix)     | `0x1a2b3c...`                                                        |
| `pub_key`  | Uncompressed secp256k1 public key (65 bytes)   | `0x04a1b2c3...`                                                      |
| `address`  | Ethereum address (20 bytes)                    | `0x742d35Cc...`                         |

All values are **hex-encoded** and compatible with standard Ethereum tooling (web3.js, ethers.js, Hardhat, Foundry, etc.).

---

## Use Cases

Locket is ideal for:

**Local development** — Generate test accounts for smart contract development  
**CI/CD pipelines** — Create deterministic test fixtures  
**Testing frameworks** — Provide accounts for automated tests (Hardhat, Foundry, etc.)  
**Cross-language verification** — Export keys for use in JavaScript, Python, Go, etc.  
**Prototyping** — Quickly generate accounts for signing service development

---

## Security Best Practices

### **DO:**

- Use Locket only in isolated development environments
- Delete `accounts.json` when no longer needed
- Keep the repository and `accounts.json` out of version control (already in `.gitignore`)
- Use testnet funds only (Sepolia, Goerli, etc.)

### **DON'T:**

- Commit `accounts.json` to Git
- Use generated keys on mainnet
- Share `accounts.json` publicly
- Store real funds in Locket-generated accounts

---

## File Structure

```
locket/
├── src/
│   ├── main.rs          # Core key generation logic
│   └── serialize.rs     # JSON export structures
├── Cargo.toml           # Rust dependencies
├── LICENSE              # MIT License
├── .gitignore           # Excludes accounts.json from Git
└── accounts.json        # Generated accounts (created on first run)
```

---

## How It Works (Technical Overview)

Locket uses industry-standard cryptographic primitives:

- **Private Key**: Generated using OS-backed randomness (`OsRng`) via the `k256` crate
- **Public Key**: Derived using secp256k1 elliptic curve (uncompressed format: `0x04 || X || Y`)
- **Address**: Computed as the last 20 bytes of `keccak256(public_key[1..])`

All operations are deterministic and follow Ethereum's key derivation standards.

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

---
## Need Help?

- **Issues**: [GitHub Issues](https://github.com/romanticNomad/Locket/issues)
- **Questions**: Open a discussion in the repository

> **Happy Testing**
