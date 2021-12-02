# Condition language specification

The condition languge allows specifying conditions for when hooks should fire.
Example conditions could be "Application was running for at least 30 minutes today" or "It is a weekday and it's between 8:00 and 18:00".

Non-terminals are wrapped in curly braces, like {Condition}, everything else is a literal.

## Condition

Top level expression that can be evauated by the leak-buster tooling.

```
{Condition} → ( {Condition} )
            | between {Time} and {Time}
            | max {Duration} in current {TimeUnit}
            | max {Duration} in window {Duration}
            | not {Condition}
            | Weekday
            | {Condition} and {Condition}
            | {Condition} or {Condition}
```

## Time

A time of day, like
* 00:00
* 07:15
* 7:15
* 13:37

Must be in a valid range (so not 26:99).

## Duration

{Duration} → {Number} s
           | {Number} m
           | {Number} h
           | {Number} D
           | {Number} W

A duration in seconds, months, hours, days, or weeks. Other units aren't permitted, since those don't have a clear conversion to seconds (this tool doesn't support timezones, leap seconds or all the other quirks with time. If you need that feature, raise an issue, and I'll look into it).

Durations are just a way of specifying a number of seconds, assuming (sometimes incorrectly!):
1 minute = 60 s
1 hour = 60 minutes
1 day = 24 hours
1 week = 7

## TimeUnit

{TimeUnit} → second | minute | hour | day | week | month | year
