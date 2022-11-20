use std::collections::HashSet;

use chrono::{Utc, DateTime};
use deku::{
    prelude::*, DekuRead, DekuWrite,
};

use teleport_server::{util::deku_custom::*, clip::ClipType};


#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
struct DekuTest {
    #[deku(update = "self.string.as_bytes().len()")]
    l_str: usize,

    #[deku(update = "self.authorities.len()")]
    l_cat: u8,

    #[deku(
        reader = "string_reader(deku::rest, *l_str)",
        writer = "string_writer(deku::output, &self.string)"
    )]
    pub string: String,
    #[deku(
        reader = "datetime_reader(deku::rest)",
        writer = "datetime_writer(deku::output, &self.time)"
    )]
    pub time: DateTime<Utc>,

    #[deku(count = "l_cat")]
    pub authorities: HashSet<ClipType>,
}

#[test]
fn deku_convert() {
    let mut set = HashSet::new();
    set.insert(ClipType::Text);
    set.insert(ClipType::Image);
    let mut v = DekuTest {
        l_str: 0,
        l_cat: 0,
        string: String::from("Hello哈哈😀"),
        time: Utc::now(),
        authorities: set,
    };
    v.update().unwrap();
    dbg!(&v);
    dbg!(&v.string.as_bytes());
    let bytes = v.to_bytes().unwrap();

    dbg!(&bytes);
    // println!("primitive: {:0x?}", bytes);
    // let i_time = unsafe { transmute::<[u8; 8], i64>(bytes[..8].try_into().unwrap()) }.to_be();
    // dbg!(i_time);
    // let i_sec = i_time / 1000_000_000;
    // let i_ns = (i_time % 1000_000_000) as u32;
    // dbg!(i_ns);
    // let n_time = NaiveDateTime::from_timestamp(i_sec, i_ns);
    // dbg!(n_time);

    let bytes = bytes.as_ref();
    let nv = DekuTest::from_bytes((bytes, 0)).unwrap();
    dbg!(&nv);
    assert_eq!(v, nv.1);
}

