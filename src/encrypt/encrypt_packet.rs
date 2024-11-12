use super::valid_for_encryption;

/// Encrypts a packet.
///
/// Packets are encrypted in three steps:
/// 1. "dickwinding"
/// 2. Flipping
/// 3. Interleaving
///
/// ## Dickwinding
/// This was named by Sausage and first implemented in the EOProxy project.
/// There are two numbers sent from the server to the client on connect
/// between 6 and 12 that represent a "send packet swap multiple"
/// and a "receive packet swap multiple".
///
/// Any two bytes next to each other in the packet data that are
/// divisible by that number are swapped.
///
/// ## Flipping
/// Each byte of the packet has their most significant bits flipped
/// ```text
/// for i in 0..length {
///     bytes[i] ^= 0x80;
/// }
/// ```
///
/// ## Interleaving
/// Bytes are "woven" in to each-other e.g.
/// ```text
/// abcde -> aebdc
///   or
/// abcdef -> afbecd
/// ```
/// For more details see [Packet](https://eoserv.net/wiki/wiki?page=Packet)
///
///
/// # Examples
/// ```
/// use eolib::encrypt::encrypt_packet;
///
/// let mut buf = [21, 18, 145, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];
///
/// encrypt_packet(&mut buf, 6);
///
/// assert_eq!(buf, [149, 161, 146, 228, 17, 242, 200, 236, 229, 239, 236, 247, 236, 160, 239, 172]);
/// ```
pub fn encrypt_packet(buf: &mut [u8], key: i32) {
    if !valid_for_encryption(buf) {
        return;
    }

    for i in 1..=buf.len() {
        let mut val = buf[i - 1] as i32;

        val = (((val + 3) % 256) + key + i as i32) & 0xFF;

        buf[i - 1] = val as u8;
    }
}
