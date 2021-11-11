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
                let mut bits = String::new();

                for _ in 0..256 {
                    let mut rng = rand::thread_rng();
                    let bit = rng.gen_range(0..=1);

                    bits.push_str(&bit.to_string());
                }

                println!("{}", bits);

                // Checksum
                let hex = sha256_digest(&bits);
                let first = hex.chars().nth(0).unwrap();
                let second = hex.chars().nth(1).unwrap();

                let decimal_map: HashMap<char, i32> = HashMap::from([
                    ('0', 0),
                    ('1', 1),
                    ('2', 2),
                    ('3', 3),
                    ('4', 4),
                    ('5', 5),
                    ('6', 6),
                    ('7', 7),
                    ('8', 8),
                    ('9', 9),
                    ('A', 10),
                    ('B', 11),
                    ('C', 12),
                    ('D', 13),
                    ('E', 14),
                    ('F', 15),
                ]);

                if let Some(dec) = decimal_map.get(&first) {
                    let binary = format!("{:0>4}", format!("{:b}", dec));
                    bits.push_str(&binary);
                }

                if let Some(dec) = decimal_map.get(&second) {
                    let binary = format!("{:0>4}", format!("{:b}", dec));

                    bits.push_str(&binary);
                }

                println!("{}", bits.chars().count());

                let mut binary_groups: Vec<String> = vec![];

                // We want 24 groups of 11 digit binary strings
                for _ in 0..24 {
                    let group = &bits[0..11];
                    binary_groups.push(group.to_string());
                    bits.replace_range(0..11, "");
                }

                let mut decimal_groups: Vec<usize> = vec![];

                for binary in binary_groups {
                    let decimal = usize::from_str_radix(&binary, 2).unwrap();

                    decimal_groups.push(decimal);
                }
                println!("{:?}", decimal_groups);

                let mut words: Vec<String> = vec![];

                for decimal in decimal_groups {
                    words.push(all_words[decimal].to_string());
                }

                let words = words.join(" ");
                println!("{:?}", words);

                for _ in 0..11 {
                    let mut rng = rand::thread_rng();
                    let random_number = rng.gen_range(0..2048);
                    let word = &all_words[random_number];
                    seed.push(word.to_string());
                }
                let seed = seed.join(" ");
                // println!("{:?}", seed);

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
    // let rand_priv_key = PrivKey::from_random();
    let seed = Seed::from_random();

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
