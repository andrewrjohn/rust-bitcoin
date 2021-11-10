use bs58;
use data_encoding::{HEXLOWER, HEXUPPER};
use hex::FromHex;
use num_bigint::{BigInt, Sign};
use rand::{thread_rng, CryptoRng, Rng};
use ring::digest::{Context, Digest, SHA256};
use ring::rand::SecureRandom;
use std::any::type_name;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use to_binary::BinaryString;

fn sha256_digest(data: &str) -> String {
    let mut context = Context::new(&SHA256);

    context.update(data.as_bytes());

    HEXUPPER.encode(context.finish().as_ref())
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

struct Seed {
    mnemonic: String,
    hex: String,
}

impl Seed {
    fn from_random() -> Self {
        fn generate_words() -> Option<String> {
            if let Ok(file) = File::open("./wordlist.txt") {
                let file = BufReader::new(file);
                let lines = file.lines();
                let mut all_words: Vec<String> = vec![];
                for line in lines {
                    let word = line.expect("Unable to read word");
                    all_words.push(word);
                }
                let mut seed: Vec<String> = vec![];
                for _ in 0..11 {
                    let mut rng = rand::thread_rng();
                    let random_number = rng.gen_range(0..2048);
                    let word = &all_words[random_number];
                    seed.push(word.to_string());
                }
                let seed = seed.join(" ");
                // println!("{:?}", seed);

                let binary = format!("{:b}", seed.replace(" ", ""));
                return Some(seed);
            } else {
                println!("Unable to open wordlist file");
                None
            }
        }

        match generate_words() {
            Some(words) => {
                let hex = sha256_digest(&words);
                Seed {
                    mnemonic: words,
                    hex,
                }
            }
            None => panic!("Cannot create seed"),
        }
    }

    // fn to_hex(&self) -> String {
    // }
}

fn main() {
    let rand_priv_key = PrivKey::from_random();
    let seed = Seed::from_random();

    // let priv_key = PrivKey::from_seed_phrase(seed.mnemonic);

    // println!("{}", rand_priv_key.decimal);
    // println!("{}", priv_key.decimal);
    // println!("{}", seed.hex);
    println!("{}", seed.mnemonic.split(" ").count());
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
