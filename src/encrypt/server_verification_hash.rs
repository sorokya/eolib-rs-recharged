/// This hash function is how the game client checks that it's communicating with a genuine server
/// during connection initialization.
///
/// - The client sends an integer value to the server in the INIT_INIT client packet, where it is referred to as the `challenge`
/// - The server hashes the value and sends the hash back in the INIT_INIT server packet.
/// - The client hashes the value and compares it to the hash sent by the server.
/// - If the hashes don't match, the client drops the connection.
///
/// # Examples
///
/// ```
/// use eolib::encrypt::server_verification_hash;
///
/// let challenge = 123456;
/// assert_eq!(server_verification_hash(challenge), 300733);
/// ````
///
/// # Warning
/// Oversized challenges may result in negative hash values, which cannot be represented properly in the EO protocol.
pub fn server_verification_hash(x: i64) -> i32 {
    ((x + 1) % 2023
        + 1
        + 110 * (31072023 - (x + 1)) % (109 * ((x + 1) % 22 + 1)) * ((x + 1) % 2 + 1)
        + 11221) as i32
}
