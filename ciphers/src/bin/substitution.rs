use ciphers::substitution::Substitution;

fn main() {
    let cipher = Substitution::new("KRYPTOS".to_string());

    let string = "FOUR TANGOS NORTHWEST CLEARED TO ENGAGE".to_string();

    let encoded = cipher.encode(string);

    println!(
        "Encoded: {}\nDecoded: {}",
        encoded.clone(),
        cipher.decode(encoded)
    );
}
