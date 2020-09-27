#![allow(unused)]

pub mod dateinfo;
pub mod intdate;
pub mod month;
pub mod weekday;

pub use self::intdate::IntDate;

// Excel counts 1900 as a leap year, which is stupid.
// therefore, to make this compatible with Excel for
// January through March of 1900, $is_stupid must be true.

#[macro_export]
macro_rules! idate {
    ($num:expr, $is_stupid:expr) => {
        IntDate::new($num, $is_stupid).get_date_info()
    };
}

#[cfg(test)]
mod tests {
    use crate::IntDate;

    #[test]
    fn day_30() {
        let date = idate!(30, true);
        assert_eq!(
            &format!("{}", date.format("%D, %M %0d, %Y")),
            "Monday, January 30, 1900"
        );
    }

    #[test]
    fn day_31() {
        let date = idate!(31, true);
        assert_eq!(
            &format!("{}", date.format("%d, %m %0d, %Y")),
            "Tue, Jan 31, 1900"
        );
    }
}
