/// power of 2
pub fn only_one_one(mut n: u8) -> bool {
    loop {
        if n == 0 {
            return false;
        }
        if n & 1 == 1 {
            return n >> 1 == 0;
        }
        n >>= 1;
    }
}
