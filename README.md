# RustCrypt

---

## Overview

RustCrypt is a lightweight cryptographic utility for generating secure, entropy-based keys and passwords.
Built with Rust for safety, speed, and cryptographic security.

---

## Features

- High-entropy random generation
- Secure cryptographic hashing pipeline
- Minimal CLI interface
- Zero unnecessary dependencies

---

## Philosophy

Safe. Fast. Cryptographic.

---

## Usage

Generate a cryptographic output from high-entropy randomness.

```bash
### 32-byte output
rustcrypt generate 32

#### Example Output
5ee56aa15401224d21694ae4740e1e8e

### 64-byte output
rustcrypt generate 64

#### Example Output
b655bd72b3ab161417ea672f833c5eb53245f471033b9adee3e0e1934702a0e6

### version info
rustcrypt version

#### Example Output
rustcrypt 0.1.0
