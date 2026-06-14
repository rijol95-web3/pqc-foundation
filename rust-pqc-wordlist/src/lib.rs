use once_cell::sync::Lazy;
use sha3::{Digest, Sha3_512};
use std::collections::{HashMap, HashSet};

pub const WORD_COUNT: usize = 4096;
pub const BITS_PER_WORD: usize = 12;

pub const MIN_INDEX: usize = 0;
pub const MAX_INDEX: usize = 4095;

pub const MNEMONIC_ENTROPY_BITS: usize = 384;
pub const MNEMONIC_CHECKSUM_BITS: usize = 12;
pub const MNEMONIC_TOTAL_BITS: usize = 396;
pub const MNEMONIC_WORDS: usize = 33;

pub const SIGNATURE_STANDARD: &str = "ML-DSA-65";
pub const KDF_STANDARD: &str = "Argon2id";
pub const PRIMARY_HASH: &str = "SHA3-512";

static WORDS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("pqc_wordlist.txt")
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
});

static INDEX_MAP: Lazy<HashMap<String, usize>> = Lazy::new(|| {
    WORDS
        .iter()
        .enumerate()
        .map(|(index, word)| (word.clone(), index))
        .collect()
});

pub fn words() -> &'static Vec<String> {
    &WORDS
}

pub fn word_at(index: usize) -> Option<&'static str> {
    WORDS.get(index).map(|word| word.as_str())
}

pub fn index_of(word: &str) -> Option<usize> {
    let normalized = word.trim().to_lowercase();
    INDEX_MAP.get(&normalized).copied()
}

pub fn contains(word: &str) -> bool {
    let normalized = word.trim().to_lowercase();
    INDEX_MAP.contains_key(&normalized)
}

fn canonical_bytes() -> Vec<u8> {
    format!("{}\n", WORDS.join("\n")).into_bytes()
}

pub fn sha3_512() -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(canonical_bytes());
    let result = hasher.finalize();

    hex::encode(result)
}

pub fn validate_wordlist() -> Result<(), String> {
    if WORDS.len() != WORD_COUNT {
        return Err(format!(
            "word count is {}, expected {}",
            WORDS.len(),
            WORD_COUNT
        ));
    }

    let unique: HashSet<&String> = WORDS.iter().collect();

    if unique.len() != WORD_COUNT {
        return Err("duplicate words found".to_string());
    }

    let mut sorted = WORDS.clone();
    sorted.sort();

    if *WORDS != sorted {
        return Err("wordlist is not alphabetically sorted".to_string());
    }

    for (index, word) in WORDS.iter().enumerate() {
        if *word != word.to_lowercase() {
            return Err(format!(
                "word at index {} is not lowercase: {}",
                index, word
            ));
        }

        if !word.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(format!(
                "word at index {} contains non alphabetic characters: {}",
                index, word
            ));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub name: &'static str,
    pub version: &'static str,
    pub words: usize,
    pub bits_per_word: usize,
    pub indexing: &'static str,
    pub min_index: usize,
    pub max_index: usize,
    pub mnemonic_entropy_bits: usize,
    pub mnemonic_checksum_bits: usize,
    pub mnemonic_total_bits: usize,
    pub mnemonic_words: usize,
    pub signature_standard: &'static str,
    pub kdf_standard: &'static str,
    pub primary_hash: &'static str,
    pub sha3_512: String,
}

pub fn metadata() -> Metadata {
    Metadata {
        name: "PQC English Wordlist Standard",
        version: "1.0.0",
        words: WORD_COUNT,
        bits_per_word: BITS_PER_WORD,
        indexing: "zero-based",
        min_index: MIN_INDEX,
        max_index: MAX_INDEX,
        mnemonic_entropy_bits: MNEMONIC_ENTROPY_BITS,
        mnemonic_checksum_bits: MNEMONIC_CHECKSUM_BITS,
        mnemonic_total_bits: MNEMONIC_TOTAL_BITS,
        mnemonic_words: MNEMONIC_WORDS,
        signature_standard: SIGNATURE_STANDARD,
        kdf_standard: KDF_STANDARD,
        primary_hash: PRIMARY_HASH,
        sha3_512: sha3_512(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(WORD_COUNT, 4096);
        assert_eq!(BITS_PER_WORD, 12);
        assert_eq!(MIN_INDEX, 0);
        assert_eq!(MAX_INDEX, 4095);
        assert_eq!(MNEMONIC_WORDS, 33);
    }

    #[test]
    fn test_word_count() {
        assert_eq!(words().len(), 4096);
    }

    #[test]
    fn test_validate_wordlist() {
        assert!(validate_wordlist().is_ok());
    }

    #[test]
    fn test_word_at() {
        assert!(word_at(0).is_some());
        assert!(word_at(4095).is_some());
        assert!(word_at(4096).is_none());
    }

    #[test]
    fn test_index_of() {
        let first = word_at(0).unwrap();
        let last = word_at(4095).unwrap();

        assert_eq!(index_of(first), Some(0));
        assert_eq!(index_of(last), Some(4095));
    }

    #[test]
    fn test_contains() {
        let first = word_at(0).unwrap();

        assert!(contains(first));
        assert!(!contains("notawordinpqcwordlist"));
    }

    #[test]
    fn test_sha3_512() {
        let hash = sha3_512();

        assert_eq!(hash.len(), 128);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_metadata() {
        let m = metadata();

        assert_eq!(m.words, 4096);
        assert_eq!(m.bits_per_word, 12);
        assert_eq!(m.mnemonic_words, 33);
        assert_eq!(m.signature_standard, "ML-DSA-65");
        assert_eq!(m.kdf_standard, "Argon2id");
        assert_eq!(m.primary_hash, "SHA3-512");
        assert_eq!(m.sha3_512.len(), 128);
    }
}