#!/usr/bin/env python3
from pathlib import Path
import hashlib
from datetime import datetime, timezone


WORDLIST_FILE = Path("pqc_wordlist.txt")
STATS_FILE = Path("stats.txt")


def read_bytes(path: Path) -> bytes:
    if not path.exists():
        raise FileNotFoundError(f"File not found: {path}")

    return path.read_bytes()


def load_words(path: Path) -> list[str]:
    text = path.read_text(encoding="utf-8")

    words = [
        line.strip()
        for line in text.splitlines()
        if line.strip()
    ]

    return words


def digest(data: bytes, algorithm: str) -> str:
    h = hashlib.new(algorithm)
    h.update(data)
    return h.hexdigest()


def validate_words(words: list[str]) -> list[str]:
    errors = []

    if len(words) != 4096:
        errors.append(f"Word count is {len(words)}, expected 4096")

    if len(set(words)) != len(words):
        errors.append("Duplicate words found")

    if words != sorted(words):
        errors.append("Words are not sorted alphabetically")

    for i, word in enumerate(words):
        if word != word.lower():
            errors.append(f"Non-lowercase word at index {i}: {word}")

        if not word.isalpha():
            errors.append(f"Non-alpha word at index {i}: {word}")

    return errors


def main() -> None:
    data = read_bytes(WORDLIST_FILE)
    words = load_words(WORDLIST_FILE)
    errors = validate_words(words)

    sha256 = digest(data, "sha256")
    sha512 = digest(data, "sha512")
    sha3_256 = digest(data, "sha3_256")
    sha3_512 = digest(data, "sha3_512")

    status = "VALID" if not errors else "INVALID"

    content = f"""PQC English Wordlist Standard

Version: v1
File: {WORDLIST_FILE.name}
Words: {len(words)}
Bits per word: 12
Indexing: zero-based
Index range: 0-4095

Mnemonic entropy: 384 bits
Mnemonic checksum: 12 bits
Mnemonic length: 33 words

Signature standard: ML-DSA-65
KDF standard: Argon2id
Primary hash standard: SHA3-512

Validation status: {status}

SHA256:
{sha256}

SHA512:
{sha512}

SHA3-256:
{sha3_256}

SHA3-512:
{sha3_512}

Generated at UTC:
{datetime.now(timezone.utc).isoformat()}
"""

    if errors:
        content += "\nValidation errors:\n"
        for error in errors:
            content += f"- {error}\n"

    STATS_FILE.write_text(content, encoding="utf-8")

    print(f"Written: {STATS_FILE}")
    print(f"Status: {status}")
    print(f"SHA3-512: {sha3_512}")


if __name__ == "__main__":
    main()
