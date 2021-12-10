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
    Weekday,
    InWindow(ConditionInWindow),
    InCurrent(ConditionInCurrent)
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
pub struct ConditionInWindow {
    pub limit: Duration,
    pub window_size: Duration
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConditionInCurrent {
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
            condition_in_current,
            condition_in_window
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
    let (input, _) = tag("weekday")(s)?;
    Ok((input, Condition::Weekday))
}

fn condition_in_current<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    let (input, (_, _, limit, _, _, _, _, _, time_unit)) = context(
        "condition_in_current",
        tuple((
            tag("max"),
            multispace1,
            duration,
            multispace1,
            tag("in"),
            multispace1,
            tag("current"),
            multispace1,
            time_unit
        ))
    )(s)?;
    Ok((input, Condition::InCurrent(
                ConditionInCurrent { limit, time_unit }
    )))
}

fn condition_in_window<'a>(s: &'a str) -> IResult<&'a str, Condition, Error<&'a str>> {
    let (input, (_, _, limit, _, _, _, _, _, window_size)) = context(
        "condition_in_window",
        tuple((
            tag("max"),
            multispace1,
            duration,
            multispace1,
            tag("in"),
            multispace1,
            tag("window"),
            multispace1,
            duration
        ))
    )(s)?;
    Ok((input, Condition::InWindow(
                ConditionInWindow { limit, window_size }
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
        assert_eq!(Ok(Condition::Weekday), parse_condition("weekday"));
    }

    #[test]
    fn whitespace_around_expression_doesnt_matter() {
        let examples = vec![
            " weekday",
            "weekday ",
            " weekday ",
            "\n\nweekday",
            "\tweekday\n\n"
        ];
        for e in examples {
            assert_eq!(Ok(Condition::Weekday), parse_condition(e));
        }
    }

    #[test]
    fn not() {
        assert_eq!(
            Ok(
                Condition::Not(ConditionNot { 
                    c: Box::new(Condition:: Weekday) 
                })
            ),
            parse_condition("not weekday")
        );
    }

    #[test]
    fn brackets_dont_matter() {
        let examples = vec![
            "weekday and weekday",
            "(weekday and weekday)",
            "( weekday and weekday )",
            "(weekday) and weekday",
            "weekday and (weekday)",
            "(weekday) and (weekday)",
            "( (weekday) and (weekday) )",
            "( weekday
                and
                        weekday  \t\t)"
        ];
        for e in examples {
            assert_eq!(
                Ok(
                    Condition::And(ConditionAnd {
                        c1: Box::new(Condition::Weekday),
                        c2: Box::new(Condition::Weekday)
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
                                c1: Box::new(Condition::Weekday),
                                c2: Box::new(Condition::Weekday),
                            })
                        ),
                    c2: Box::new(Condition::Weekday)
                })
            ),
            parse_condition("(weekday or weekday) and weekday)")
        );
    }
}
