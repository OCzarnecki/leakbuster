use nom::{
    Finish,
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{map_res, recognize},
    error::{context, convert_error, ParseError, VerboseError},
    multi::many1,
    sequence::{preceded, terminated, delimited, tuple},
};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Condition {
    And(ConditionAnd),
    Or(ConditionOr),
    Not(ConditionNot),
    Weekday(ConditionWeekday),
    AtMostInSliding(ConditionAtMostInSliding),
    AtMostInThis(ConditionAtMostInThis)
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConditionAnd {
    pub c1: Box<Condition>,
    pub c2: Box<Condition>
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConditionOr {
    pub c1: Box<Condition>,
    pub c2: Box<Condition>
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConditionNot {
    pub c: Box<Condition>
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConditionWeekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConditionAtMostInSliding {
    pub limit: Duration,
    pub window_size: Duration
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConditionAtMostInThis {
    pub limit: Duration,
    pub time_unit: TimeUnit
}

#[derive(Debug, Eq, PartialEq)]
pub struct Duration {
    pub seconds: u64
}

#[derive(Debug, Eq, PartialEq)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year
}

type Error<I> = VerboseError<I>;

pub fn parse_condition(expr: &str) -> Result<Condition, String> {
    let result: IResult<&str, Condition, VerboseError<&str>> = delimited(
        multispace0,
        condition_expr,
        multispace0
    )(expr);
    match result.finish() {
        Ok((_, cond)) => Ok(cond),
        Err(e) => Err(format!(
            "Couldn't parse condition: {}",
            convert_error(expr, e)
        ))
    }
}


fn condition_expr<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    context(
        "condition_expr",
        alt((
            condition_and,
            condition_or,
            condition_not,
            condition_term
    )))(s)
}

fn condition_term<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    context(
        "condition_term",
        alt((
            delimited(
                terminated(char('('), multispace0),
                condition_expr,
                preceded(multispace0, char(')'))
            ),
            condition_weekday,
            condition_at_most_in_this,
            condition_at_most_in_sliding
    )))(s)
}

fn condition_and<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    let (input, (c1, _, _, _, c2)) = context(
        "condition_and",
        tuple((
            condition_term,
            multispace1,
            tag("and"),
            multispace1,
            condition_expr
        ))
    )(s)?;
    let cnd = ConditionAnd {
        c1: Box::new(c1), c2: Box::new(c2)
    };
    Ok((input, Condition::And(cnd)))
}

fn condition_or<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    let (input, (c1, _, _, _, c2)) = context(
        "condition_or",
        tuple((
            condition_term,
            multispace1,
            tag("or"),
            multispace1,
            condition_expr
        ))
    )(s)?;
    let cnd = ConditionOr {
        c1: Box::new(c1), c2: Box::new(c2)
    };
    Ok((input, Condition::Or(cnd)))
}

fn condition_not<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    let (input, (_, _, c)) = context(
        "condition_not",
        tuple((
        tag("not"),
        multispace1,
        condition_expr
        ))
    )(s)?;
    let cnd = ConditionNot { c: Box::new(c) };
    Ok((input, Condition::Not(cnd)))
}

fn condition_weekday<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    let (input, weekday_str) = context(
        "condition_weekday",
        alt((
            tag("Mon"),
            tag("Tue"),
            tag("Wed"),
            tag("Thu"),
            tag("Fri"),
            tag("Sat"),
            tag("Sun"),
        )))(s)?;
    let wd = match weekday_str {
        "Mon" => ConditionWeekday::Mon,
        "Tue" => ConditionWeekday::Tue,
        "Wed" => ConditionWeekday::Wed,
        "Thu" => ConditionWeekday::Thu,
        "Fri" => ConditionWeekday::Fri,
        "Sat" => ConditionWeekday::Sat,
        "Sun" => ConditionWeekday::Sun,
        _ => panic!("Reached unreachable code. Draw your own conclusions...")
    };
    Ok((input, Condition::Weekday(wd)))
}

fn condition_at_most_in_this<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    let (input, (_, _, limit, _, _, _, _, _, time_unit)) = context(
        "condition_at_most_in_this",
        tuple((
            tag("atmost"),
            multispace1,
            duration,
            multispace1,
            tag("in"),
            multispace1,
            tag("this"),
            multispace1,
            time_unit
        ))
    )(s)?;
    Ok((input, Condition::AtMostInThis(
                ConditionAtMostInThis { limit, time_unit }
    )))
}

fn condition_at_most_in_sliding<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    let (input, (_, _, limit, _, _, _, _, _, window_size)) = context(
        "condition_at_most_in_sliding",
        tuple((
            tag("atmost"),
            multispace1,
            duration,
            multispace1,
            tag("in"),
            multispace1,
            tag("sliding"),
            multispace1,
            duration
        ))
    )(s)?;
    Ok((input, Condition::AtMostInSliding(
                ConditionAtMostInSliding { limit, window_size }
    )))
}

fn duration<'a>(s: &'a str) -> IResult<&'a str, Duration, Error<&'a str>> {
    let (input, (number, _, unit)) = context(
        "duration",
        tuple((
            integer,
            multispace1,
            one_of("smhDW")
        ))
    )(s)?;
    let unit_seconds = match unit {
        's' => 1,
        'm' => 60,
        'h' => 3600,
        'D' => 86400,
        'W' => 604800,
        _ => panic!("Reached unreachable state. Draw your own conclusions.")
    };
    Ok((input, Duration { seconds: number * unit_seconds } ))
}

fn time_unit<'a>(s: &'a str) -> IResult<&'a str, TimeUnit, Error<&'a str>> {
    let (input, unit_str) = context(
        "time_unit",
        alt((
            tag("second"),
            tag("minute"),
            tag("hour"),
            tag("day"),
            tag("week"),
            tag("month"),
            tag("year")
        ))
    )(s)?;
    let unit = match unit_str {
        "second" => TimeUnit::Second,
        "minute" => TimeUnit::Minute,
        "hour" => TimeUnit::Hour,
        "day" => TimeUnit::Day,
        "week" => TimeUnit::Week,
        "month" => TimeUnit::Month,
        "year" => TimeUnit::Year,
        _ => panic!("Reached unreachable state. Draw your own conclusions.")
    };
    Ok((input, unit))
}

fn integer<'a>(s: &'a str) -> IResult<&'a str, u64, Error<&'a str>> {
    context(
        "integer",
        map_res(
            recognize(
                many1(
                    one_of("0123456789")
                )
            ),
            |s: &str| u64::from_str(s)
        )
    )(s)
}

mod test {
    use crate::expressions::parser::*;

    #[test]
    fn weekday() {
        let test_cases = vec![
            ("Mon", ConditionWeekday::Mon),
            ("Tue", ConditionWeekday::Tue),
            ("Wed", ConditionWeekday::Wed),
            ("Thu", ConditionWeekday::Thu),
            ("Fri", ConditionWeekday::Fri),
            ("Sat", ConditionWeekday::Sat),
            ("Sun", ConditionWeekday::Sun),
        ];
        for (expr, cnd) in test_cases {
            assert_eq!(Ok(Condition::Weekday(cnd)), parse_condition(expr));
        };
    }

    #[test]
    fn whitespace_around_expression_doesnt_matter() {
        let examples = vec![
            " Mon",
            "Mon ",
            " Mon ",
            "\n\nMon",
            "\tMon\n\n"
        ];
        for e in examples {
            assert_eq!(Ok(Condition::Weekday(ConditionWeekday::Mon)), parse_condition(e));
        }
    }

    #[test]
    fn not() {
        assert_eq!(
            Ok(
                Condition::Not(ConditionNot { 
                    c: Box::new(Condition::Weekday(ConditionWeekday::Mon)),
                })
            ),
            parse_condition("not Mon")
        );
    }

    #[test]
    fn superfluous_brackets_dont_matter() {
        let examples = vec![
            "Mon and Mon",
            "(Mon and Mon)",
            "( Mon and Mon )",
            "(Mon) and Mon",
            "Mon and (Mon)",
            "(Mon) and (Mon)",
            "( (Mon) and (Mon) )",
            "( Mon
                and
                        Mon  \t\t)"
        ];
        for e in examples {
            assert_eq!(
                Ok(
                    Condition::And(ConditionAnd {
                        c1: Box::new(Condition::Weekday(ConditionWeekday::Mon)),
                        c2: Box::new(Condition::Weekday(ConditionWeekday::Mon)),
                    })
                ),
                parse_condition(e),
                "Trying to parse '{}'", e 
            );
        }
    }

    #[test]
    fn and_or() {
        assert_eq!(
            Ok(
                Condition::And(ConditionAnd {
                    c1: Box::new(
                            Condition::Or(ConditionOr {
                                c1: Box::new(Condition::Weekday(ConditionWeekday::Mon)),
                                c2: Box::new(Condition::Weekday(ConditionWeekday::Mon)),
                            })
                        ),
                    c2: Box::new(Condition::Weekday(ConditionWeekday::Mon)),
                })
            ),
            parse_condition("(Mon or Mon) and Mon)")
        );
    }

    #[test]
    fn at_most_in_sliding() {
        let exprs = vec![
            "atmost 15 s in sliding 10 m",
            " atmost    15  s     in  sliding     10  m ",
            "atmost\n\n15\ns\nin\nsliding\n10\nm",
            "\tatmost\t15\ts\tin\tsliding\t10\tm\t"
        ];
        for e in exprs {
            assert_eq!(
                Ok(
                    Condition::AtMostInSliding(ConditionAtMostInSliding {
                        limit: Duration { seconds: 15 },
                        window_size: Duration {seconds: 600 }
                    })
                ),
                parse_condition(e)
            );
        }
    }

    #[test]
    fn at_most_in_this() {
        let exprs = vec![
            "atmost 15 s in this week",
            " atmost    15  s     in  this     week",
            "atmost\n\n15\ns\nin\nthis\nweek\n",
            "\tatmost\t15\ts\tin\tthis\tweek\t"
        ];
        for e in exprs {
            assert_eq!(
                Ok(
                    Condition::AtMostInThis(ConditionAtMostInThis {
                        limit: Duration { seconds: 15 },
                        time_unit: TimeUnit::Week
                    })
                ),
                parse_condition(e)
            );
        }
    }

    #[test]
    fn integers() {
        let test_cases = vec![0, 1, 10, 12343, 0007, 18446744073709551615u64];
        for t in test_cases {
            match integer(&u64::to_string(&t)) {
                Ok((_, x)) => assert_eq!(t, x),
                _ => panic!("Parsing int resulted in error")
            }
        }
    }
}
