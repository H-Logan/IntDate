#![allow(unused)]

mod dateinfo;
mod intdate;
mod month;
mod weekday;

use self::intdate::IntDate;

#[macro_export]
macro_rules! idate {
    ($num:expr) => {
        IntDate::new($num).get_date_info()
    }
}

#[cfg(test)]
mod tests { 

    use crate::IntDate;

    #[test]
    fn iter_2015() {
        let start = 42005;
        let end = 42369; 
        
        for n in start..=end {
            let date = idate!(n);
            println!("{}", date.format("%M %-d, %Y"));
        }
    }

    #[test]
    fn iter_2016() {
        let start = 42370;
        let end = 42735; 
        
        for n in start..=end {
            let date = idate!(n);
            println!("{}\t{}", date.format("%M %-d, %Y"), n);
        }
    }
    
    #[test]
    fn print_date_info() {
        let num = 42433; // Friday, March 4th, 2016
        let date = idate!(num);
        println!("{}", date.format(
            "
            Year:\t\t(long) %Y\t(short) %y
            Month:\t\t(long) %M\t(short) %m
            Month #:\t\t(long) %0m\t(short) %-m
            Weekday:\t\t(long) %D\t(short) %d
            Day #:\t\t(long) %0d\t(short) %-d
            Day of Year:\t(long) %0j\t(short) %j
            Suffixed Day:\t%.d
            "
        ));
    }
}