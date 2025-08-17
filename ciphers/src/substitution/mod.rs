use super::{ALPHABET, remove_duplicate_chars};
use std::collections::HashMap;

pub struct Substitution {
    pub key: String,
    pub enc_map: HashMap<char, char>,
    pub dec_map: HashMap<char, char>,
}

impl Substitution {
    pub fn new(key: String) {
        let key = remove_duplicate_chars(key.as_str());
        println!("{key}")
    }
}
