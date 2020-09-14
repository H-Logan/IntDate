use std::fmt;

pub enum WeekdayError {
    InvalidWeekdayValue,
}
impl fmt::Display for WeekdayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeekdayError::InvalidWeekdayValue => {
                write!(f, "Invalid value for Weekday (Accepts 1 - 7)")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Write::write_str(f, self.name())
    }
}
impl std::convert::TryFrom<usize> for Weekday {
    type Error = WeekdayError;
    fn try_from(input: usize) -> Result<Self, WeekdayError> {
        match Self::LIST.get(input as usize) {
            Some(&d) => Ok(d),
            None => Err(WeekdayError::InvalidWeekdayValue),
        }
    }
}
impl Weekday {
    pub const LIST: [Self; 7] = [
        Self::Sunday,
        Self::Monday,
        Self::Tuesday,
        Self::Wednesday,
        Self::Thursday,
        Self::Friday,
        Self::Saturday,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Self::Monday => 1,
            Self::Tuesday => 2,
            Self::Wednesday => 3,
            Self::Thursday => 4,
            Self::Friday => 5,
            Self::Saturday => 6,
            Self::Sunday => 7,
        }
    }
}
