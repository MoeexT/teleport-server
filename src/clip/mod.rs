use deku::prelude::*;
// use chrono::{DateTime, Utc, NaiveDateTime};
use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ClipMessage {
    l_data: u32,
    pub data_type: ClipType, // 1B

    #[deku(count = "l_data")]
    pub data: Vec<u8>,

    #[deku(bytes = "8")]
    pub created_at: i64, // DateTime<Utc>, 8B
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "SmallInteger")]

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
    Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, DeriveActiveEnum, DekuRead, DekuWrite,
)]
#[sea_orm(rs_type = "u8", db_type = "SmallInteger")]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u16", db_type = "SmallInteger")]
pub enum ClipLimit {
    Ten = 10,
    Fifty = 50,
    Hundred = 100,
    ThreeHundred = 300,
    FiveHundred = 500,
    Thousand = 1000,
}
