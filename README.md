# Locket

**Locket** is a minimal, deterministic-friendly **EVM account generation tool** written in Rust.
It generates Ethereum-compatible key material using industry-standard cryptography and exports it in a structured, machine-readable JSON format.

Locket is intended for **development, testing, and prototyping** of EVM signing systems and account management workflows.

---

## Features (Current)

### 1. Secure EVM Pvt Key Generation

* Generates **secp256k1** pvt keys
* Uses OS-backed cryptographic randomness (`OsRng`)
* Produces valid EVM-compatible pvt keys (`1 ≤ k < secp256k1_n`)

### 2. Deterministic Public Key Derivation

* Derives the public key directly from the pvt key
* Uses **uncompressed SEC1 format**:

  ```
  0x04 || X || Y
  ```
* Fully compatible with Ethereum tooling

### 3. Ethereum Address Computation

* Computes Ethereum addresses using:

  * `keccak256(uncompressed_public_key[1..])`
  * Last 20 bytes of the hash
* Outputs standard **hex-encoded EVM addresses**

### 4. Structured JSON Output

Generated accounts are written to a JSON file in an **indexed object format**, suitable for fixtures and tests.

Example output:

```json
{
  "account1": {
    "pvt_key": "0x…",
    "pub_key": "0x04…",
    "address": "0x…"
  },
  "account2": {
    "pvt_key": "0x…",
    "pub_key": "0x04…",
    "address": "0x…"
  }
}
```

### 5. Append-Only Account Generation

* New accounts are **appended**, not overwritten
* Accounts are indexed as `account1`, `account2`, `account3`, …
* Uses ordered serialization to keep entries in **increasing order**

---

## Output Format

Each generated account includes:

| Field                     | Description                                           |
| ------------------------- | ----------------------------------------------------- |
| `pvt_key`                 | 32-byte EVM pvt key, hex-encoded with `0x` prefix |   | 
| `pub_key`                 | Uncompressed secp256k1 public key                     |   
| `address`                 | Ethereum address derived from the public key          | 

All values are encoded in **hexadecimal** and are **language-agnostic**.

---

## Intended Use

Locket is designed for:

* EVM signing service prototyping
* Test account and fixture generation
* Local development and CI pipelines
* Cross-language signature verification

---

## Security Notice

> **Locket writes raw pvt keys to disk.**

This tool is **NOT** intended for:

* production key management
* custody of real funds
* long-term storage of pvt keys

Use only in **development and testing environments**.
