use std::convert::TryInto;
use std::num::{ParseIntError};

use anyhow::Result;
use chrono::prelude::*;
use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
#[clap(author = "Joar Wandborg", version = "1.0.0", about = "1970 + x ms == ?")]
struct Cli {
    timestamp_millis: String
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("not really an integer")]
    ParseInt(#[from] ParseIntError),
    #[error("the question, when formulated as `{0}`, can't be answered.")]
    Empty(String)
}

fn parse_timestamp_millis(val: &str) -> Result<i64, ParsingError> {
    // who would copy a millis timestamp from a JSON object?!
    let cleaner = match val.strip_suffix(",") {
        Some(remainder) => remainder,
        None => val
    };
    Ok(cleaner.parse::<i64>()?)
}

fn main() {
    let cli: Cli = Cli::parse();

    let millis: i64 = parse_timestamp_millis(cli.timestamp_millis.as_str()).unwrap();

    let secs = millis / 1000;
    let nsecs: u32 = ((millis % 1000) * 1_000_000).try_into().unwrap();
    let ndt = NaiveDateTime::from_timestamp(secs, nsecs);
    let wokedt: DateTime<Utc> = DateTime::from_utc(ndt, Utc);

    println!("{}", wokedt.to_rfc3339());
}
