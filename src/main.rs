extern crate chrono;

use chrono::{Utc, Local, DateTime, Date, NaiveDateTime};

fn main() {
    let utc_datetime: DateTime<Utc> = Utc::now();
    let utc_date: Date<Utc> = Utc::today();

    println!("{}", utc_datetime); // 2018-12-08 12:45:45.389758 UTC
    println!("{}", utc_date); // 2018-12-08UTC

    let local_datetime: DateTime<Local> = Local::now();
    let local_date: Date<Local> = Local::today();

    println!("{}", local_datetime); // 2018-12-08 21:45:45.389882 +09:00
    println!("{}", local_date); // 2018-12-08+09:00

    let dt = DateTime::parse_from_rfc3339("2018-12-07T19:31:28+09:00");
    println!("DateTime::parse_from_rfc3339: {:?}", dt); // DateTime::parse_from_rfc3339: Ok(2018-12-07T19:31:28+09:00)

    // タイムゾーン情報がないのでErr()が返ってくる
    let dt = DateTime::parse_from_str("2018/12/07 19:31:28", "%Y/%m/%d %H:%M:%S");
    println!("DateTime::parse_from_str: {:?}", dt); // DateTime::parse_from_str: Err(ParseError(NotEnough))

    // Naiveであればタイムゾーン情報がなくても扱える
    let dt = NaiveDateTime::parse_from_str("2018/12/07 19:31:28", "%Y/%m/%d %H:%M:%S");
    println!("DateTime::parse_from_str: {:?}", dt); // DateTime::parse_from_str: Ok(2018-12-07T19:31:28)
}

