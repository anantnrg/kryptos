use ciphers::substitution::Substitution;

fn main() {
    let cipher = Substitution::new("KRYPTOS".to_string());

    let string = "FOUR TANGOS NORTHWEST. CLEARED TO ENGAGE";

    let encoded = cipher.encode(string);

    println!(
        "Encoded: {}\nDecoded: {}",
        encoded.clone(),
        cipher.decode(encoded.as_str())
    );
}
