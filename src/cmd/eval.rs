use std::path::Path;
use crate::db;
use crate::expressions;
use crate::expressions::{parse_condition, parser::Condition};

pub fn eval<P: AsRef<Path>>(db_path: P, app_id: &str, condition_str: &str) {
    let db = db::connect_sqlite(db_path).unwrap();
    let condition: Condition = parse_condition(condition_str).unwrap();
    match expressions::check_condition(&condition, &db, app_id) {
        Ok(b) => if b {
            std::process::exit(0)
        } else {
            std::process::exit(1)
        },
        Err(db_error) => {
            println!("Database read error while evaluating expression: {:?}", db_error);
            std::process::exit(3)
        }
    }
}
