use std::convert::TryInto;
use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::prelude::*;
use clap::{Command, CommandFactory, Parser};
use num_integer::div_mod_floor;
use thiserror::Error;

#[derive(Parser)]
#[command(author = "Joar Wandborg", version, long_about)]
/// Converts millisecond-precision UNIX timestamps to the more human-readable and as-precise
/// RFC3339 form.
struct Cli {
    /// A timestamp formulated as the number of milliseconds since "1970-01-01T00:00:00+00:00".
    ///{n}
    /// • Trailing and leading garbage is thrown away, i.e.{n}
    /// • `1 hello there`, `1,` and `"1",` would all be interpreted as `1`.{n}
    /// • Negative numbers are fine, positive numbers are ok too, both have some limitations:{n}
    /// • We can't construct datetimes outside the range of (-262144-01-01T00:00:00Z, +262143-12-31T23:59:59.999999999Z), so{n}
    /// • we only accept input values in the range of (-8334632851200000, 8210298412799999).
    #[arg(allow_hyphen_values = true)]
    timestamp_millis: String,
}

#[derive(Error, Debug)]
enum CliError {
    #[error("FromTimestamp error: {0}")]
    FromTimestamp(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// returns the opposite of `char::is_ascii_digit`
fn not_ascii_digit(ch: char) -> bool {
    !ch.is_ascii_digit()
}

/// Figure where the millisecond timestamp is hidden in a string.
fn parse_timestamp_millis(val: &str) -> Result<i64> {
    let numeric_str = val
        // A single leading hyphen is fine, so an infinite number of leading hyphens should be fine too.
        .trim_start_matches(|ch| not_ascii_digit(ch) && ch != '-')
        .trim_end_matches(not_ascii_digit);
    numeric_str.parse::<i64>().with_context(|| {
        format!(
            "could not parse integer from trimmed string {0:?}",
            numeric_str
        )
    })
}

fn split_timestamp_millis(millis: i64) -> Result<(i64, u32)> {
    let (secs, rem_millis) = div_mod_floor(millis, 1000);
    let nanos = (rem_millis * 1_000_000).abs().try_into().with_context(|| {
        format!(
            "could not fit nanos (i64 -> u32) {0:?}",
            (rem_millis * 1_000_000)
        )
    })?;
    Ok((secs, nanos))
}

fn naive_datetime_from_timestamp_millis(millis: i64) -> Result<NaiveDateTime, CliError> {
    let (secs, nanos) = split_timestamp_millis(millis)?;
    match NaiveDateTime::from_timestamp_opt(secs, nanos) {
        Some(ndt) => Ok(ndt),
        None => {
            Err(CliError::FromTimestamp(format!(
                "Sorry, we can't handle timestamps outside the range ({:?}, {:?}), because we can't represent datetimes outside the range ({:?}, {:?})",
                chrono::MIN_DATETIME.timestamp_millis(), chrono::MAX_DATETIME.timestamp_millis(),
                chrono::MIN_DATETIME, chrono::MAX_DATETIME
            )))
        }
    }
}

fn datetime_utc_from_timestamp_millis(timestamp_millis: i64) -> Result<DateTime<Utc>> {
    Ok(DateTime::from_utc(
        naive_datetime_from_timestamp_millis(timestamp_millis)?,
        Utc,
    ))
}

/// Performs  arithmetic to figure out the RFC 3339 representation of a millisecond timestamp.
fn rfc3339_from_timestamp_millis(millis: i64) -> Result<String> {
    datetime_utc_from_timestamp_millis(millis).map(|dt| dt.to_rfc3339())
}

fn gen_manpage(path: &PathBuf) -> Result<()> {
    let command: Command = Cli::command();
    let man = clap_mangen::Man::new(command);

    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(std::path::Path::new(path), buffer)?;
    eprintln!("Wrote manpage to {:?}", path);
    Ok(())
}

fn main() -> Result<()> {
    if let Some(path) = std::env::var_os("UNMILLIS_GEN_MANPAGE_PATH") {
        return gen_manpage(&path.into());
    }

    let cli: Cli = Cli::parse();

    let millis: i64 = parse_timestamp_millis(cli.timestamp_millis.as_str()).with_context(|| {
        format!(
            "Failed to parse timestamp millis from {0:?}",
            cli.timestamp_millis
        )
    })?;
    let rfc3339 = rfc3339_from_timestamp_millis(millis).with_context(|| {
        format!(
            "could not generate RFC 3339 datetime from millis: {0:?}",
            millis
        )
    });
    println!("{}", rfc3339?);
    Ok(())
}

#[cfg(test)]
mod tests {
    macro_rules! from_timestamp_millis_tests {
        ($($name:ident: $millis:expr,)*) => {
            mod datetime_utc_from_timestamp_millis {
                use crate::datetime_utc_from_timestamp_millis;
            $(
                #[test]
                fn $name() {
                    let ndt = datetime_utc_from_timestamp_millis($millis).unwrap();
                    let ndt_millis = ndt.timestamp_millis();
                    println!("{0:?} -> {1:?} -> {2:?}", $millis, ndt, ndt_millis);
                    assert_eq!(ndt_millis, $millis);
                }
            )*
            }
        };
    }
    from_timestamp_millis_tests! {
        negative_1h: -1000 * 60 * 60,
        negative_1100ms: -1100,
        negative_1500ms: -1500,
        negative_1ms: -1,
        zero: 0,
        positive_1ms: 1,
        positive_1s: 1000,
        now_back_then: 1645450419455i64,
        max_datetime: chrono::MAX_DATETIME.timestamp_millis(),
        min_datetime: chrono::MIN_DATETIME.timestamp_millis(),
    }

    macro_rules! parse_timestamp_millis_tests {
        ($($name:ident: [$input:expr, $output:expr],)*) => {
            mod parse_timestamp_millis {
                use crate::parse_timestamp_millis;
            $(
            #[test]
            fn $name() {
                assert_eq!(parse_timestamp_millis($input).unwrap(), $output);
            }
            )*
            }
        }
    }

    parse_timestamp_millis_tests! {
        should_be_happy: ["123", 123],
        // who would copy a millis timestamp from a JSON object?!
        should_trim_trailing: ["123,", 123],
        should_trim_leading: ["\"123", 123],
        should_trim_both: ["\"123\"", 123],
        should_not_understand_binary: ["101010", 101010],
        should_trim_null_bytes: ["\01\0", 1],
        should_not_trim_leading_hyphen: ["-10", -10],
        should_ignore_non_numeric_sql_injections: [" 001; DROP TABLE timestamps WHERE year = 'the-seventies'", 1],
        should_not_be_distracted_by_ancient_greek_numerals: ["𐅀42", 42],
    }
}
