pub mod config;
pub mod db;
pub mod usage_monitor;

use crate::parse_tree::*;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    sequence::{preceded, terminated, delimited, tuple},
};

mod parse_tree {

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
        pub current: TimeUnit
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
}

pub fn parse_condition(expr: &str) -> Option<Condition> {
    let (_, cond) = delimited(
        multispace0,
        condition_expr,
        multispace0
    )(expr).ok()?;
    Some(cond)
}

fn condition_expr(s: &str) -> IResult<&str, Condition> {
    alt((
        condition_and,
        condition_or,
        condition_not,
        condition_term
    ))(s)
}

fn condition_term(s: &str) -> IResult<&str, Condition> {
    alt((
        delimited(
            terminated(char('('), multispace0),
            condition_expr,
            preceded(multispace0, char(')'))
        ),
        condition_weekday
    ))(s)
}

fn condition_and(s: &str) -> IResult<&str, Condition> {
    let (input, (c1, _, _, _, c2)) = tuple((
        condition_term,
        multispace1,
        tag("and"),
        multispace1,
        condition_expr
    ))(s)?;
    let cnd = ConditionAnd {
        c1: Box::new(c1), c2: Box::new(c2)
    };
    Ok((input, Condition::And(cnd)))
}

fn condition_or(s: &str) -> IResult<&str, Condition> {
    let (input, (c1, _, _, _, c2)) = tuple((
        condition_term,
        multispace1,
        tag("or"),
        multispace1,
        condition_expr
    ))(s)?;
    let cnd = ConditionOr {
        c1: Box::new(c1), c2: Box::new(c2)
    };
    Ok((input, Condition::Or(cnd)))
}

fn condition_not(s: &str) -> IResult<&str, Condition> {
    let (input, (_, _, c)) = tuple((
        tag("not"),
        multispace1,
        condition_expr
    ))(s)?;
    let cnd = ConditionNot { c: Box::new(c) };
    Ok((input, Condition::Not(cnd)))
}

fn condition_weekday(s: &str) -> IResult<&str, Condition> {
    let (input, _) = tag("weekday")(s)?;
    Ok((input, Condition::Weekday))
}

mod test {
    use crate::parse_condition;
    use crate::parse_tree::*;

    #[test]
    fn weekday() {
        assert_eq!(Some(Condition::Weekday), parse_condition("weekday"));
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
            assert_eq!(Some(Condition::Weekday), parse_condition(e));
        }
    }

    #[test]
    fn not() {
        assert_eq!(
            Some(
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
                Some(
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
            Some(
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
