use std::mem::transmute;

use log::error;

use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sea_orm::{DeriveActiveEnum, EnumIter};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{Offsets, converter::{NumberConverter, BytesConverter}};

// [token, data_type, data]

// const CREATE_START: usize = 8;
const TYPE_START: usize = 16;
const DATA_START: usize = 20;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipMessage {
    offsets: Offsets, // 8B
    created_at: DateTime<Utc>, // 8B
    clip_type: ClipType, // 1B
    // nop. nop, nop 3B
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

impl ClipMessage {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match serde_json::from_str::<Self>(s) {
            Ok(r) => Some(r),
            Err(err) => {
                error!("Deserialize RedisRecord failed: {}", err);
                None
            }
        }
    }
}

impl BytesConverter for ClipMessage {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(128);
        bytes.extend([0u8; 8]);

        let v_time: [u8; 8] = self.created_at.timestamp_millis().to_be_bytes();
        bytes.extend(v_time);

        let v_type = self.clip_type.to_number::<u8>();
        bytes.extend([v_type, 0, 0, 0]); // [clip_type, nop, nop, nop]
        let len = bytes.len() as u16;
        bytes[0] = (len >> 8) as u8;
        bytes[1] = (len & 0xff) as u8;
        // TODO check
        bytes.extend(&self.data);
        let len = bytes.len() as u16;
        bytes[2] = (len >> 8) as u8;
        bytes[3] = (len & 0xff) as u8;

        bytes
    }

    fn from_bytes(bytes: Vec<u8>) -> Self {
        let mut offsets: Offsets = [0; 4];

        // deserialize `offsets`
        for i in 0..offsets.len() {
            offsets[i] = ((bytes[i] as u16) << 8) + bytes[i+1] as u16;
        }

        // deserialize `created_at`
        let a_time: [u8; 8] = bytes[8..16].try_into().unwrap();
        let i_time = unsafe {transmute::<[u8; 8], i64>(a_time)}.to_be();
        let i_sec = i_time / 1000;
        let i_ns = (i_time % 1000) as u32;
        let n_time = NaiveDateTime::from_timestamp(i_sec, i_ns * 1000);

        // deserialize `clip_type`
        let clip_type = ClipType::try_from(bytes[TYPE_START]).unwrap();

        Self {
            offsets,
            created_at: DateTime::from_utc(n_time, Utc),
            clip_type,
            data: bytes[DATA_START..].to_vec(),
        }
    }
}

// 3bit
#[repr(u8)]
#[derive(
    Debug, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive, EnumIter, DeriveActiveEnum,
)]
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

impl NumberConverter for Ttl {
    fn to_number<T: From<Self>>(&self) -> T
    where
        Self: Sized,
    {
        match self {
            Ttl::Transient => Ttl::Transient.into(),
            Ttl::HalfHour => Ttl::HalfHour.into(),
            Ttl::OneHour => Ttl::OneHour.into(),
            Ttl::HalfDay => Ttl::HalfDay.into(),
            Ttl::OneDay => Ttl::OneDay.into(),
            Ttl::OneWeek => Ttl::OneWeek.into(),
            Ttl::OneMonth => Ttl::OneMonth.into(),
            Ttl::Permanent => Ttl::Permanent.into(),
        }
    }
}
// 3bit
#[repr(u8)]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    IntoPrimitive,
    TryFromPrimitive,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
)]
#[sea_orm(rs_type = "u8", db_type = "SmallInteger")]
pub enum ClipType {
    Text = 0b001,      // rich text
    Image = 0b010,     // image
    Stream = 0b100,    // byte stream, used for large file
    All = 0b1111_1111, // all data types
}

impl NumberConverter for ClipType {
    fn to_number<T: From<Self>>(&self) -> T {
        match self {
            ClipType::Text => ClipType::Text.into(),
            ClipType::Image => ClipType::Image.into(),
            ClipType::Stream => ClipType::Stream.into(),
            ClipType::All => ClipType::All.into(),
        }
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Hash, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u16", db_type = "SmallInteger")]
pub enum ClipLimit {
    Ten = 10,
    Fifty = 50,
    Hundred = 100,
    ThreeHundred = 300,
    FiveHundred = 500,
    Thousand = 1000,
}


