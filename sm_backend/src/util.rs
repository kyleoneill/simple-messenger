use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::time::SystemTime;

pub fn random_string(length: u8) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}

pub fn get_unix_time() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time) => time.as_millis() as i64,
        Err(__) => panic!("SystemTime before unix epoch")
    }
}
