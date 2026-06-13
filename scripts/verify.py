#!/usr/bin/env python3
from pathlib import Path
import re
import sys

WORDLIST = Path("pqc_wordlist_4096.txt")
EXPECTED_COUNT = 4096
MIN_LEN = 5
MAX_LEN = 10

WORD_RE = re.compile(r"^[a-z]+$")

def fail(msg):
    print(f"[FAIL] {msg}")
    sys.exit(1)

def main():
    if not WORDLIST.exists():
        fail("pqc_wordlist_4096.txt not found")

    text = WORDLIST.read_text(encoding="utf-8")
    lines = text.splitlines()

    if len(lines) != EXPECTED_COUNT:
        fail(f"Expected {EXPECTED_COUNT} words, found {len(lines)}")

    seen = set()

    for i, word in enumerate(lines, start=1):
        if word.strip() != word:
            fail(f"Line {i}: leading/trailing space: {word!r}")

        if not word:
            fail(f"Line {i}: empty line")

        if not WORD_RE.fullmatch(word):
            fail(f"Line {i}: invalid characters: {word!r}")

        if not (MIN_LEN <= len(word) <= MAX_LEN):
            fail(f"Line {i}: invalid length {len(word)}: {word!r}")

        if word in seen:
            fail(f"Line {i}: duplicate word: {word!r}")

        seen.add(word)

    if lines != sorted(lines):
        fail("pqc_wordlist_4096.txt is not alphabetically sorted")

    print("[OK] pqc_wordlist_4096.txt passed verification")
    print(f"Total words: {len(lines)}")
    print(f"Min length: {min(len(w) for w in lines)}")
    print(f"Max length: {max(len(w) for w in lines)}")

if __name__ == "__main__":
    main()