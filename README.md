# varint

[![](https://travis-ci.com/cryptonote-rust/varint.svg?branch=master)](https://travis-ci.com/cryptonote-rust/varint)
[![](https://img.shields.io/crates/v/cryptonote-varint.svg)](https://crates.io/crates/cryptonote-varint)
[![codecov](https://codecov.io/gh/cryptonote-rust/varint/branch/master/graph/badge.svg)](https://codecov.io/gh/cryptonote-rust/varint)

## Usage

1. Write
```
    let mut c = Cursor::new(Vec::new());
    c.seek(SeekFrom::Start(0)).unwrap();
    write::<u8, _>(&mut c, 1 as u8);
```

2. Read
```
    let data0 = [0x01];
    read::<u8, _>(&data0[..]);
```