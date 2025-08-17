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

        let alphabet_vec = alphabet.chars().collect::<Vec<_>>();

        for (i, c) in alphabet_vec.iter().enumerate() {
            enc_map.insert(*c, sub_alphabets[i]);
            dec_map.insert(sub_alphabets[i], *c);
        }

        Substitution {
            key,
            enc_map,
            dec_map,
        }
    }

    pub fn encode(&self, string: String) -> String {
        let enc_map = &self.enc_map;
        let mut enc_string = String::new();

        for c in string.to_ascii_uppercase().chars() {
            if enc_map.contains_key(&c) {
                enc_string.push(*enc_map.get(&c).unwrap());
            } else {
                enc_string.push(c);
            }
        }

        enc_string
    }

    pub fn decode(&self, encoded: String) -> String {
        let dec_map = &self.dec_map;
        let mut dec_string = String::new();

        for c in encoded.to_ascii_uppercase().chars() {
            if dec_map.contains_key(&c) {
                dec_string.push(*dec_map.get(&c).unwrap());
            } else {
                dec_string.push(c);
            }
        }

        dec_string
    }
}
