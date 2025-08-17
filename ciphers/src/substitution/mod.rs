use super::{ALPHABET, remove_duplicate_chars};
use std::collections::HashMap;

pub struct Substitution {
    pub key: String,
    pub enc_map: HashMap<char, char>,
    pub dec_map: HashMap<char, char>,
}

impl Substitution {
    pub fn new(key: String) -> Self {
        let key = remove_duplicate_chars(key.as_str());
        let alphabet = ALPHABET.to_string();
        let mut sub_alphabets: Vec<char> = vec![];

        for c in key.chars() {
            sub_alphabets.push(c);
        }

        for c in alphabet.chars() {
            if !key.contains(c) {
                sub_alphabets.push(c);
            }
        }

        let mut enc_map: HashMap<char, char> = HashMap::new();
        let mut dec_map: HashMap<char, char> = HashMap::new();

        for (i, c) in alphabet.chars().enumerate() {
            enc_map.insert(c, sub_alphabets[i]);
        }

        for (i, c) in sub_alphabets.iter().enumerate() {
            dec_map.insert(*c, alphabet.chars().nth(i).unwrap());
        }

        Substitution {
            key,
            enc_map,
            dec_map,
        }
    }
}
