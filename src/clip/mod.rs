use chrono::{DateTime, Utc};
use deku::prelude::*;

use crate::util::deku_custom::{datetime_reader, datetime_writer};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ClipMessage {
    l_data: usize,
    pub data_type: ClipType, // 1B

    #[deku(count = "l_data")]
    pub data: Vec<u8>,

    #[deku(
        reader = "datetime_reader(deku::rest)",
        writer = "datetime_writer(deku::output, &self.created_at)"
    )]
    pub created_at: DateTime<Utc>, // 8B
}

impl ClipMessage {
    pub fn new(dp: ClipType, time_ms: DateTime<Utc>, data: Vec<u8>) -> Self {
        Self {
            l_data: data.len(),
            data_type: dp,
            data,
            created_at: time_ms,
        }
    }

    pub fn new_text(time: DateTime<Utc>, text: String) -> Self {
        Self {
            l_data: 0,
            data_type: ClipType::Text,
            data: text.into_bytes(),
            created_at: time,
        }
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite,
)]
#[deku(
    type = "u8",
    bytes = "1",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian"
)]
pub enum Ttl {
    Transient = 0b000, // delete after reading
    HalfHour = 0b001,  // 半小时
    OneHour = 0b010,   // 一小时
    HalfDay = 0b011,   // 12小时
    OneDay = 0b100,    // 一天
    OneWeek = 0b101,   // 一周
    OneMonth = 0b110,  // 一个月
    Permanent = 0b111, // 永久
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite,
)]
#[deku(
    type = "u8",
    bytes = "1",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian"
)]
pub enum ClipType {
    Text = 0b001,      // rich text
    Image = 0b010,     // image
    Stream = 0b100,    // byte stream, used for large file
    All = 0b1111_1111, // all data types
}

impl TryFrom<u8> for ClipType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ClipType::Text),
            2 => Ok(ClipType::Image),
            4 => Ok(ClipType::Stream),
            0xff => Ok(ClipType::All),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClipLimit {
    Ten = 10,
    Fifty = 50,
    Hundred = 100,
    ThreeHundred = 300,
    FiveHundred = 500,
    Thousand = 1000,
}

pub fn get_clip_bytes() {}
