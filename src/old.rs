// fn old_generate_words() {
//        // if let Ok(file) = File::open("./wordlist.txt") {
//             //     let file = BufReader::new(file);
//             //     let lines = file.lines();
//             //     let mut all_words: Vec<String> = vec![];
//             //     for line in lines {
//             //         let word = line.expect("Unable to read word");
//             //         all_words.push(word);
//             //     }
//             //     let mut seed: Vec<String> = vec![];
//             //     let bits = generate_random_bit_string();

//             //     println!("{}", bits);

//             //     // Checksum
//             //     let hex = sha256_digest(&bits);
//             //     let first = hex.chars().nth(0).unwrap();
//             //     let second = hex.chars().nth(1).unwrap();

//             //     let decimal_map: HashMap<char, i32> = HashMap::from([
//             //         ('0', 0),
//             //         ('1', 1),
//             //         ('2', 2),
//             //         ('3', 3),
//             //         ('4', 4),
//             //         ('5', 5),
//             //         ('6', 6),
//             //         ('7', 7),
//             //         ('8', 8),
//             //         ('9', 9),
//             //         ('A', 10),
//             //         ('B', 11),
//             //         ('C', 12),
//             //         ('D', 13),
//             //         ('E', 14),
//             //         ('F', 15),
//             //     ]);

//             //     if let Some(dec) = decimal_map.get(&first) {
//             //         let binary = format!("{:0>4}", format!("{:b}", dec));
//             //         bits.push_str(&binary);
//             //     }

//             //     if let Some(dec) = decimal_map.get(&second) {
//             //         let binary = format!("{:0>4}", format!("{:b}", dec));

//             //         bits.push_str(&binary);
//             //     }

//             //     println!("{}", bits.chars().count());

//             //     let mut binary_groups: Vec<String> = vec![];

//             //     // We want 24 groups of 11 digit binary strings
//             //     for _ in 0..24 {
//             //         let group = &bits[0..11];
//             //         binary_groups.push(group.to_string());
//             //         bits.replace_range(0..11, "");
//             //     }

//             //     let mut decimal_groups: Vec<usize> = vec![];

//             //     for binary in binary_groups {
//             //         let decimal = usize::from_str_radix(&binary, 2).unwrap();

//             //         decimal_groups.push(decimal);
//             //     }
//             //     println!("{:?}", decimal_groups);

//             //     let mut words: Vec<String> = vec![];

//             //     for decimal in decimal_groups {
//             //         words.push(all_words[decimal].to_string());
//             //     }

//             //     let words = words.join(" ");
//             //     println!("{:?}", words);

//             //     for _ in 0..11 {
//             //         let mut rng = rand::thread_rng();
//             //         let random_number = rng.gen_range(0..2048);
//             //         let word = &all_words[random_number];
//             //         seed.push(word.to_string());
//             //     }
//             //     let seed = seed.join(" ");
//             //     // println!("{:?}", seed);

//             //     return Some(seed);
//             // } else {
//             //     println!("Unable to open wordlist file");
//             //     None
//             // }
// }
