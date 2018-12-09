extern crate chrono;

use chrono::{Utc, Local, DateTime, Date, NaiveDateTime, Duration};
use chrono::offset::TimeZone;
use chrono::FixedOffset;

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

    let text = Utc::now().format("%Y年%m月%d日 %H時%M分%S秒 %Z").to_string();
    println!("{}", text); // 2018年12月09日 01時25分59秒 UTC

    let utc: DateTime<Utc> = Utc::now();
    println!("[DateTime<Utc> -> NaiveDateTime] utc: {:?}", utc); // [DateTime<Utc> -> NaiveDateTime] utc: 2018-12-09T01:37:57.621780Z
    let naive: NaiveDateTime = utc.naive_local(); // タイムゾーンがUTUなのでnaive_localとnaive_utcが一致する。
    println!("[DateTime<Utc> -> NaiveDateTime] naive_local: {:?}", naive); // [DateTime<Utc> -> NaiveDateTime] naive_local: 2018-12-09T01:37:57.621780
    let naive: NaiveDateTime = utc.naive_utc();
    println!("[DateTime<Utc> -> NaiveDateTime] naive_utc: {:?}", naive); // [DateTime<Utc> -> NaiveDateTime] naive_utc: 2018-12-09T01:37:57.621780

    let local: DateTime<Local> = Local::now();
    println!("[DateTime<Local> -> NaiveDateTime] utc: {:?}", utc); // [DateTime<Local> -> NaiveDateTime] utc: 2018-12-09T01:37:57.621780Z
    let naive: NaiveDateTime = local.naive_local(); // タイムゾーンが+9:00なのでnaive_localとnaive_utcが一致しない。
    println!("[DateTime<Local> -> NaiveDateTime] naive_local: {:?}", naive); // [DateTime<Local> -> NaiveDateTime] naive_local: 2018-12-09T10:37:57.621796
    let naive: NaiveDateTime = local.naive_utc();
    println!("[DateTime<Local> -> NaiveDateTime] naive_utc: {:?}", naive); // [DateTime<Local> -> NaiveDateTime] naive_utc: 2018-12-09T01:37:57.621796

    let dt: NaiveDateTime = NaiveDateTime::parse_from_str("2018/12/07 19:31:28", "%Y/%m/%d %H:%M:%S").unwrap();

    let utc: DateTime<Utc> = Utc.from_local_datetime(&dt).unwrap(); // この時点でchrono::offset::TimeZoneが必要
    println!("{}", utc); // 2018-12-07 19:31:28 UTC

    let local: DateTime<Local> = Local.from_local_datetime(&dt).unwrap();
    println!("{}", local); // 2018-12-07 19:31:28 +09:00

    let offset: DateTime<FixedOffset> = FixedOffset::east(3 * 3600).from_local_datetime(&dt).unwrap();
    println!("{}", offset); // 2018-12-07 19:31:28 +03:00

    let dt1: NaiveDateTime = NaiveDateTime::parse_from_str("2018/12/07 19:31:28", "%Y/%m/%d %H:%M:%S").unwrap();
    let dt2: NaiveDateTime = NaiveDateTime::parse_from_str("2018/12/07 12:31:28", "%Y/%m/%d %H:%M:%S").unwrap();
    let duration: Duration = dt1 - dt2;

    println!("secs: {}", duration.num_seconds()); // secs: 25200
    println!("minutes: {}", duration.num_minutes()); // minutes: 420
    println!("hours: {}", duration.num_hours()); // hours: 7

    println!("dt3: {}", dt1 + Duration::hours(3)); // dt3: 2018-12-07 22:31:28

    let local: DateTime<Local> = Local.datetime_from_str("2018/12/07 19:31:28", "%Y/%m/%d %H:%M:%S").unwrap();
    println!("local: {}", local); // local: 2018-12-07 19:31:28 +09:00
    let dt4: DateTime<Utc> = local.with_timezone(&Utc);
    println!("dt4: {}", dt4); // dt4: 2018-12-07 10:31:28 UTC

    let local: DateTime<Local> = Local.timestamp(1_500_000_000, 0);
    println!("local: {}", local); // local: 2017-07-14 11:40:00 +09:00
    let timestamp: i64 = local.timestamp();
    println!("unix time: {}", timestamp); // unix time: 1500000000
}

