# Install

```bash
cargo add pqc-wordlist
```

# Usage

```rust
use pqc_wordlist::{
    words,
    word_at,
    index_of,
    contains,
    validate_wordlist,
    sha3_512,
};

fn main() {
    println!("{}", words().len());
    println!("{}", word_at(0).unwrap());

    println!(
        "{}",
        index_of(word_at(0).unwrap()).unwrap()
    );

    println!("{}", contains("about"));

    validate_wordlist().unwrap();

    println!("{}", sha3_512());
}
```