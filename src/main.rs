use chrono::{DateTime, Utc};
use std::time::SystemTime;

fn main() {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    println!("Current now() from SystemTime= {}", datetime.format("%m/%d/%Y %T"));

    // take round trip from an arbitrary datetime to systemtime and back.
    let datetime = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap();
    println!("Arbitrary Datetime = {}", datetime.format("%m/%d/%Y %T"));

    let back_to_systime: SystemTime = SystemTime::from(datetime);
    let back_to_datetime: DateTime<Utc> = back_to_systime.into();
    println!("back_to_datetime from systemtime = {}", 
        back_to_datetime.format("%m/%d/%Y %T"));

}
