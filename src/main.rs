extern crate chrono;

use chrono::{Utc, Local, DateTime, Date};

fn main() {
    let utc_datetime: DateTime<Utc> = Utc::now();
    let utc_date: Date<Utc> = Utc::today();

    println!("{}", utc_datetime);
    println!("{}", utc_date);

    let local_datetime: DateTime<Local> = Local::now();
    let local_date: Date<Local> = Local::today();

    println!("{}", local_datetime);
    println!("{}", local_date);
}

