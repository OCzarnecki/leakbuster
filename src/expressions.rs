pub mod parser;

use crate::db;

use std::time::SystemTime;
use std::time;
//pub use crate::expressions::parser::parse_condition;

pub fn check_condition<T: db::Db>(condition: &str, db: &T) -> bool {
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Could not get timestamp!")
        .as_secs();
    check_condition_at_ts(condition, db, ts)
}

fn check_condition_at_ts<T: db::Db>(
    condition: &str,
    db: &T,
    timestamp: u64
) -> bool {
    return false;
}
