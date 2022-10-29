// https://lib.rs/crates/chrono
use chrono::{DateTime, FixedOffset};
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#positionals
use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(version="0.1.0")]
#[command(about="Display the current time in the given FORMAT", long_about=None)]
#[command(group(
    ArgGroup::new("use_utc")
        .required(false)
        .args(["utc", "universal"]),
))]
struct Args {
    #[arg(index=1, value_name="FORMAT", help="specify the output format")]
    fmt: Option<String>,

    #[arg(short='d', long, help="specify the datetime, 'now' by default, local timezone by default")]
    date: Option<String>,

    #[arg(short='u', long, default_value_t=false, group="input", help="Print in UTC")]
    utc: bool,

    #[arg(long, default_value_t=false, group="input", help="Print in UTC")]
    universal: bool,

    #[arg(long, action=clap::ArgAction::Count, help="Print debug information")]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    // println!("{:?}", &args);

    let now = chrono::Local::now();
    let use_utc = args.utc || args.universal;

    let dt = match &args.date {
        Some(s) => s.parse::<DateTime<chrono::Local>>(),
        None => Ok(now.clone()),
    };

    let dt = dt.map(|dt| {
        if use_utc {
            dt.with_timezone(&FixedOffset::east(0))
        }
        else {
            dt.with_timezone(now.offset())
        }
    });

    let fmt = match &args.fmt {
        Some(s) => &s,
        None => "%Y-%m-%d %H:%M:%S",
    };

    println!("{}", dt.unwrap().format(fmt).to_string());
}
