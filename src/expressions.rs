pub mod parser;

use crate::db;
use crate::expressions::parser::{Condition, TimeUnit};

use chrono::prelude::*;
use std::convert::TryInto;

struct EvalContext<'a, DB: db::Db, Z: TimeZone> {
    pub db: &'a DB,
    pub time: &'a DateTime<Z>,
    pub app_id: &'a str
}

// TODO should instead take a condition, parsed on config init, not runtime
pub fn check_condition<T: db::Db>(condition: &str, db: &T, app_id: &str) -> Result<bool, String> {
    let time = Local::now();
    check_condition_at_time(condition, db, app_id, &time)
}

fn check_condition_at_time<T: db::Db, U: TimeZone>(
    condition: &str,
    db: &T,
    app_id: &str,
    time: &DateTime<U>
) -> Result<bool, String> {
    let c = parser::parse_condition(condition)?;
    Ok(eval(&c, &EvalContext { db, time, app_id }))
}

fn eval<DB: db::Db, Z: TimeZone>(c: &Condition, ctx: &EvalContext<DB, Z>)
    -> bool {
    println!("Checking condition: {:?}", c);
    match c {
        Condition::And(c_and) => eval(&c_and.c1, ctx)
            && eval(&c_and.c2, ctx),
        Condition::Or(c_or) => eval(&c_or.c1, ctx)
            || eval(&c_or.c2, ctx),
        Condition::Not(c_not) => !eval(&c_not.c, ctx),
        Condition::Weekday => match ctx.time.weekday() {
            Weekday::Sat | Weekday::Sun => false,
            _ => true
        },
        Condition::InWindow(c_in_window) => {
            // We'll never have negative time stamps in a real use case
            let ts: u64 = ctx.time.timestamp().try_into().unwrap(); 
            let usage = ctx.db.get_usage(ctx.app_id, ts - c_in_window.window_size.seconds, ts)
                .expect("Failed to read usage from database");
            usage > c_in_window.limit.seconds
        },
        Condition::InCurrent(c_in_current) => {
            let start_of_window = match c_in_current.time_unit {
                TimeUnit::Second => ctx.time.clone(),
                TimeUnit::Minute => ctx.time.with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .unwrap(),
                TimeUnit::Hour => ctx.time.with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .unwrap(),
                TimeUnit::Day => ctx.time.with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_hour(0))
                    .unwrap(),
                TimeUnit::Week => ctx.time.with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_hour(0))
                    .unwrap(),
                TimeUnit::Month => ctx.time.with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_hour(0))
                    .and_then(|t| t.with_day(0))
                    .unwrap(),
                TimeUnit::Year => ctx.time.with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_hour(0))
                    .and_then(|t| t.with_day(0))
                    .and_then(|t| t.with_month(0))
                    .unwrap(),
            };
            let ts_now: u64 = ctx.time.timestamp().try_into().unwrap();
            let ts_start: u64 = start_of_window.timestamp().try_into().unwrap();
            let usage = ctx.db.get_usage(ctx.app_id, ts_start, ts_now)
                .expect("Failed to read usage from database");
            usage > c_in_current.limit.seconds
        }
    }
}

mod test {
    use chrono::prelude::*;

    use crate::db;
    use crate::expressions;

    #[test]
    fn in_window() {
        let db = db::open_in_memory().unwrap();
        let time = Local::now();
        expressions::check_condition_at_time(
            "max 1 m in window 1 h", &db, "app", &time
        ).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn test_weekday() {
        let db = db::open_in_memory().unwrap();
        let sunday = Utc.ymd(2000, 01, 02).and_hms(12, 0, 0);
        assert!(!expressions::check_condition_at_time(
                "weekday", &db, "app", &sunday
        ).unwrap());
        let monday = Utc.ymd(2000, 01, 03).and_hms(12, 0, 0);
        assert!(expressions::check_condition_at_time(
                "weekday", &db, "app", &monday
        ).unwrap());
    }
}
