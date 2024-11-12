pub fn decrypt_string(encrypted_str: &str, key: i32) -> String {
    let encrypted_bytes = encrypted_str.as_bytes();
    let mut result: Vec<u8> = Vec::with_capacity(encrypted_str.len() / 2);

    for i in (0..encrypted_bytes.len()).step_by(2) {
        let first_char = encrypted_bytes[i] as i32 - 0x41;
        let second_char = encrypted_bytes[i + 1] as i32 - 0x41;

        let encrypted_value = first_char * 24 + second_char;
        let decrypted_value = ((encrypted_value - (i as i32 / 2 + 1) * key) & 0xFF) as u8;
        result.push(decrypted_value);
    }

    String::from_utf8_lossy(&result).to_string()
}
