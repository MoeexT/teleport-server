
// pub trait BytesConverter {
//     fn to_bytes(&self) -> Vec<u8>;
//     fn from_bytes(bytes: Vec<u8>) -> Self;
// }

// pub trait NumberConverter {
//     fn to_number<T: From<Self>>(&self) -> T
//     where
//         Self: Sized;

//     // fn from_number() -> Self;
// }

// impl BytesConverter for i64 {
//     fn to_bytes(&self) -> Vec<u8> {
//         let mut bytes = vec![0; 8];
//         let mut this = *self;

//         for i in 7..=0 {
//             bytes[i] = i as u8;
//             this >>= 8;
//         }
//         dbg!(this);
//         bytes
//     }

//     fn from_bytes(bytes: Vec<u8>) -> Self {
//         let mut this: Self = 0;

//         for i in 0..=7 {
//             this += bytes[i] as i64;
//             this <<= 8
//         }

//         this
//     }
// }
