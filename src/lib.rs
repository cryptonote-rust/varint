
//! Cryptonote varint implement
//!
//! ## Features
//!   * read varint
//!   * write varint
//!
//! ## Usage
//! ```toml
//! [dependencies]
//! cryptonote-varint = "0.1"
//! ```
//!
//! ## Usage
//!
//! 1. Write
//! ```
//!     let mut c = Cursor::new(Vec::new());
//!     c.seek(SeekFrom::Start(0)).unwrap();
//!     write::<u8, _>(&mut c, 1 as u8);
//! ```

//! 2. Read
//! ```
//!     let data0 = [0x01];
//!     read::<u8, _>(&data0[..]);
//! ```


#![allow(non_snake_case)]

use std::io::{Read, Write};
use std::ops::{BitAnd, BitOrAssign, Shl};

const MSB: u8 = 0x80;
const REST: u8 = 0x7F;
pub trait VarInt {
  fn parse(value: u128) -> Self;
  fn retrieve(value: Self) -> u128;
}

impl VarInt for u64 {
  fn parse(data: u128) -> u64 {
    return data as u64;
  }
  fn retrieve(value: u64) -> u128 {
    return value as u128;
  }
}

impl VarInt for u32 {
  fn parse(data: u128) -> u32 {
    return data as u32;
  }
  fn retrieve(value: u32) -> u128 {
    return value as u128;
  }
}

impl VarInt for u16 {
  fn parse(data: u128) -> u16 {
    return data as u16;
  }
  fn retrieve(value: u16) -> u128 {
    return value as u128;
  }
}

impl VarInt for u8 {
  fn parse(data: u128) -> u8 {
    return data as u8;
  }
  fn retrieve(value: u8) -> u128 {
    return value as u128;
  }
}

pub fn read<T: VarInt, R: Read>(mut reader: R) -> T
where
  T: From<u8> + BitAnd + BitOrAssign + Shl + VarInt,
{
  let mut temp: u128 = 0;
  for shift in (0..).step_by(7) {
    let mut piece = [0];
    reader.read_exact(&mut piece[..]).expect("Failed to read");
    temp |= ((piece[0] & REST) as u128) << shift;
    if (piece[0] & MSB) == 0 {
      if piece[0] == 0 && shift != 0 {
        panic!("Read Varint, invalid value representation");
      }
      break;
    }
  }
  return T::parse(temp);
}

pub fn write<T: VarInt, W: Write>(writer: &mut W, value: T)
where
  T: From<u8> + BitAnd + BitOrAssign + Shl + VarInt,
{
  let mut temp = T::retrieve(value);
  while temp >= 0x80 {
    print!("temp = {}\n", temp);

    let mut piece = [0];
    piece[0] = temp as u8 | MSB;
    writer.write(&piece).expect("Failed to write");
    temp >>= 7;
  }
  print!("temp = {}\n", temp);
  let mut piece = [0];
  piece[0] = temp as u8;
  writer.write(&piece).expect("Failed to write");
}

fn main () {}

#[cfg(test)]
mod tests {
  use super::{read, write, main};
  use std::io::{Cursor, Seek, SeekFrom};

  #[test]
  fn it_should_read() {
    let data0 = [0x01];
    assert_eq!(read::<u8, _>(&data0[..]), 1);
    let data = [0x01];
    assert_eq!(read::<u16, _>(&data[..]), 1);
    let data1 = [0xF, 0x00];
    println!("{}", read::<u32, _>(&data1[..]));
    assert_eq!(read::<u32, _>(&data1[..]), 15);
    let data2 = [0x80, 0x1E];
    assert_eq!(read::<u32, _>(&data2[..]), 0x0F00);

    let data3 = [0x80, 0x1E, 0x11, 0x11, 0x11, 0x11];
    println!("{}", read::<u64, _>(&data3[..]));
    assert_eq!(read::<u64, _>(&data3[..]), 3840);
  }

  #[test]
  fn it_should_write() {
    let mut c = Cursor::new(Vec::new());
    c.seek(SeekFrom::Start(0)).unwrap();
    write::<u8, _>(&mut c, 1 as u8);
    let mut c1 = Cursor::new(Vec::new());
    c1.seek(SeekFrom::Start(0)).unwrap();
    write::<u16, _>(&mut c1, 128 as u16);
    let mut c2 = Cursor::new(Vec::new());
    c2.seek(SeekFrom::Start(0)).unwrap();
    write::<u32, _>(&mut c2, 129 as u32);
    let mut c3 = Cursor::new(Vec::new());
    c3.seek(SeekFrom::Start(0)).unwrap();
    write::<u64, _>(&mut c3, 0xF1 as u64);
    let mut c4 = Cursor::new(Vec::new());
    c4.seek(SeekFrom::Start(0)).unwrap();
    write::<u64, _>(&mut c4, 300 as u64);
    let cArr = c.get_ref();
    assert_eq!(cArr[0], 1);

    let cArr1 = c1.get_ref();
    assert_eq!(cArr1[0], 128);
    assert_eq!(cArr1[1], 1);
    let cArr2 = c2.get_ref();
    assert_eq!(cArr2[0], 129);
    assert_eq!(cArr2[1], 1);

    let cArr3 = c3.get_ref();
    assert_eq!(cArr3[0], 241);
    assert_eq!(cArr3[1], 1);
    let cArr4 = c4.get_ref();
    assert_eq!(cArr4[0], 0xAC);
    assert_eq!(cArr4[1], 0x02);
    println!("c = {:?}\n", c.get_ref());
    println!("c1 = {:?}\n", c1.get_ref());
    println!("c2 = {:?}\n", c2.get_ref());
    println!("c3 = {:?}\n", c3.get_ref());
    println!("c4 = {:?}\n", c4.get_ref());
  }
  #[test]
  #[should_panic]
  fn it_should_panic_when_read() {
    main();
    let data = [0xFF];
    read::<u32, _>(&data[..]);
  }
}
