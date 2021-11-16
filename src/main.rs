use bs58;
use data_encoding::{HEXLOWER, HEXUPPER};
use hex::FromHex;
use num_bigint::{BigInt, Sign};
use rand::{thread_rng, CryptoRng, Rng};
// extern crate ring::digest::{Context, Digest, SHA256};
// use ring::rand::SecureRandom;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use sv::util::sha256d;
use sv::wallet::{load_wordlist, mnemonic_decode, mnemonic_encode, Wordlist};

fn sha256_digest(data: &str) -> String {
    let hash_hex = sha256d(data.as_bytes()).encode();

    // context.update(data.as_bytes());

    // HEXUPPER.encode(context.finish().as_ref())
    return hash_hex;
}

struct Utils {}

impl Utils {
    fn generate_words() -> Option<String> {
        let bits = generate_entropy();
        let bits: &[u8] = &bits;

        let mnemonic = mnemonic_encode(bits, &load_wordlist(Wordlist::English));
        let mnemonic_string = mnemonic.join(" ");

        return Some(mnemonic_string);
    }
}

struct PrivKey {
    decimal: BigInt,
    seed: String,
}

impl PrivKey {
    fn from_random() -> Self {
        let seed = Seed::from_random();
        let seed = seed.mnemonic;

        let hash = sha256_digest(&seed);
        let decimal = BigInt::parse_bytes(hash.as_bytes(), 16).unwrap();

        Self { decimal, seed }
    }

    fn from_seed_phrase(seed: String) -> Self {
        if seed.split(" ").count() != 12 {
            panic!("Seed phrase must be 12 words.")
        }

        let hash = sha256_digest(&seed);
        let decimal = BigInt::parse_bytes(hash.as_bytes(), 16).unwrap();

        Self { decimal, seed }
    }

    fn to_wif(&self) -> String {
        format!(
            "{}{}",
            "K",
            bs58::encode(&self.decimal.to_str_radix(16)).into_string()
        )
        .to_string()
    }
}

fn generate_entropy() -> Vec<u8> {
    let mut bits: Vec<u8> = vec![];

    for _ in 0..32 {
        let mut rng = rand::thread_rng();
        let bit = rng.gen_range(0..=128);

        bits.push(bit.clone());
    }

    return bits;
}

struct Seed {
    mnemonic: String,
    hex: String,
}

impl Seed {
    fn from_random() -> Self {
        match Utils::generate_words() {
            Some(words) => {
                let hex = hex::encode(&words);
                Seed {
                    mnemonic: words,
                    hex,
                }
            }
            None => panic!("Cannot create seed"),
        }
    }

    // fn from_mnemonic(mnemonic: String) -> Self {
    //     let words = men
    // }
}

fn main() {
    // let rand_priv_key = PrivKey::from_random();
    let seed = Seed::from_random();

    println!("{:?}", seed.mnemonic);
    // let priv_key = PrivKey::from_seed_phrase(seed.mnemonic);

    // println!("{}", rand_priv_key.decimal);
    // println!("{}", priv_key.decimal);
    // println!("{}", seed.hex);
    // println!("{}", seed.mnemonic.split(" ").count());
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn priv_key_from_seed_phrase() {
//         let seed = Seed::from_random();
//         let priv_key = PrivKey::from_seed_phrase(seed.mnemonic);

//         assert_eq!(seed.mnemonic, priv_key.seed.to_string());
//     }
// }
