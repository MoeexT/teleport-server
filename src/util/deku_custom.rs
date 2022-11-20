use chrono::{DateTime, NaiveDateTime, Utc};
use deku::{
    bitvec::{BitSlice, BitVec, Msb0},
    ctx::{ByteSize, Endian},
    DekuError, DekuRead, DekuWrite,
};
use num::Unsigned;

type DateTimeUtc = DateTime<Utc>;

pub fn datetime_reader(
    rest: &BitSlice<u8, Msb0>,
) -> Result<(&BitSlice<u8, Msb0>, DateTimeUtc), DekuError> {
    let (rest, v) = i64::read(rest, Endian::Big)?;
    Ok((
        rest,
        DateTimeUtc::from_utc(
            NaiveDateTime::from_timestamp(v / 1000_000_000, (v % 1000_000_000) as u32),
            Utc,
        ),
    ))
}

pub fn datetime_writer(
    output: &mut BitVec<u8, Msb0>,
    datetime: &DateTimeUtc,
) -> Result<(), DekuError> {
    datetime.timestamp_nanos().write(output, Endian::Big)
}

pub fn string_reader<T>(
    rest: &BitSlice<u8, Msb0>,
    len: T,
) -> Result<(&BitSlice<u8, Msb0>, String), DekuError> where T: Len {
    let len = len.to_usize();
    let mut bytes = vec![0; len];
    let mut rest = rest;

    for i in 0..len {
        let (rst, value) = u8::read(rest, ByteSize(1))?;
        rest = rst;
        bytes[i] = value;
    }

    Ok((rest, String::from_utf8(bytes).unwrap()))
}

pub fn string_writer(output: &mut BitVec<u8, Msb0>, s: &str) -> Result<(), DekuError> {
    s.as_bytes().write(output, Endian::Big)
}

pub trait Len: Unsigned {
    fn to_usize(self) -> usize;
}

macro_rules! unsigned_trait_impl {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            fn to_usize(self) -> usize {
                self as usize
            }
        }
    )*)
}

unsigned_trait_impl!(Len for u8 u16 u32 usize);


pub fn hashset_reader(){}
pub fn hashset_writer(){}
