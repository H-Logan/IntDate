use std::convert::TryFrom;
use std::str::FromStr;

use crate::dateinfo::DateInfo;
use crate::month::Month;
use crate::weekday::Weekday;

pub struct IntDate {
    value: u32,
}
impl IntDate {
    pub fn new(value: u32) -> Self {
        IntDate { value }
    }

    fn year(&self) -> (u16, u16) {
        let mut days = self.value;
        let mut years = 1_900;

        loop {
            let leap_year = Self::is_leap_year(years) as u32;

            if days <= 366 + leap_year {
                break;
            }

            days -= 365 + leap_year;
            years += 1;
        }

        (years as u16, days as u16 - 1)
    }

    fn month(&self) -> (&str, u8, u8) {
        let (year, mut days) = self.year();
        let mut month = "";
        let mut month_num: u8 = 1;

        // month_number, month, month_length
        for (mn, (m, ml)) in Month::LENGTHS.iter().enumerate() {
            let mut length = *ml as u16;

            if days < length {
                month = m.name();
                month_num = (mn + 1) as u8;
                break;
            }

            length += (m.name() == "February" && Self::is_leap_year(year.into())) as u16;

            days -= length;
        }

        (month, month_num, days as u8)
    }

    fn weekday(&self, mut year: u16, month: u8, day: u8) -> usize {
        let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        year -= (month < 3) as u16;

        ((year + (year as f32 / 4.).floor() as u16 - (year as f32 / 100.).floor() as u16
            + (year as f32 / 400.).floor() as u16
            + t[(month - 1) as usize]
            + day as u16)
            % 7) as usize
    }

    fn is_leap_year(x: u32) -> bool {
        (x % 4 == 0) && (x % 100 != 0) || (x % 400 == 0)
    }

    pub fn get_date_info(&self) -> DateInfo {
        let (year, day_of_year) = self.year();
        let (month_name, month_num, day) = self.month();

        let month = Month::from_str(month_name).unwrap();
        let weekday = Weekday::try_from(self.weekday(year, month_num, day))
            .ok()
            .unwrap();

        DateInfo {
            year,
            month,
            weekday,
            day,
            day_of_year,
        }
    }
}
