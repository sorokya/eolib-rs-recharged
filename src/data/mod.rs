/// The maximum value of an EO char (1-byte encoded integer type)
pub const CHAR_MAX: i32 = 253;

/// The maximum value of an EO short (2-byte encoded integer type)
pub const SHORT_MAX: i32 = CHAR_MAX * CHAR_MAX;

/// The maximum value of an EO three (3-byte encoded integer type)
pub const THREE_MAX: i32 = CHAR_MAX * CHAR_MAX * CHAR_MAX;

/// The maximum value of an EO int (4-byte encoded integer type)
pub const INT_MAX: i64 = CHAR_MAX as i64 * CHAR_MAX as i64 * CHAR_MAX as i64 * CHAR_MAX as i64;

/// Returns an encoded byte array from `number`
///
/// EO uses a maximum of four bytes to represent a number
/// in a data stream.
///
/// The value is spread across the four bytes based on if the
/// value is greater than the defined maximum for each amount of bytes
/// see [CHAR_MAX], [SHORT_MAX], [THREE_MAX]
///
/// The four bytes are initialized with a value of 254.
/// This is used later in [decode_number] for translating to a 0 value.
///
/// Bytes 4, 3, and 2 are set as follows if `number` is greater than or equal to
/// the corresponding `MAX` constant.
///
/// `bytes[x] = (number / MAX_x) + 1`
///
/// the number is then set to be the remainder of the division as follows
///
/// `number %= MAX_x`
///
/// Byte 1 is simply the remaining `number` plus one.
///
/// `bytes[0] = number + 1`
///
/// # Examples
///
/// ## Number less than CHAR_MAX
/// ```
/// use eolib::data::encode_number;
///
/// let result = encode_number(42).unwrap();
/// assert_eq!(result, [43, 254, 254, 254]);
/// ```
/// since 42 is less than CHAR_MAX it is simply incremented by 1
/// and the remaining bytes are set to 254
///
/// ## Number less than SHORT_MAX
/// ```
/// use eolib::data::encode_number;
/// let result = encode_number(533).unwrap();
/// assert_eq!(result, [28, 3, 254, 254]);
/// ```
///
/// since 533 is grater than CHAR_MAX byte 2 is set to
///
/// `(533 / CHAR_MAX) + 1 // 3`
///
/// byte 1 is set to the the remainder + 1
///
/// `(533 % CHAR_MAX) + 1 // 28`
///
/// and the remaining bytes are set to 254
///
/// ## Number less than THREE_MAX
/// ```
/// use eolib::data::encode_number;
/// let result = encode_number(888888).unwrap();
/// assert_eq!(result, [100, 225, 14, 254]);
/// ```
///
/// since 888888 is grater than SHORT_MAX byte 3 is set to
///
/// `(888888 / SHORT_MAX) + 1 // 14`
///
/// byte 2 is set to
///
/// `((888888 % SHORT_MAX) / CHAR_MAX) + 1 // 225`
///
/// byte 1 is set to the the remainder + 1
///
/// `(888888 % SHORT_MAX % CHAR_MAX) + 1 // 100`
///
/// and the last byte is set to 254
///
/// ## Number less than MAX4
/// ```
/// use eolib::data::encode_number;
/// let result = encode_number(18994242).unwrap();
/// assert_eq!(result, [15, 189, 44, 2]);
/// ```
///
/// since 18994242 is grater than THREE_MAX byte 4 is set to
///
/// `(18994242 / THREE_MAX) + 1 // 2`
///
/// byte 3 is set to
///
/// `((18994242 % THREE_MAX) / SHORT_MAX) + 1 // 44`
///
/// byte 2 is set to
///
/// `((18994242 % THREE_MAX % SHORT_MAX) / CHAR_MAX) + 1 // 189`
///
/// byte 1 is set to the the remainder + 1
///
/// `(18994242 % THREE_MAX % SHORT_MAX % CHAR_MAX) + 1 // 15`
pub fn encode_number(number: i32) -> Result<[u8; 4], EoWriterError> {
    let mut bytes: [u8; 4] = [254, 254, 254, 254];

    // Unwrap negative i32 to positive i64
    let mut number = if number < 0 {
        number.abs() as i64 + i32::MAX as i64
    } else {
        number as i64
    };

    let original_number = number;

    if original_number >= INT_MAX {
        return Err(EoWriterError::InvalidIntValue(original_number));
    }

    if original_number >= THREE_MAX as i64 {
        bytes[3] = (number / THREE_MAX as i64) as u8 + 1;
        number %= THREE_MAX as i64;
    }

    if original_number >= SHORT_MAX as i64 {
        bytes[2] = (number / SHORT_MAX as i64) as u8 + 1;
        number %= SHORT_MAX as i64;
    }

    if original_number >= CHAR_MAX as i64 {
        bytes[1] = (number / CHAR_MAX as i64) as u8 + 1;
        number %= CHAR_MAX as i64;
    }

    bytes[0] = number as u8 + 1;

    Ok(bytes)
}

pub fn encode_number_64(number: i64) -> Result<[u8; 5], EoWriterError> {
    let mut bytes: [u8; 5] = [254, 254, 254, 254, 254];

    let original_number = number;
    let mut number = number;

    if original_number >= INT_MAX {
        bytes[4] = (number / INT_MAX as i64) as u8 + 1;
        number %= INT_MAX as i64;
    }

    if original_number >= THREE_MAX as i64 {
        bytes[3] = (number / THREE_MAX as i64) as u8 + 1;
        number %= THREE_MAX as i64;
    }

    if original_number >= SHORT_MAX as i64 {
        bytes[2] = (number / SHORT_MAX as i64) as u8 + 1;
        number %= SHORT_MAX as i64;
    }

    if original_number >= CHAR_MAX as i64 {
        bytes[1] = (number / CHAR_MAX as i64) as u8 + 1;
        number %= CHAR_MAX as i64;
    }

    bytes[0] = number as u8 + 1;

    Ok(bytes)
}

/// Returns a decoded number from an EO Byte array
///
/// EO uses a maximum of four bytes to represent a number
/// in a data stream.
///
/// You can provide any number of [u8]s in `bytes`
/// but only the first four are used.
///
/// If you provide less than four than the remaining bytes default to 254
///
/// The byte array is iterated over and any byte of 254 is changed to 1, and
/// each byte is decremented by 1.
///
/// The result is then calculated like so
///
/// `(b4 * THREE_MAX) + (b3 * SHORT_MAX) + (b2 * CHAR_MAX) + b1`
///
/// # Examples
/// ```
/// use eolib::data::decode_number;
/// let result = decode_number(&[43, 254, 254, 254]);
/// assert_eq!(result, 42);
/// ```
///
/// * bytes with `254` are swapped to `1`
/// `[43, 1, 1, 1]`
/// * bytes are decremented by 1
/// `[42, 0, 0, 0]`
/// * bytes are multiplied by MAX's and summed
/// `(0 * THREE_MAX) + (0 * SHORT_MAX) + (0 * CHAR_MAX) + 42 == 42`
///
pub fn decode_number(bytes: &[u8]) -> i32 {
    let mut data: [u8; 4] = [254, 254, 254, 254];
    for i in 0..4 {
        if bytes.len() > i && bytes[i] != 0 {
            data[i] = bytes[i];
        }
        if data[i] == 254 {
            data[i] = 1;
        }
        data[i] -= 1;
    }

    ((data[3] as i32).wrapping_mul(THREE_MAX))
        .wrapping_add((data[2] as i32).wrapping_mul(SHORT_MAX))
        .wrapping_add((data[1] as i32).wrapping_mul(CHAR_MAX))
        .wrapping_add(data[0] as i32)
}

pub fn decode_number_64(bytes: &[u8]) -> i64 {
    let mut data: [u8; 5] = [254, 254, 254, 254, 254];
    for i in 0..5 {
        if bytes.len() > i && bytes[i] != 0 {
            data[i] = bytes[i];
        }
        if data[i] == 254 {
            data[i] = 1;
        }
        data[i] -= 1;
    }

    ((data[4] as i64).wrapping_mul(INT_MAX))
        .wrapping_add((data[3] as i64).wrapping_mul(THREE_MAX as i64))
        .wrapping_add((data[2] as i64).wrapping_mul(SHORT_MAX as i64))
        .wrapping_add((data[1] as i64).wrapping_mul(CHAR_MAX as i64))
        .wrapping_add(data[0] as i64)
}

/// Decodes a string in place
///
/// This is used for map names and sign text in map files
///
/// # Examples
///
/// ```
/// use eolib::data::decode_string;
///
/// let mut buf = [0x69, 0x36, 0x5E, 0x49];
/// decode_string(&mut buf);
///
/// let name = String::from_utf8_lossy(&buf).to_string();
/// assert_eq!(name, "Void");
/// ````
pub fn decode_string(buf: &mut [u8]) {
    for (i, c) in buf.iter_mut().enumerate() {
        *c += if i % 2 == 0 { 2 } else { 1 }
    }
}

/// Encodes a string in place
///
/// This is used for map names and sign text in map files
///
/// # Examples
///
/// ```
/// use eolib::data::encode_string;
///
/// let mut buf = b"Void".to_vec();
/// encode_string(&mut buf);
///
/// assert_eq!(buf, [0x69, 0x36, 0x5E, 0x49]);
/// ````
pub fn encode_string(buf: &mut [u8]) {
    for (i, c) in buf.iter_mut().enumerate() {
        *c -= if i % 2 == 0 { 2 } else { 1 }
    }
}

mod eo_reader;
pub use eo_reader::{EoReader, EoReaderError};
mod eo_writer;
pub use eo_writer::{EoWriter, EoWriterError};
mod eo_serialize;
pub use eo_serialize::{EoSerialize, EoSerializeError};
