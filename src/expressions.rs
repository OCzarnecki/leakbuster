pub mod parser;
pub use crate::expressions::parser::parse_condition;

use crate::db;
use crate::db::Db;
use crate::expressions::parser::{Condition, ConditionWeekday, TimeUnit};

use chrono::prelude::*;
use std::convert::TryInto;

struct EvalContext<'a, Z: TimeZone> {
    pub db: &'a Db,
    pub time: &'a DateTime<Z>,
    pub app_id: &'a str,
}

pub fn check_condition(
    condition: &Condition,
    db: &Db,
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

fn eval<Z: TimeZone>(
    ctx: &EvalContext<Z>,
    c: &Condition,
) -> Result<bool, db::Error> {
    match c {
        Condition::And(c_and) => Ok(eval(ctx, &c_and.c1)? && eval(ctx, &c_and.c2)?),
        Condition::Or(c_or) => Ok(eval(ctx, &c_or.c1)? || eval(ctx, &c_or.c2)?),
        Condition::Not(c_not) => Ok(!eval(ctx, &c_not.c)?),
        Condition::Weekday(cwd) => match ctx.time.weekday() {
            Weekday::Mon => Ok(*cwd == ConditionWeekday::Mon),
            Weekday::Tue => Ok(*cwd == ConditionWeekday::Tue),
            Weekday::Wed => Ok(*cwd == ConditionWeekday::Wed),
            Weekday::Thu => Ok(*cwd == ConditionWeekday::Thu),
            Weekday::Fri => Ok(*cwd == ConditionWeekday::Fri),
            Weekday::Sat => Ok(*cwd == ConditionWeekday::Sat),
            Weekday::Sun => Ok(*cwd == ConditionWeekday::Sun) 
        },
        Condition::AtMostInSliding(c_in_window) => {
            // We'll never have negative time stamps in a real use case
            let ts: u64 = ctx.time.timestamp().try_into().unwrap();
            let usage = ctx
                .db
                .get_usage(ctx.app_id, ts - c_in_window.window_size.seconds, ts)?;
            Ok(usage < c_in_window.limit.seconds)
        }
        Condition::AtMostInThis(c_in_current) => {
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

#[allow(dead_code)]
mod test {
    use chrono::prelude::*;

    use crate::db;
    use crate::db::Db;
    use crate::expressions;

    #[test]
    fn no_expression_panics_on_empty_db() {
        let db = db::open_in_memory().unwrap();
        let time = Utc.ymd(2000, 3, 20).and_hms(12, 0, 0);
        let exprs = vec![
            "Mon",
            "Tue",
            "Wed",
            "Thu",
            "Fri",
            "Sat",
            "Sun",
            "Mon and Tue",
            "Wed or Thu",
            "not Fri",
            "atmost 5 m in sliding 1 h",
            "atmost 1 h in this week"
        ];
        for e in exprs {
            check_str_condition(&db, &time, "app", e).unwrap();
        }
    }

    fn check_str_condition<Z: TimeZone>(
        db: &Db,
        time: &DateTime<Z>,
        app_id: &str,
        condition_str: &str,
    ) -> Result<bool, db::Error> {
        let condition = expressions::parse_condition(condition_str).unwrap();
        let ctx = expressions::EvalContext { db, time, app_id };
        expressions::eval(&ctx, &condition)
    }
}
