mod server_verification_hash;
pub use server_verification_hash::server_verification_hash;
mod generate_encryption_key;
pub use generate_encryption_key::generate_encryption_key;
mod encrypt_packet;
pub use encrypt_packet::encrypt_packet;
mod decrypt_packet;
pub use decrypt_packet::decrypt_packet;
mod encrypt_string;
pub use encrypt_string::encrypt_string;
mod decrypt_string;
pub use decrypt_string::decrypt_string;

pub(crate) fn valid_for_encryption(buf: &[u8]) -> bool {
    buf.len() > 2 && buf[0..=1] != [0xff, 0xff]
}

pub const LOGIN_DO2_ENCRYPTION_KEY: i32 = 120;
pub const ACCOUNT_DELETE_ENCRYPTION_KEY: i32 = 111;
pub const ACCOUNT_DO2_ENCRYPTION_KEY: i32 = 126;
pub const LOGIN_REQUEST_ENCRYPTION_KEY: i32 = 132;
pub const ACCOUNT_PRIVATE_ENCRYPTION_KEY: i32 = 127;
