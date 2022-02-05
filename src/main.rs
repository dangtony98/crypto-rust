use std::fs;
mod encrypt;
mod decrypt;

fn main() {
    const SUB_TEXT_FILENAME: &str = "text.txt";
    const SUB_CIPHER_FILENAME: &str = "sub_cipher.txt";
    const SUB_CIPHER_KEY: u8 = 6;
    let msg = fs::read_to_string(SUB_TEXT_FILENAME).unwrap();

    // Test substitution cipher method
    encrypt::sub_cipher(&SUB_TEXT_FILENAME, &SUB_CIPHER_FILENAME, &SUB_CIPHER_KEY);
    // assert!(msg == decrypt::sub_decipher(&SUB_CIPHER_FILENAME, &SUB_CIPHER_KEY));

    // encode string as bytes

    // TODO: Test one-time pad method
    // TODO: Test RC4 stream cipher - key scheduling, key stream generator
    // TODO: text to binary, binary to text
}