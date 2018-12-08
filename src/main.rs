extern crate chrono;

use chrono::{Utc, Local, DateTime, Date};

fn main() {
    let utc_datetime: DateTime<Utc> = Utc::now();
    let utc_date: Date<Utc> = Utc::today();

    println!("{}", utc_datetime);
    println!("{}", utc_date);
}

