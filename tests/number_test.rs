use teleport_server::{util::numbers::only_one_one, clip::ClipType};


#[test]
fn u16_to_u8() {
    let u16: u16 = 0b1111_1111_1111_1111;
    println!("{}", u16 as u8);
    assert_eq!(u16 as u8, 255)
}


#[test]
fn check_ont() {

    for i in 0..=255 {
        if only_one_one(i) {
            println!("{}", i)
        }
    }

}


#[test]
fn print_enum() {
    println!("{:?}", ClipType::Text)
}


#[test]
fn converter_i64() {
    dbg!((-65 as i64).to_be_bytes());
    dbg!((65 as i64).to_be_bytes());
}


#[test]
fn endian() {
    let x = 0x0102u16;
    println!("{}", x as u8)
}
