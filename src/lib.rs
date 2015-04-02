//! Additional methods for Read and Write
//!
//! The additional methods implemented allow reading and writing integers and floats
//! in the specified endianness.
//!
//! # Usage
//!
//! Basically, you need to `use` the trait WritePodExt or ReadPodExt.
//!
//! # Examples
//!
//! ## Reading
//!
//! To read some value from a reader, import ReadPodExt and the needed endianness.
//!
//! ```
//! use podio::{ReadPodExt, BigEndian};
//!
//! let slice: &[u8] = &[0x10, 0x20, 0x30, 0x40];
//! let mut reader = std::io::Cursor::new(slice);
//!
//! let value = reader.read_u32::<BigEndian>().unwrap();
//!
//! assert_eq!(value, 0x10203040);
//! ```
//!
//! ## Writing
//!
//! For writing, you need to import the trait WritePodExt.
//!
//! ```
//! use podio::{WritePodExt, LittleEndian};
//!
//! let slice: &mut [u8] = &mut [0; 2];
//! let mut writer = std::io::Cursor::new(slice);
//!
//! writer.write_u16::<LittleEndian>(0x8802).unwrap();
//!
//! assert_eq!(writer.into_inner(), &[0x02, 0x88]);
//! ```
//!
//! ## Read exact
//!
//! One additional method, not really dealing with POD, is `read_exact`.
//!
//! ```
//! use podio::ReadPodExt;
//!
//! let slice: &[u8] = &[0, 1, 2, 3];
//! let mut reader = std::io::Cursor::new(slice);
//!
//! assert_eq!(reader.read_exact(1).unwrap(), [0]);
//! assert_eq!(reader.read_exact(2).unwrap(), [1,2]);
//! assert_eq!(reader.read_exact(0).unwrap(), []);
//! assert_eq!(reader.read_exact(1).unwrap(), [3]);
//! assert!(reader.read_exact(1).is_err());

#![warn(missing_docs)]
#![feature(io)]

use std::io;
use std::io::prelude::*;

/// Little endian. The number `0xABCD` is stored `[0xCD, 0xAB]`
pub enum LittleEndian {}
/// Big endian. The number `0xABCD` is stored `[0xAB, 0xCD]`
pub enum BigEndian {}

/// Trait implementing conversion methods for a specific endianness
pub trait Endianness : std::marker::PhantomFn<Self> {
    /// Converts a value from the platform type to the specified endianness
    fn int_to_target<T: EndianConvert>(val: T) -> T;
    /// Converts a value from the sepcified endianness to the platform type
    fn int_from_target<T: EndianConvert>(val: T) -> T;
}

/// Generic trait for endian conversions on integers
pub trait EndianConvert {
    /// Convert self to a big-endian value
    fn to_be(self) -> Self;
    /// Convert self to a little-endian value
    fn to_le(self) -> Self;
    /// Convert a big-endian value to the target endianness
    fn from_be(x: Self) -> Self;
    /// Convert a little-endian value to the target endiannes
    fn from_le(x: Self) -> Self;
}

/// Additional write methods for a io::Write
pub trait WritePodExt {
    /// Write a u64
    fn write_u64<T: Endianness>(&mut self, u64) -> io::Result<()>;
    /// Write a u32
    fn write_u32<T: Endianness>(&mut self, u32) -> io::Result<()>;
    /// Write a u16
    fn write_u16<T: Endianness>(&mut self, u16) -> io::Result<()>;
    /// Write a u8
    fn write_u8(&mut self, u8) -> io::Result<()>;
    /// Write a i64
    fn write_i64<T: Endianness>(&mut self, i64) -> io::Result<()>;
    /// Write a i32
    fn write_i32<T: Endianness>(&mut self, i32) -> io::Result<()>;
    /// Write a i16
    fn write_i16<T: Endianness>(&mut self, i16) -> io::Result<()>;
    /// Write a i8
    fn write_i8(&mut self, i8) -> io::Result<()>;
    /// Write a f32
    fn write_f32<T: Endianness>(&mut self, f32) -> io::Result<()>;
    /// Write a f64
    fn write_f64<T: Endianness>(&mut self, f64) -> io::Result<()>;
}

/// Additional read methods for a io::Read
pub trait ReadPodExt {
    /// Read a u64
    fn read_u64<T: Endianness>(&mut self) -> io::Result<u64>;
    /// Read a u32
    fn read_u32<T: Endianness>(&mut self) -> io::Result<u32>;
    /// Read a u16
    fn read_u16<T: Endianness>(&mut self) -> io::Result<u16>;
    /// Read a u8
    fn read_u8(&mut self) -> io::Result<u8>;
    /// Read a i64
    fn read_i64<T: Endianness>(&mut self) -> io::Result<i64>;
    /// Read a i32
    fn read_i32<T: Endianness>(&mut self) -> io::Result<i32>;
    /// Read a i16
    fn read_i16<T: Endianness>(&mut self) -> io::Result<i16>;
    /// Read a i8
    fn read_i8(&mut self) -> io::Result<i8>;
    /// Read a f32
    fn read_f32<T: Endianness>(&mut self) -> io::Result<f32>;
    /// Read a f64
    fn read_f64<T: Endianness>(&mut self) -> io::Result<f64>;
    /// Read a specific number of bytes
    fn read_exact(&mut self, usize) -> io::Result<Vec<u8>>;
}

impl Endianness for LittleEndian {
    fn int_to_target<T: EndianConvert>(val: T) -> T {
        val.to_le()
    }
    fn int_from_target<T: EndianConvert>(val: T) -> T {
        <T as EndianConvert>::from_le(val)
    }
}

impl Endianness for BigEndian {
    fn int_to_target<T: EndianConvert>(val: T) -> T {
        val.to_be()
    }
    fn int_from_target<T: EndianConvert>(val: T) -> T {
        <T as EndianConvert>::from_be(val)
    }
}

macro_rules! impl_platform_convert {
    ($T:ty) => {
        impl EndianConvert for $T {
            fn to_be(self) -> $T {
                self.to_be()
            }
            fn to_le(self) -> $T {
                self.to_le()
            }
            fn from_be(x: $T) -> $T {
                if cfg!(target_endian = "big") { x } else { x.swap_bytes() }
            }
            fn from_le(x: $T) -> $T {
                if cfg!(target_endian = "little") { x } else { x.swap_bytes() }
            }
        }
    };
}

impl_platform_convert!(u8);
impl_platform_convert!(u16);
impl_platform_convert!(u32);
impl_platform_convert!(u64);

macro_rules! val_to_buf {
    ($val:ident, $buf:ident) => {
        {
            for i in 0..$buf.len() {
                $buf[i] = ($val >> (i * 8)) as u8;
            }
        }
    };
}

impl<W: Write> WritePodExt for W {
    fn write_u64<T: Endianness>(&mut self, val: u64) -> io::Result<()> {
        let mut buf = [0u8; 8];
        let tval = <T as Endianness>::int_to_target(val);
        val_to_buf!(tval, buf);
        self.write_all(&buf)
    }

    fn write_u32<T: Endianness>(&mut self, val: u32) -> io::Result<()> {
        let mut buf = [0u8; 4];
        let tval = <T as Endianness>::int_to_target(val);
        val_to_buf!(tval, buf);
        self.write_all(&buf)
    }

    fn write_u16<T: Endianness>(&mut self, val: u16) -> io::Result<()> {
        let mut buf = [0u8; 2];
        let tval = <T as Endianness>::int_to_target(val);
        val_to_buf!(tval, buf);
        self.write_all(&buf)
    }

    fn write_u8(&mut self, val: u8) -> io::Result<()> {
        self.write_all(&[val])
    }

    fn write_i64<T: Endianness>(&mut self, val: i64) -> io::Result<()> {
        self.write_u64::<T>(val as u64)
    }

    fn write_i32<T: Endianness>(&mut self, val: i32) -> io::Result<()> {
        self.write_u32::<T>(val as u32)
    }

    fn write_i16<T: Endianness>(&mut self, val: i16) -> io::Result<()> {
        self.write_u16::<T>(val as u16)
    }

    fn write_i8(&mut self, val: i8) -> io::Result<()> {
        self.write_u8(val as u8)
    }

    fn write_f32<T: Endianness>(&mut self, val: f32) -> io::Result<()> {
        let tval: u32 = unsafe { std::mem::transmute::<f32, u32>(val) };
        self.write_u32::<T>(tval)
    }

    fn write_f64<T: Endianness>(&mut self, val: f64) -> io::Result<()> {
        let tval: u64 = unsafe { std::mem::transmute::<f64, u64>(val) };
        self.write_u64::<T>(tval)
    }
}

fn fill_buf<R: Read>(reader: &mut R, buf: &mut [u8]) -> io::Result<()> {
    let mut idx = 0;
    while idx != buf.len() {
        match reader.read(&mut buf[idx..]) {
            Ok(0) => return Err(io::Error::new(io::ErrorKind::Other, "Could not read enough bytes", None)),
            Ok(v) => { idx += v; }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

macro_rules! buf_to_val {
    ($buf:ident, $typ:ty) => {
        {
            let mut val: $typ = 0;
            for i in 0..$buf.len() {
                val |= ($buf[i] as $typ) << (i * 8);
            }
            val
        }
    };
}

impl<R: Read> ReadPodExt for R {
    fn read_u64<T: Endianness>(&mut self) -> io::Result<u64> {
        let buf = &mut [0u8; 8];
        try!(fill_buf(self, buf));
        let tval = buf_to_val!(buf, u64);
        Ok(<T as Endianness>::int_from_target(tval))
    }
    fn read_u32<T: Endianness>(&mut self) -> io::Result<u32> {
        let buf = &mut [0u8; 4];
        try!(fill_buf(self, buf));
        let tval = buf_to_val!(buf, u32);
        Ok(<T as Endianness>::int_from_target(tval))
    }
    fn read_u16<T: Endianness>(&mut self) -> io::Result<u16> {
        let buf = &mut [0u8; 2];
        try!(fill_buf(self, buf));
        let tval = buf_to_val!(buf, u16);
        Ok(<T as Endianness>::int_from_target(tval))
    }
    fn read_u8(&mut self) -> io::Result<u8> {
        let buf = &mut [0u8; 1];
        try!(fill_buf(self, buf));
        Ok(buf[0])
    }
    fn read_i64<T: Endianness>(&mut self) -> io::Result<i64> {
        self.read_u64::<T>().map(|v| v as i64)
    }
    fn read_i32<T: Endianness>(&mut self) -> io::Result<i32> {
        self.read_u32::<T>().map(|v| v as i32)
    }
    fn read_i16<T: Endianness>(&mut self) -> io::Result<i16> {
        self.read_u16::<T>().map(|v| v as i16)
    }
    fn read_i8(&mut self) -> io::Result<i8> {
        self.read_u8().map(|v| v as i8)
    }
    fn read_f64<T: Endianness>(&mut self) -> io::Result<f64> {
        self.read_u64::<T>().map(|v| unsafe { std::mem::transmute::<u64, f64>(v) })
    }
    fn read_f32<T: Endianness>(&mut self) -> io::Result<f32> {
        self.read_u32::<T>().map(|v| unsafe { std::mem::transmute::<u32, f32>(v) })
    }
    fn read_exact(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut res = vec![0; len];
        try!(fill_buf(self, &mut res));
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use super::{LittleEndian, BigEndian};
    use super::{ReadPodExt, WritePodExt};

    #[test]
    fn write_be() {
        let buf: &mut [u8] = &mut [0u8; 8];
        let mut writer = io::Cursor::new(buf);

        writer.set_position(0);
        writer.write_u64::<BigEndian>(0x01_23_45_67_89_ab_cd_ef).unwrap();
        assert_eq!(&writer.get_ref()[0..8], &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

        writer.set_position(0);
        writer.write_u32::<BigEndian>(0x01_23_45_67).unwrap();
        assert_eq!(&writer.get_ref()[0..4], &[0x01, 0x23, 0x45, 0x67]);

        writer.set_position(0);
        writer.write_u16::<BigEndian>(0x01_23).unwrap();
        assert_eq!(&writer.get_ref()[0..2], &[0x01, 0x23]);
    }

    #[test]
    fn write_le() {
        let buf: &mut [u8] = &mut [0u8; 8];
        let mut writer = io::Cursor::new(buf);

        writer.set_position(0);
        writer.write_u64::<LittleEndian>(0x01_23_45_67_89_ab_cd_ef).unwrap();
        assert_eq!(&writer.get_ref()[0..8], &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]);

        writer.set_position(0);
        writer.write_u32::<LittleEndian>(0x01_23_45_67).unwrap();
        assert_eq!(&writer.get_ref()[0..4], &[0x67, 0x45, 0x23, 0x01]);

        writer.set_position(0);
        writer.write_u16::<LittleEndian>(0x01_23).unwrap();
        assert_eq!(&writer.get_ref()[0..2], &[0x23, 0x01]);
    }

    #[test]
    fn write_octet() {
        let buf: &mut [u8] = &mut [0u8; 8];
        let mut writer = io::Cursor::new(buf);

        writer.set_position(0);
        writer.write_u8(0x01).unwrap();
        assert_eq!(&writer.get_ref()[0..1], &[0x01]);
    }

    #[test]
    fn write_float() {
        let buf: &mut [u8] = &mut [0u8; 8];
        let mut writer = io::Cursor::new(buf);

        writer.set_position(0);
        writer.write_f32::<LittleEndian>(10.12f32).unwrap();
        assert_eq!(&writer.get_ref()[0..4], &[0x85, 0xEB, 0x21, 0x41]);

        writer.set_position(0);
        writer.write_f32::<BigEndian>(10.12f32).unwrap();
        assert_eq!(&writer.get_ref()[0..4], &[0x41, 0x21, 0xEB, 0x85]);

        writer.set_position(0);
        writer.write_f64::<LittleEndian>(10.12f64).unwrap();
        assert_eq!(&writer.get_ref()[0..8], &[0x3D, 0x0A, 0xD7, 0xA3, 0x70, 0x3D, 0x24, 0x40]);

        writer.set_position(0);
        writer.write_f64::<BigEndian>(10.12f64).unwrap();
        assert_eq!(&writer.get_ref()[0..8], &[0x40, 0x24, 0x3D, 0x70, 0xA3, 0xD7, 0x0A, 0x3D]);
    }

    #[test]
    fn read_be() {
        let buf: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let mut reader = io::Cursor::new(buf);

        reader.set_position(0);
        assert_eq!(reader.read_u64::<BigEndian>().unwrap(), 0x0123456789abcdef);

        reader.set_position(0);
        assert_eq!(reader.read_u32::<BigEndian>().unwrap(), 0x01234567);

        reader.set_position(0);
        assert_eq!(reader.read_u16::<BigEndian>().unwrap(), 0x0123);
    }

    #[test]
    fn read_le() {
        let buf: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let mut reader = io::Cursor::new(buf);

        reader.set_position(0);
        assert_eq!(reader.read_u64::<LittleEndian>().unwrap(), 0xefcdab8967452301);

        reader.set_position(0);
        assert_eq!(reader.read_u32::<LittleEndian>().unwrap(), 0x67452301);

        reader.set_position(0);
        assert_eq!(reader.read_u16::<LittleEndian>().unwrap(), 0x2301);
    }

    #[test]
    fn read_octet() {
        let buf: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let mut reader = io::Cursor::new(buf);

        reader.set_position(0);
        assert_eq!(reader.read_u8().unwrap(), 0x01);
    }

    #[test]
    fn read_float() {
        let mut buf: &[u8] = &[0x41, 0x21, 0xEB, 0x85];
        assert_eq!(buf.read_f32::<BigEndian>().unwrap(), 10.12f32);

        let mut buf: &[u8] = &[0x85, 0xEB, 0x21, 0x41];
        assert_eq!(buf.read_f32::<LittleEndian>().unwrap(), 10.12f32);

        let mut buf: &[u8] = &[0x40, 0x24, 0x3D, 0x70, 0xA3, 0xD7, 0x0A, 0x3D];
        assert_eq!(buf.read_f64::<BigEndian>().unwrap(), 10.12f64);

        let mut buf: &[u8] = &[0x3D, 0x0A, 0xD7, 0xA3, 0x70, 0x3D, 0x24, 0x40];
        assert_eq!(buf.read_f64::<LittleEndian>().unwrap(), 10.12f64);
    }

    #[test]
    fn read_exact() {
        let mut buf: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(<&[u8] as ReadPodExt>::read_exact(&mut buf, 2).unwrap(), [1,2]);
        assert_eq!(<&[u8] as ReadPodExt>::read_exact(&mut buf, 1).unwrap(), [3]);
        assert_eq!(<&[u8] as ReadPodExt>::read_exact(&mut buf, 0).unwrap(), []);
        assert_eq!(<&[u8] as ReadPodExt>::read_exact(&mut buf, 5).unwrap(), [4,5,6,7,8]);
    }
}
