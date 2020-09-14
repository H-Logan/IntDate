use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Write::write_str(f, self.name())
    }
}
impl core::str::FromStr for Month {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() == 3 {
            Ok( match () {
                _ if input.eq_ignore_ascii_case(&Self::January.name()[..3])
                    => Self::January,

                _ if input.eq_ignore_ascii_case(&Self::February.name()[..3])
                    => Self::February,

                _ if input.eq_ignore_ascii_case(&Self::March.name()[..3])
                    => Self::March,

                _ if input.eq_ignore_ascii_case(&Self::April.name()[..3])
                    => Self::April,

                _ if input.eq_ignore_ascii_case(&Self::May.name()[..3])
                    => Self::May,

                _ if input.eq_ignore_ascii_case(&Self::June.name()[..3])
                    => Self::June,

                _ if input.eq_ignore_ascii_case(&Self::July.name()[..3])
                    => Self::July,

                _ if input.eq_ignore_ascii_case(&Self::August.name()[..3])
                    => Self::August,

                _ if input.eq_ignore_ascii_case(&Self::September.name()[..3])
                    => Self::September,

                _ if input.eq_ignore_ascii_case(&Self::October.name()[..3])
                    => Self::October,

                _ if input.eq_ignore_ascii_case(&Self::November.name()[..3])
                    => Self::November,

                _ if input.eq_ignore_ascii_case(&Self::December.name()[..3])
                    => Self::December,

                _ => return Err(()),
            })
        } else {
            Ok(match input {
                "January" => Self::January,
                "February" => Self::February,
                "March" => Self::March,
                "April" => Self::April,
                "May" => Self::May,
                "June" => Self::June,
                "July" => Self::July,
                "August" => Self::August,
                "September" => Self::September,
                "October" => Self::October,
                "November" => Self::November,
                "December" => Self::December,
                _ => return Err(()),
            })
        }
    }
}
impl Month {
    pub const LIST: [Self; 12] = [
        Self::January,
        Self::February,
        Self::March,
        Self::April,
        Self::May,
        Self::June,
        Self::July,
        Self::August,
        Self::September,
        Self::October,
        Self::November,
        Self::December,
    ];

    pub const LENGTHS: [(Self, u8); 12] = [
        (Self::January, 31),
        (Self::February, 28),
        (Self::March, 31),
        (Self::April, 30),
        (Self::May, 31),
        (Self::June, 30),
        (Self::July, 31),
        (Self::August, 31),
        (Self::September, 30),
        (Self::October, 31),
        (Self::November, 30),
        (Self::December, 31),
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::January => "January",
            Self::February => "February",
            Self::March => "March",
            Self::April => "April",
            Self::May => "May",
            Self::June => "June",
            Self::July => "July",
            Self::August => "August",
            Self::September => "September",
            Self::October => "October",
            Self::November => "November",
            Self::December => "December",
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Self::January => 1,
            Self::February => 2,
            Self::March => 3,
            Self::April => 4,
            Self::May => 5,
            Self::June => 6,
            Self::July => 7,
            Self::August => 8,
            Self::September => 9,
            Self::October => 10,
            Self::November => 11,
            Self::December => 12,
        }
    }
}
