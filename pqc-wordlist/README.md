# PQC English Wordlist Standard

A deterministic 4096-word English mnemonic wordlist designed for the PQC (Post Quantum Crypto) ecosystem.

This repository contains the official reference wordlist used by the PQC mnemonic standard for entropy encoding, checksum validation, mnemonic generation, and deterministic wallet derivation.

---

## Overview

The PQC English Wordlist Standard defines a fixed set of 4096 unique English words.

Each word represents a 12-bit unsigned integer value.

```text
4096 words = 2^12
```

The wordlist is intended for:

* Mnemonic phrase generation
* Wallet backup and recovery
* Entropy encoding
* Seed derivation
* Deterministic wallet generation
* Post-quantum cryptographic systems

---

## Specification

| Property      | Value           |
| ------------- | --------------- |
| Language      | English         |
| Total Words   | 4096            |
| Sorting       | Alphabetical    |
| Indexing      | Zero-Based      |
| First Index   | 0               |
| Last Index    | 4095            |
| Bits Per Word | 12              |
| Character Set | Lowercase ASCII |
| Duplicates    | Not Allowed     |

---

## PQC Mnemonic Format

The PQC mnemonic format uses 33 words.

```text
33 words × 12 bits = 396 bits
```

Mnemonic structure:

```text
384-bit Entropy
+
12-bit Checksum
=
396-bit Mnemonic
```

---

## Checksum

Checksum generation:

```text
checksum = first_12_bits(
    SHA3-512(entropy)
)
```

Parameters:

```text
Entropy Size: 384 bits (48 bytes)
Checksum Size: 12 bits
Hash Function: SHA3-512
```

---

## Word Index Encoding

Each word corresponds to a 12-bit unsigned integer.

Example:

```text
Index 0     -> first word
Index 1     -> second word
...
Index 4095  -> last word
```

Binary representation:

```text
000000000000
...
111111111111
```

---

## Repository Structure

```text
wordlist/
├── wordlist.txt
├── stats.txt
└── scripts/
    ├── verify.py
    └── checksum.py
```

### wordlist.txt

Official PQC English wordlist.

### stats.txt

Statistical information and checksum references.

### verify.py

Validates:

* Word count
* Duplicate entries
* Word length rules
* Character rules
* Alphabetical ordering

### checksum.py

Generates repository checksum values.

---

## Design Goals

The wordlist is designed to provide:

* Deterministic encoding
* Human-readable backups
* Efficient binary conversion
* Long-term stability
* Implementation simplicity
* Post-quantum ecosystem compatibility

---

## Compatibility

This wordlist is not compatible with:

* BIP39
* Electrum
* Monero mnemonic formats

It is intended exclusively for the PQC mnemonic standard.

---

## License

MIT License

---

## Status

Reference Specification

Version: 1.0

