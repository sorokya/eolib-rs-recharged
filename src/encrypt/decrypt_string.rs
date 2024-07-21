pub fn decrypt_string(encrypted_str: &str, key: i32) -> String {
    let buf: Vec<u8> = encrypted_str.as_bytes().to_vec();
    let mut result: Vec<u8> = Vec::with_capacity(encrypted_str.len() / 2);

    for i in 0..(buf.len() / 2) {
        let encrypted_value = (buf[2 * i] - 0x41) * 24 + (buf[2 * i + 1] - 0x41);
        let original_value = (encrypted_value as i32 - (i as i32 + 1) * key as i32).rem_euclid(256);
        result.push(original_value as u8);
    }

    String::from_utf8_lossy(&result).to_string()
}
