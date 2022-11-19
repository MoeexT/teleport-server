use deku::prelude::*;


pub mod clip;
pub mod converter;
pub mod db;
pub mod util;

// u4
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "4", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum MessageType {
    // #[deku(id = "0x00")]
    Control = 0,
    // #[deku(id = "0x01")]
    Clip = 1,
}

// u4
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "4", endian = "endian", ctx = "endian: deku::ctx::Endian")]
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Message {
    #[deku(update = "self.uid.len()")]
    l_uid: u8,
    #[deku(update = "self.token.len()")]
    l_tkn: u8,
    #[deku(update = "self.content.len()")]
    l_cnt: u32,
    #[deku(count = "l_uid")]
    pub uid: Vec<u8>,
    #[deku(count = "l_tkn")]
    pub token: Vec<u8>,
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
        let uid = uid.as_bytes().to_vec();
        let token = token.as_bytes().to_vec();
        Self {
            l_uid: uid.len() as u8,
            l_tkn: token.len() as u8,
            l_cnt: content.len() as u32,
            uid,
            token,
            message_type: m_type,
            encryption: encrypt,
            content,
        }
    }
}


