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
    let num = 42429; // Friday, March 4th, 2016
    let date = idate!(num);

    println!("{}", date.format(
        "
        Year:\t\t(long) %Y\t(short) %y
        Month:\t\t(long) %M\t(short) %m
        Month #:\t(long) %0m\t(short) %-m
        Weekday:\t(long) %D\t(short) %d
        Day #:\t\t(long) %0d\t(short) %-d
        Day of Year:\t(long) %0j\t(short) %j
        Suffixed Day:\t%.d
        "
    ));
}
