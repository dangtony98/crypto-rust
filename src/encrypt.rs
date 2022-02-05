use std::fs;
use std::io::Write;

// Encrypts the given file using the given key using the substitution cipher method.
// https://en.wikipedia.org/wiki/Caesar_cipher
pub fn sub_cipher(text_filename: &str, cipher_filename: &str, key: &u8) -> String {
    let message = fs::read_to_string(text_filename).unwrap();
    let mut cipher_vec: Vec<char> = Vec::new();

    for i in message.as_bytes() {
        // for each byte in message

        // add key to ordinal
        let ordinal = i + key;

        // convert back to char
        let cipher_char = (ordinal % 255) as char;

        // add [cipher_char] to [cipherVec]
        cipher_vec.push(cipher_char);
    }

    // write [cipher_vec] to file
    let cipher_text = cipher_vec.iter().collect::<String>();
    fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(cipher_filename)
        .unwrap()
        .write_all(cipher_text.as_bytes())
        .unwrap();

    cipher_text
}