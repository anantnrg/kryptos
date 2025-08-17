use std::collections::HashMap;

pub struct Substitution {
    pub key: String,
    pub alphabet: Option<HashMap<char, char>>,
}
