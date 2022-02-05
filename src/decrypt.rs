use std::fs;

// return the decrypted message
pub fn sub_decipher(cipher_filename: &str, key: &u8) -> String {
    let cipher_text = fs::read_to_string(cipher_filename).unwrap();
    let mut text_vec: Vec<char> = Vec::new();
    
    for i in cipher_text.chars() {
        // for each character in message

        // convert to ordinal
        let ordinal = i as u8;

        // subtract key from ordinal
        let ordinal = ordinal - key;

        // convert back to char
        let text_char = (ordinal % 255) as char;

        // add [cipher_char] to [cipherVec]
        text_vec.push(text_char);
        // println!("{}, {}, {}", i, ordinal, cipher_char);
    }

    let message = text_vec.iter().collect::<String>();
    message
}