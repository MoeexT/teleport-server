use std::{os::raw::c_uchar, slice};

use deku::prelude::*;
use util::deku_custom::*;

pub mod clip;
pub mod util;

// u4
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "4",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian"
)]
pub enum MessageType {
    // #[deku(id = "0x00")]
    Control = 0,
    // #[deku(id = "0x01")]
    Clip = 1,
}

impl TryFrom<u8> for MessageType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::Control),
            1 => Ok(MessageType::Clip),
            _ => Err(()),
        }
    }
}

// u4
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "4",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian"
)]
pub enum EncryptType {
    // #[deku(id = "0x00")]
    None = 0,
    // #[deku(id = "0x01")]
    AES256 = 1,
    // #[deku(id = "0x02")]
    RSA = 2,
    // #[deku(id = "0x03")]
    DSA = 3,
}

impl TryFrom<u8> for EncryptType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EncryptType::None),
            1 => Ok(EncryptType::AES256),
            2 => Ok(EncryptType::RSA),
            3 => Ok(EncryptType::DSA),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Message {
    #[deku(update = "self.uid.as_bytes().len()")]
    l_uid: u8,
    #[deku(update = "self.token.as_bytes().len()")]
    l_tkn: u8,
    #[deku(update = "self.content.len()")]
    l_cnt: usize,

    #[deku(
        reader = "string_reader(deku::rest, *l_uid)",
        writer = "string_writer(deku::output, &self.uid)"
    )]
    pub uid: String,

    #[deku(
        reader = "string_reader(deku::rest, *l_tkn)",
        writer = "string_writer(deku::output, &self.token)"
    )]
    pub token: String,

    pub message_type: MessageType,
    pub encryption: EncryptType,

    #[deku(count = "l_cnt")]
    pub content: Vec<u8>,
}

impl Message {
    pub fn new(
        uid: String,
        token: String,
        m_type: MessageType,
        encrypt: EncryptType,
        content: Vec<u8>,
    ) -> Self {
        let mut msg = Self {
            l_uid: 0,
            l_tkn: 0,
            l_cnt: 0,
            uid,
            token,
            message_type: m_type,
            encryption: encrypt,
            content,
        };

        let _ = &msg.update();
        msg
    }
}

pub extern "C" fn get_message_bytes(
    mt: u8,
    ec: u8,
    lu: usize,
    lt: usize,
    lc: usize,
    uid: *const c_uchar,
    tkn: *const c_uchar,
    cnt: *const c_uchar,
) -> *const c_uchar {
    let uid = unsafe { slice::from_raw_parts(uid, lu) };
    let tkn = unsafe { slice::from_raw_parts(tkn, lt) };
    let cnt = unsafe { slice::from_raw_parts(cnt, lc) };

    let msg = Message::new(
        String::from_utf8(uid.to_vec()).unwrap(),
        String::from_utf8(tkn.to_vec()).unwrap(),
        MessageType::try_from(mt).unwrap(),
        EncryptType::try_from(ec).unwrap(),
        cnt.to_vec(),
    );

    msg.to_bytes().unwrap().as_ptr()
}
