use super::{ALPHABET, remove_duplicate_chars};
use std::collections::HashMap;

#[derive(Debug)]
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

        sub_alphabets.extend(key.chars());

        for c in alphabet.chars() {
            if !key.contains(c) {
                sub_alphabets.push(c);
            }
        }

        let mut enc_map: HashMap<char, char> = HashMap::new();
        let mut dec_map: HashMap<char, char> = HashMap::new();

        for (i, c) in alphabet.chars().enumerate() {
            enc_map.insert(c, sub_alphabets[i]);
            dec_map.insert(sub_alphabets[i], c);
        }

        Substitution {
            key,
            enc_map,
            dec_map,
        }
    }

    pub fn encode(&self, string: &str) -> String {
        let mut enc_string = String::new();

        for c in string.chars().map(|c| c.to_ascii_uppercase()) {
            if let Some(&enc_c) = self.enc_map.get(&c) {
                enc_string.push(enc_c);
            } else {
                enc_string.push(c);
            }
        }

        enc_string
    }

    pub fn decode(&self, encoded: &str) -> String {
        let mut dec_string = String::new();

        for c in encoded.chars().map(|c| c.to_ascii_uppercase()) {
            if let Some(&dec_c) = self.dec_map.get(&c) {
                dec_string.push(dec_c);
            } else {
                dec_string.push(c);
            }
        }

        dec_string
    }
}
