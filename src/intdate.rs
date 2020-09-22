use std::convert::TryFrom;
use std::str::FromStr;

use crate::dateinfo::DateInfo;
use crate::month::Month;
use crate::weekday::Weekday;


pub struct IntDate {
    value: u32,
    is_stupid: bool,
}
impl IntDate {
    pub fn new(value: u32, is_stupid: bool) -> Self {
        IntDate { value, is_stupid }
    }

    fn year(&self) -> (u16, u16) {
        let mut days = self.value;
        let mut years = 1_900;

        loop {
            let leap_year = if self.is_leap_year(years) {1} else {0};

            if days <= (365 + leap_year) { break; }

            days -= (365 + leap_year);
            years += 1;
        }

        (years as u16, days as u16)
    }

    fn month(&self, year: u16, mut days: u16)
        -> (&str, u8, u8) {
        let mut month = "January";
        let mut month_num: u8 = 1;

        let mut last_month = "December";
        let mut last_length: u16 = 31;

        // month_number, month, month_length
        for (mn, (m, ml)) in Month::LENGTHS.iter().enumerate() {
            let mut length = *ml as u16;
            if m.name() == "February" && self.is_leap_year(year)
                { length = 29; }

            if days < length {
                month = m.name();
                month_num = (mn + 1) as u8;
                break;
            }

            days -= length;

            last_length = length;
            last_month = m.name();
        }

        if days <= 0 {
            days = last_length;
            month = last_month;
        }
        (month, month_num, days as u8)
    }

    fn weekday(&self, mut year: u16, month: u8, mut day: u8) -> usize {
        if self.is_stupid {
            day -= (year == 1_900) as u8;
            print!("Day: {}, Month: {}, Year: {},\t\t", day, month, year);
        }

        let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        year -= (month < 3) as u16;

        (( year
            + (year as f32 / 4.).floor() as u16
            - (year as f32 / 100.).floor() as u16
            + (year as f32 / 400.).floor() as u16
            + t[(month - 1) as usize]
            + day as u16
            + (year == 1_900 && self.is_stupid) as u16
        ) % 7) as usize
    }

    fn is_leap_year(&self, x: u16) -> bool {
        (x % 4 == 0) && (x % 100 != 0) || (x % 400 == 0)
        || (x == 1_900) && (self.is_stupid)
    }

    pub fn get_date_info(&self) -> DateInfo {
        let (year, day_of_year) = self.year();
        let (month_name, month_num, day) = self.month(year, day_of_year);

        let month = Month::from_str(month_name).unwrap();
        let weekday = Weekday::try_from(
            self.weekday(year, month_num, day)
        ).unwrap();

        DateInfo {
            year,
            month,
            weekday,
            day,
            day_of_year,
        }
    }
}
