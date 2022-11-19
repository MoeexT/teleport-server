use converter::{BytesConverter, NumberConverter};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub mod clip;
pub mod converter;
pub mod db;
pub mod util;

pub type Offsets = [u16; 4];

// u4
#[derive(Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum MessageType {
    Control = 0,
    Clip = 1,
}

impl NumberConverter for MessageType {
    fn to_number<T: From<Self>>(&self) -> T
    where
        Self: Sized,
    {
        match self {
            MessageType::Control => MessageType::Control.into(),
            MessageType::Clip => MessageType::Clip.into(),
        }
    }
}

// u4
#[derive(Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum EncryptType {
    None = 0,
    AES256 = 1,
    RSA = 2,
    DSA = 3,
}

impl NumberConverter for EncryptType {
    fn to_number<T: From<Self>>(&self) -> T
    where
        Self: Sized,
    {
        match self {
            EncryptType::None => EncryptType::None.into(),
            EncryptType::AES256 => EncryptType::AES256.into(),
            EncryptType::RSA => EncryptType::RSA.into(),
            EncryptType::DSA => EncryptType::DSA.into(),
        }
    }
}

pub struct Message {
    pub offsets: Offsets,
    pub uid: String,
    pub token: String,
    pub message_type: MessageType,
    pub encryption: EncryptType,
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
        let mut offsets: Offsets = [0; 4];

        offsets[0] = (8 + uid.as_bytes().len()) as u16;
        offsets[1] = offsets[0] + token.as_bytes().len() as u16;
        offsets[2] = offsets[1] + 1 + content.len() as u16;

        Self {
            offsets,
            uid,
            token,
            message_type: m_type,
            encryption: encrypt,
            content,
        }
    }
}

impl BytesConverter for Message {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(128);
        bytes.extend([0u8; 8]);

        let v_uid = self.uid.as_bytes();
        bytes.extend(v_uid);
        let len = bytes.len() as u16;
        bytes[0] = (len >> 8) as u8;
        bytes[1] = (len & 0xff) as u8;

        let v_token = self.token.as_bytes();
        bytes.extend(v_token);
        let len = bytes.len() as u16;
        bytes[2] = (len >> 8) as u8;
        bytes[3] = (len & 0xff) as u8;

        let v_mt = self.message_type.to_number::<u8>();
        let v_et = self.encryption.to_number::<u8>();
        let m_e = (v_mt << 4) + v_et & 0xf;
        bytes.extend([m_e]);
        // bytes[len as usize] = ;
        // TODO check
        bytes.extend(&self.content);
        let len = bytes.len() as u16;
        bytes[4] = (len >> 8) as u8;
        bytes[5] = (len & 0xff) as u8;

        bytes
    }

    fn from_bytes(bytes: Vec<u8>) -> Self {
        let mut offsets: Offsets = [0; 4];
        dbg!(&bytes[..8]);
        for i in 0..offsets.len() {
            // TODO 'attempt to shift left with overflow'
            // format!("{:b}", bytes [i]) = "0"
            dbg!(format!("{:b}, {:b}", bytes[i], bytes[i+1]));
            offsets[i] = u16::from(bytes[i<<1]) << 8 | u16::from(bytes[(i<<1) + 1]);
        }
        dbg!(bytes.len());
        dbg!(offsets);
        let uid = String::from_utf8_lossy(&bytes[4..offsets[0] as usize]).to_string();
        let token =
            String::from_utf8_lossy(&bytes[offsets[0] as usize..offsets[1] as usize]).to_string();

        let m_e = bytes[offsets[1] as usize];
        let message_type = MessageType::try_from(m_e >> 4).unwrap();
        let encryption = EncryptType::try_from(m_e & 0xf).unwrap();

        Self {
            offsets,
            uid,
            token,
            message_type,
            encryption,
            content: bytes[offsets[1] as usize + 1..].to_vec(),
        }
    }
}
