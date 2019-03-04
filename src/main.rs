/// Cryptonote varint implement
///
/// ## Features
///   * read varint
///   * write varint
///
/// ## Usage
/// ```toml
/// [dependencies]
/// cryptonote-varint = "0.1"
/// ```
///
/// ## Usage
///
/// 1. Write
/// ```
///     let mut c = Cursor::new(Vec::new());
///     c.seek(SeekFrom::Start(0)).unwrap();
///     write::<u8, _>(&mut c, 1 as u8);
/// ```
/// 
/// 2. Read
/// ```
///     let data0 = [0x01];
///     read::<u8, _>(&data0[..]);
/// ```
///

pub mod lib;

fn main() {
}

