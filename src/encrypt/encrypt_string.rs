pub fn encrypt_string(str: &str, key: i32) -> String {
    let buf: Vec<u8> = str.as_bytes().to_vec();
    let mut result: Vec<u8> = Vec::with_capacity(str.len() * 2);

    for (i, &c) in buf.iter().enumerate() {
        let encrypted_value = ((c as i32 + (i + 1) as i32 * key as i32) % 256) as u8;
        result.push(0x41 + encrypted_value / 24);
        result.push(0x41 + encrypted_value % 24);
    }

    String::from_utf8_lossy(&result).to_string()
}
