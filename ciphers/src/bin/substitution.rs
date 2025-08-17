use ciphers::substitution::Substitution;

fn main() {
    let cipher = Substitution::new("KRYPTOS".to_string());

    println!(
        "Key: {}\nEncode Map: {:?}\nDecode Map: {:?}",
        cipher.key, cipher.enc_map, cipher.dec_map
    );
}
