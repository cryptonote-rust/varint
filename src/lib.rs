#![allow(non_snake_case)]

use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Write};
use std::mem::size_of;
use std::ops::{BitAnd, BitOrAssign, Shl};

pub trait VarInt {
  fn parse(value: u128) -> Self;
}

impl VarInt for u64 {
  fn parse(data: u128) -> u64 {
    return data as u64;
  }
}

impl VarInt for u32 {
  fn parse(data: u128) -> u32 {
    return data as u32;
  }
}

impl VarInt for u16 {
  fn parse(data: u128) -> u16 {
    return data as u16;
  }
}

impl VarInt for u8 {
  fn parse(data: u128) -> u8 {
    return data as u8;
  }
}

// fn readByte(reader: Read, data: &mut [u8]) -> std::io::Result<()> {
//   reader.read_exact(data);
// }

// fn writeByte(writer: Write, data: &mut [u8]) -> std::io::Result<()> {
//   writer.write(data);
// }

pub fn read<T: VarInt, R: Read>(mut reader: R) -> T
where
  T: From<u8> + BitAnd + BitOrAssign + Shl + VarInt,
{
  let mut temp: u128 = 0;
  for shift in (0..).step_by(7) {
    let mut piece = [0];
    reader.read_exact(&mut piece[..]).expect("Failed to read");
    if (shift >= size_of::<T>() * 8 - 7) && piece[0] >= 1 << (size_of::<T>() * 8 - shift) {
      panic!("Read Varint, value overflow")
    }
    temp |= ((piece[0] & 0x7f) as u128) << shift;
    if (piece[0] & 0x80) == 0 {
      if piece[0] == 0 && shift != 0 {
        panic!("Read Varint, invalid value representation");
      }
      break;
    }
  }
  return T::parse(temp);
}

// fn write<T, W: Write>(writer: BufWriter<W>, value: T) {
//   while value >= 0x80 {
//     writeByte(writer, (value as u8 | 0x80) as [u8]);
//     value >>= 7;
//   }
//   writeByte(writer, value as [u8]);
// }

#[cfg(test)]
mod tests {
  use super::read;
  use std::io::{BufWriter, Cursor};

  #[test]
  fn it_works() {
    let mut data = [0x01];
    assert_eq!(read::<u16, _>(&data[..]),  1);
    // data = [0xFF];
    // assert_eq!(read::<u16, _>(&data[..]), 127);
    // data = [0x0F, 0x01];
    // assert_eq!(read::<u32, _>(&data[..]), 128 + 15);

    // write(unsafe { BufWriter::new(cursor) }, 300 as u32);
  }
}
