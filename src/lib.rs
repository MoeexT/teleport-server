use log::error;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sea_orm::{DeriveActiveEnum, EnumIter};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub mod db;
pub mod thread_pool;
pub mod util;

pub trait EnumToNumber {
    fn convert_to_number<T: From<Self>>(&self) -> T
    where
        Self: Sized;
}

const LOW_SIZE: u8 = 8;

pub struct UserAuthority {
    pub ttl: Ttl,
    pub data_type: ClipType,
}

impl UserAuthority {
    pub fn from(pv: u16) -> Self {
        let ttl = (pv & 0b1111_1111) as u8;
        let data_type = ((pv & 0b1111_1111_0000_0000) >> LOW_SIZE) as u8;
        Self {
            ttl: Ttl::try_from_primitive(ttl).unwrap_or(Ttl::Transient),
            data_type: ClipType::try_from_primitive(data_type)
                .unwrap_or(ClipType::Stream),
        }
    }

    pub fn to_number(&self) -> u16 {
        ((self.data_type.convert_to_number::<u8>() as u16) << LOW_SIZE)
            + (self.ttl.convert_to_number::<u8>()) as u16
    }
}





// pub enum SimpleAuthority {
//     Peak = 0b1111_1111_1111_0111, // all authority
//     High = 0b111_0110,            //
//     Medium = 0b11_0100,           //
//     Low = 1,                      //
// }

// 3bit
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive, EnumIter, DeriveActiveEnum)]
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

impl EnumToNumber for Ttl {
    fn convert_to_number<T: From<Self>>(&self) -> T
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
#[derive(Debug, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "u8", db_type = "SmallInteger")]
pub enum ClipType {
    Text = 0b001,      // rich text
    Image = 0b010,     // image
    Stream = 0b100,    // byte stream, used for large file
    All = 0b1111_1111, // all data types
}

impl EnumToNumber for ClipType {
    fn convert_to_number<T: From<Self>>(&self) -> T {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipItem {
    // pub key: &'a str,
    pub data_type: ClipType,
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
    created_at: DateTime<Utc>,
}

impl ClipItem {
    fn new(data_type: ClipType, content: Vec<u8>) -> Self {
        ClipItem {
            data_type,
            content,
            created_at: Utc::now(),
        }
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_str(s: &str) -> Option<Self> {
        match serde_json::from_str::<Self>(s) {
            Ok(r) => Some(r),
            Err(err) => {
                error!("Deserialize RedisRecord failed: {}", err);
                None
            }
        }
    }
}

