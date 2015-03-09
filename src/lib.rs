#![feature(io, core)]

use std::io;
use std::io::prelude::*;

pub enum LittleEndian {}
pub enum BigEndian {}

pub trait Endianness : std::marker::MarkerTrait {
    fn int_to_target<T: std::num::Int>(val: T) -> T;
    fn int_from_target<T: std::num::Int>(val: T) -> T;
}

/// Additional integer write methods for a io::Write
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
    /// Write a isize
    fn write_isize<T: Endianness>(&mut self, isize) -> io::Result<()>;
    /// Write a i64
    fn write_i64<T: Endianness>(&mut self, i64) -> io::Result<()>;
    /// Write a i32
    fn write_i32<T: Endianness>(&mut self, i32) -> io::Result<()>;
    /// Write a i16
    fn write_i16<T: Endianness>(&mut self, i16) -> io::Result<()>;
    /// Write a i8
    fn write_i8<T: Endianness>(&mut self, i8) -> io::Result<()>;
    /// Write a f32
    fn write_f32<T: Endianness>(&mut self, f32) -> io::Result<()>;
    /// Write a f64
    fn write_f64<T: Endianness>(&mut self, f64) -> io::Result<()>;
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

    fn write_isize<T: Endianness>(&mut self, val: isize) -> io::Result<()> {
        self.write_usize::<T>(val as usize)
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

    fn write_i8<T: Endianness>(&mut self, val: i8) -> io::Result<()> {
        self.write_u8::<T>(val as u8)
    }

    fn write_f32<T: Endianness>(&mut self, val: f32) -> io::Result<()> {
        let tval: u32 = unsafe { ::std::mem::transmute::<f32, u32>(val) };
        self.write_u32::<T>(tval)
    }

    fn write_f64<T: Endianness>(&mut self, val: f64) -> io::Result<()> {
        let tval: u64 = unsafe { ::std::mem::transmute::<f64, u64>(val) };
        self.write_u64::<T>(tval)
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
        assert_eq!(&writer.get_ref()[0..::std::usize::BYTES as usize], &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef][8-::std::usize::BYTES as usize..]);
        
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
        assert_eq!(&writer.get_ref()[0..::std::usize::BYTES as usize], &[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01][..::std::usize::BYTES as usize]);
        
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
}
