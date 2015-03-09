#![feature(io, core)]

use std::io;
use std::io::prelude::*;

pub enum LittleEndian {}
pub enum BigEndian {}

pub trait Endianness : std::marker::MarkerTrait {
    fn int_to_target<T: std::num::Int>(val: T) -> T;
    fn int_from_target<T: std::num::Int>(val: T) -> T;
}

impl Endianness for LittleEndian {
    fn int_to_target<T: std::num::Int>(val: T) -> T {
        val.to_le()
    }
    fn int_from_target<T: std::num::Int>(val: T) -> T {
        <T as std::num::Int>::from_le(val)
    }
}

impl Endianness for BigEndian {
    fn int_to_target<T: std::num::Int>(val: T) -> T {
        val.to_be()
    }
    fn int_from_target<T: std::num::Int>(val: T) -> T {
        <T as std::num::Int>::from_be(val)
    }
}

/// Additional integer write methods for a io::Read
pub trait WritePodExt {
    /// Write a usize
    fn write_usize<T: Endianness>(&mut self, usize) -> io::Result<()>;
    /// Write a u64
    fn write_u64<T: Endianness>(&mut self, u64) -> io::Result<()>;
    /// Write a u32
    fn write_u32<T: Endianness>(&mut self, u32) -> io::Result<()>;
    /// Write a u16
    fn write_u16<T: Endianness>(&mut self, u16) -> io::Result<()>;
    /// Write a u8
    fn write_u8<T: Endianness>(&mut self, u8) -> io::Result<()>;
}

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
    fn write_usize<T: Endianness>(&mut self, val: usize) -> io::Result<()> {
        let mut buf = [0u8; ::std::usize::BYTES as usize];
        let tval = <T as Endianness>::int_to_target(val);
        val_to_buf!(tval, buf);
        self.write_all(&buf)
    }

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

    fn write_u8<T: Endianness>(&mut self, val: u8) -> io::Result<()> {
        self.write_all(&[val])
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use super::{LittleEndian, BigEndian};
    use super::WritePodExt;

    #[test]
    fn write_be() {
        let buf: &mut [u8] = &mut [0u8; 8];
        let mut writer = io::Cursor::new(buf);

        writer.set_position(0);
        writer.write_usize::<BigEndian>(0x01_23_45_67_89_ab_cd_ef_u64 as usize).unwrap();
        assert_eq!(&writer.get_ref()[0..::std::usize::BYTES as usize], &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef][0..::std::usize::BYTES as usize]);
        
        writer.set_position(0);
        writer.write_u64::<BigEndian>(0x01_23_45_67_89_ab_cd_ef).unwrap();
        assert_eq!(&writer.get_ref()[0..8], &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

        writer.set_position(0);
        writer.write_u32::<BigEndian>(0x01_23_45_67).unwrap();
        assert_eq!(&writer.get_ref()[0..4], &[0x01, 0x23, 0x45, 0x67]);

        writer.set_position(0);
        writer.write_u16::<BigEndian>(0x01_23).unwrap();
        assert_eq!(&writer.get_ref()[0..2], &[0x01, 0x23]);

        writer.set_position(0);
        writer.write_u8::<BigEndian>(0x01).unwrap();
        assert_eq!(&writer.get_ref()[0..1], &[0x01]);
    }

    #[test]
    fn write_le() {
        let buf: &mut [u8] = &mut [0u8; 8];
        let mut writer = io::Cursor::new(buf);

        writer.set_position(0);
        writer.write_usize::<LittleEndian>(0x01_23_45_67_89_ab_cd_ef_u64 as usize).unwrap();
        assert_eq!(&writer.get_ref()[0..::std::usize::BYTES as usize], &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01][0..::std::usize::BYTES as usize]);
        
        writer.set_position(0);
        writer.write_u64::<LittleEndian>(0x01_23_45_67_89_ab_cd_ef).unwrap();
        assert_eq!(&writer.get_ref()[0..8], &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]);

        writer.set_position(0);
        writer.write_u32::<LittleEndian>(0x01_23_45_67).unwrap();
        assert_eq!(&writer.get_ref()[0..4], &[0x67, 0x45, 0x23, 0x01]);

        writer.set_position(0);
        writer.write_u16::<LittleEndian>(0x01_23).unwrap();
        assert_eq!(&writer.get_ref()[0..2], &[0x23, 0x01]);

        writer.set_position(0);
        writer.write_u8::<LittleEndian>(0x01).unwrap();
        assert_eq!(&writer.get_ref()[0..1], &[0x01]);
    }
}
