podio
=====

[![Build Status](https://travis-ci.org/mvdnes/podio.svg?branch=master)](https://travis-ci.org/mvdnes/podio)
[![Crates.io version](https://img.shields.io/crates/v/podio.svg)](https://crates.io/crates/podio)

[Documentation](http://mvdnes.github.io/rust-docs/podio/podio/index.html)

Implementation for reading and writing POD (plain old data) values in Rust. The name stands for POD I/O.

Keywords: byte, be, le, big-endian, little-endian

Usage
-----

Include the following code:

```toml
[dependencies]
podio = "0.1"
```

Example
-------

```rust
extern crate podio;

use podio::{ReadPodExt, BigEndian};

fn main() {
    let slice: &[u8] = &[0x10, 0x20, 0x30, 0x40];
    let mut reader = std::io::Cursor::new(slice);

    let value = reader.read_u32::<BigEndian>().unwrap();

    assert_eq!(value, 0x10203040);
}
```
