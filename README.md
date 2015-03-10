podio
=====

[![Build Status](https://travis-ci.org/mvdnes/podio.svg?branch=master)](https://travis-ci.org/mvdnes/podio)

[Documentation](http://mvdnes.github.io/podio/)

Implementation for reading and writing POD (plain old data) values in Rust. The name stands for POD I/O.

Usage
-----

Include the following code:

```toml
[dependencies]
podio = "*"
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
