use std::str;
use core::num::NonZeroUsize;

use redis::{Client, Commands};

use super::{entity::user::Model as UserModel};

pub struct SizedList<'a> {
    pub con: &'a Client,
}

pub fn connect() -> Client {
    redis::Client::open("redis://localhost/").unwrap()
}


/// RPUSH list value
fn push(con: &mut Client, key: &str, data: Vec<u8>) -> usize {
    con.rpush(key, data).unwrap()
    // redis::cmd("RPUSH").arg(key).arg(data).query(con).unwrap()
}

/// LPOP list
fn pop(con: &mut Client, key: &str, count: usize) -> Vec<Vec<u8>> {
    con.lpop(key, NonZeroUsize::new(count)).unwrap()

    // let result = redis::cmd("LPOP").arg(key).query::<String>(con);
    // match result {
    //     Ok(s) => Some(s),
    //     Err(_) => None,
    // }
}

fn _len(con: &mut Client, key: &str) -> usize {
    con.llen(key).unwrap()
    // redis::cmd("LLEN").arg(key).query(con).unwrap()
}

/// Push a string to the specified list, with checking list's length.
/// If list's length is grater than the capacity, older data will be deleted,
/// and the deleted strings will be returned.
/// If not overflow, it returns `None`.
fn range_push(con: &mut Client, key: &str, data: Vec<u8>, capacity: usize) -> Option<Vec<Vec<u8>>> {
    let size = push(con, key, data);

    if size > capacity {
        return Some(pop(con, key, size - capacity));
    }

    None
}

pub fn set(con: &mut Client, usr: &UserModel, data: String) {
    range_push(
        con,
        &usr.user_key,
        data.into_bytes(),
        usr.capacity as usize,
    )
    .unwrap();
}

/// get all values of user
pub fn get(con: &mut Client, usr: &UserModel) -> Vec<String> {
    let records: Vec<Vec<u8>> = con.lrange(&usr.user_key, 0, usr.capacity as isize).unwrap();
    records.into_iter()
        .map(|bytes| str::from_utf8(&bytes).unwrap().to_string())
        .collect()
}
