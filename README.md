# cRypto

A modern, Rust-based **one-way crypting (hashing) CLI tool** focused on security, flexibility, and multiple cryptographic backends.

cRypto is designed for **password hashing, license key generation, and secure fingerprint-style crypting**, not for reversible encryption.

---

## ✨ Features

* Multiple **one-way crypting algorithms**
* Multiple **output encoding formats**
* Random salt generation
* Clean CLI interface powered by `clap`
* Modular cryptographic backend design
* Security-first approach (no decryption by design)

---

## 🔐 Supported Crypting Methods

| Method        | Description                                   |
| ------------- | --------------------------------------------- |
| `ARGON2`      | Industry-standard password hashing algorithm  |
| `ASCONHASH`   | Lightweight cryptographic hash (Ascon family) |
| `ASCONXOF`    | Extendable-output hash (XOF)                  |
| `BALLOONHASH` | Memory-hard hashing algorithm                 |
| `SCRYPT`      | Memory-hard KDF                               |
| `CRYPTIT`     | Hardware-aware custom crypting method         |

> All methods are **one-way**. There is no recovery or decryption.

---

## 🔤 Supported Encoding Types

Encodings are applied **after crypting** to convert raw bytes into readable strings.

| Encoding | Description                  |
| -------- | ---------------------------- |
| `HEX`    | Hexadecimal encoding         |
| `B64`    | Base64 encoding              |
| `B58`    | Base58 encoding              |
| `B85`    | Base85 encoding              |
| `BIN`    | Binary string representation |

> ⚠ Encoding does **not** weaken cryptographic security.

---

## 🚀 Usage

After building the project, the executable name is:

```bash
cRypto
```

### Basic Example

```bash
cRypto --pass myPassword
```

### Specify Crypting Method

```bash
cRypto --enc-method ARGON2 --pass myPassword
```

### Specify Encoding Type

```bash
cRypto --encode-type B64 --pass myPassword
```

### Custom Salt Length

```bash
cRypto --salt-len 32 --pass myPassword
```

### Full Example

```bash
cRypto --enc-method ASCONXOF --encode-type HEX --salt-len 24 --pass secret123
```

On success, the tool prints the crypted & encoded output.

---

## 🧱 Project Structure

```
src/
├── main.rs            # CLI entry point
├── libs/
│   ├── builder.rs     # Core crypt + encode dispatcher
│   ├── argon2_lib.rs
│   ├── ascon_hash_lib.rs
│   ├── ascon_xof_lib.rs
│   ├── baloon_hash_lib.rs
│   ├── scrypt_lib.rs
│   ├── cRyptit.rs
│   ├── hex_lib.rs
│   ├── b58_lib.rs
│   ├── b64_lib.rs
│   ├── b85_lib.rs
│   ├── bin_lib.rs
│   └── errors.rs
```

The architecture is intentionally modular, making it easy to:

* Add new crypting algorithms
* Add new encoders
* Integrate cRypto as a library later

---

## 🛡️ Security Notes

* cRypto **does not support decryption** by design
* Intended for:

  * Password storage
  * License / token generation
  * Hardware-bound identifiers
* Always store the full output string (including salt-derived data)

---

## 📦 Requirements

* Rust (stable)
* Cargo

---

## 📜 License

MIT License

---

## ⚠ Disclaimer

This tool is intended for **security-conscious use cases**. Always validate cryptographic choices according to your threat model.

---

**cRypto** — *One-way by design.*
# Made By CSDC-K
