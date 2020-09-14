#![allow(unused)]

mod dateinfo;
mod intdate;
mod month;
mod weekday;

use intdate::IntDate;

macro_rules! idate {
    ($num:expr) => {
        IntDate::new($num).get_date_info()
    }
}

fn main() {
    let num = 42433; // Friday, March 4th, 2016
    let date = idate!(num);

    println!("{}", date.format("ISO 8601 Standard Date Format: %Y-%0m-%0d"));
}
