use std::collections::HashMap;

pub struct Substitution {
    pub key: String,
    pub enc_map: HashMap<char, char>,
    pub dec_map: HashMap<char, char>,
}
