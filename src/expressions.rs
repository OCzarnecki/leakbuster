pub mod parser;
pub use crate::expressions::parser::parse_condition;

use crate::db;
use crate::expressions::parser::{Condition, TimeUnit};

use chrono::prelude::*;
use std::convert::TryInto;

struct EvalContext<'a, DB: db::Db, Z: TimeZone> {
    pub db: &'a DB,
    pub time: &'a DateTime<Z>,
    pub app_id: &'a str,
}

pub fn check_condition<T: db::Db>(
    condition: &Condition,
    db: &T,
    app_id: &str,
) -> Result<bool, db::Error> {
    let time = Local::now();
    let ctx = EvalContext {
        db,
        time: &time,
        app_id,
    };
    eval(&ctx, condition)
}

fn eval<DB: db::Db, Z: TimeZone>(
    ctx: &EvalContext<DB, Z>,
    c: &Condition,
) -> Result<bool, db::Error> {
    match c {
        Condition::And(c_and) => Ok(eval(ctx, &c_and.c1)? && eval(ctx, &c_and.c2)?),
        Condition::Or(c_or) => Ok(eval(ctx, &c_or.c1)? || eval(ctx, &c_or.c2)?),
        Condition::Not(c_not) => Ok(!eval(ctx, &c_not.c)?),
        Condition::Weekday => match ctx.time.weekday() {
            Weekday::Sat | Weekday::Sun => Ok(false),
            _ => Ok(true),
        },
        Condition::InWindow(c_in_window) => {
            // We'll never have negative time stamps in a real use case
            let ts: u64 = ctx.time.timestamp().try_into().unwrap();
            let usage = ctx
                .db
                .get_usage(ctx.app_id, ts - c_in_window.window_size.seconds, ts)?;
            Ok(usage < c_in_window.limit.seconds)
        }
        Condition::InCurrent(c_in_current) => {
            let start_of_window = match c_in_current.time_unit {
                TimeUnit::Second => ctx.time.clone(),
                TimeUnit::Minute => ctx
                    .time
                    .with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .unwrap(),
                TimeUnit::Hour => ctx
                    .time
                    .with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .unwrap(),
                TimeUnit::Day => ctx
                    .time
                    .with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_hour(0))
                    .unwrap(),
                TimeUnit::Week => ctx
                    .time
                    .with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_hour(0))
                    .unwrap(),
                TimeUnit::Month => ctx
                    .time
                    .with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_hour(0))
                    .and_then(|t| t.with_day(0))
                    .unwrap(),
                TimeUnit::Year => ctx
                    .time
                    .with_nanosecond(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_minute(0))
                    .and_then(|t| t.with_hour(0))
                    .and_then(|t| t.with_day(0))
                    .and_then(|t| t.with_month(0))
                    .unwrap(),
            };
            let ts_now: u64 = ctx.time.timestamp().try_into().unwrap();
            let ts_start: u64 = start_of_window.timestamp().try_into().unwrap();
            let usage = ctx.db.get_usage(ctx.app_id, ts_start, ts_now)?;
            Ok(usage < c_in_current.limit.seconds)
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
        check_str_condition(&db, &time, "app", "max 1 m in window 1 h").unwrap();
    }

    fn check_str_condition<Z: TimeZone, DB: db::Db>(
        db: &DB,
        time: &DateTime<Z>,
        app_id: &str,
        condition_str: &str,
    ) -> Result<bool, db::Error> {
        let condition = expressions::parse_condition(condition_str).unwrap();
        let ctx = expressions::EvalContext { db, time, app_id };
        expressions::eval(&ctx, &condition)
    }
}
