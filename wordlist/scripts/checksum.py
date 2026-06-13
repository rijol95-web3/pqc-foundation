#!/usr/bin/env python3
from pathlib import Path
import hashlib

WORDLIST = Path("pqc_wordlist_4096.txt")

def main():
    data = WORDLIST.read_bytes()

    sha256 = hashlib.sha256(data).hexdigest()
    sha3_512 = hashlib.sha3_512(data).hexdigest()

    print("PQC Wordlist Checksum")
    print("=====================")
    print(f"File: {WORDLIST}")
    print(f"Bytes: {len(data)}")
    print(f"SHA256:   {sha256}")
    print(f"SHA3-512: {sha3_512}")

if __name__ == "__main__":
    main()