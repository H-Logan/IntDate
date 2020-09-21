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
    }
}

fn main() {
    let range: Vec<u32> = (1..=100).collect();
    for (n, res) in range.iter().zip(
        [
            "Sun, Jan 01, 1900",
            "Mon, Jan 02, 1900",
            "Tue, Jan 03, 1900",
            "Wed, Jan 04, 1900",
            "Thu, Jan 05, 1900",
            "Fri, Jan 06, 1900",
            "Sat, Jan 07, 1900",
            "Sun, Jan 08, 1900",
            "Mon, Jan 09, 1900",
            "Tue, Jan 10, 1900",
            "Wed, Jan 11, 1900",
            "Thu, Jan 12, 1900",
            "Fri, Jan 13, 1900",
            "Sat, Jan 14, 1900",
            "Sun, Jan 15, 1900",
            "Mon, Jan 16, 1900",
            "Tue, Jan 17, 1900",
            "Wed, Jan 18, 1900",
            "Thu, Jan 19, 1900",
            "Fri, Jan 20, 1900",
            "Sat, Jan 21, 1900",
            "Sun, Jan 22, 1900",
            "Mon, Jan 23, 1900",
            "Tue, Jan 24, 1900",
            "Wed, Jan 25, 1900",
            "Thu, Jan 26, 1900",
            "Fri, Jan 27, 1900",
            "Sat, Jan 28, 1900",
            "Sun, Jan 29, 1900",
            "Mon, Jan 30, 1900",
            "Tue, Jan 31, 1900",
            "Wed, Feb 01, 1900",
            "Thu, Feb 02, 1900",
            "Fri, Feb 03, 1900",
            "Sat, Feb 04, 1900",
            "Sun, Feb 05, 1900",
            "Mon, Feb 06, 1900",
            "Tue, Feb 07, 1900",
            "Wed, Feb 08, 1900",
            "Thu, Feb 09, 1900",
            "Fri, Feb 10, 1900",
            "Sat, Feb 11, 1900",
            "Sun, Feb 12, 1900",
            "Mon, Feb 13, 1900",
            "Tue, Feb 14, 1900",
            "Wed, Feb 15, 1900",
            "Thu, Feb 16, 1900",
            "Fri, Feb 17, 1900",
            "Sat, Feb 18, 1900",
            "Sun, Feb 19, 1900",
            "Mon, Feb 20, 1900",
            "Tue, Feb 21, 1900",
            "Wed, Feb 22, 1900",
            "Thu, Feb 23, 1900",
            "Fri, Feb 24, 1900",
            "Sat, Feb 25, 1900",
            "Sun, Feb 26, 1900",
            "Mon, Feb 27, 1900",
            "Tue, Feb 28, 1900",
            "Wed, Feb 29, 1900",
            "Thu, Mar 01, 1900",
            "Fri, Mar 02, 1900",
            "Sat, Mar 03, 1900",
            "Sun, Mar 04, 1900",
            "Mon, Mar 05, 1900",
            "Tue, Mar 06, 1900",
            "Wed, Mar 07, 1900",
            "Thu, Mar 08, 1900",
            "Fri, Mar 09, 1900",
            "Sat, Mar 10, 1900",
            "Sun, Mar 11, 1900",
            "Mon, Mar 12, 1900",
            "Tue, Mar 13, 1900",
            "Wed, Mar 14, 1900",
            "Thu, Mar 15, 1900",
            "Fri, Mar 16, 1900",
            "Sat, Mar 17, 1900",
            "Sun, Mar 18, 1900",
            "Mon, Mar 19, 1900",
            "Tue, Mar 20, 1900",
            "Wed, Mar 21, 1900",
            "Thu, Mar 22, 1900",
            "Fri, Mar 23, 1900",
            "Sat, Mar 24, 1900",
            "Sun, Mar 25, 1900",
            "Mon, Mar 26, 1900",
            "Tue, Mar 27, 1900",
            "Wed, Mar 28, 1900",
            "Thu, Mar 29, 1900",
            "Fri, Mar 30, 1900",
            "Sat, Mar 31, 1900",
            "Sun, Apr 01, 1900",
            "Mon, Apr 02, 1900",
            "Tue, Apr 03, 1900",
            "Wed, Apr 04, 1900",
            "Thu, Apr 05, 1900",
            "Fri, Apr 06, 1900",
            "Sat, Apr 07, 1900",
            "Sun, Apr 08, 1900",
            "Mon, Apr 09, 1900",
            "Tue, Apr 10, 1900",
        ].iter()

    ) {
        let date_real = idate!(*n, false);
        let date_stupid = idate!(*n, true);

        let format = "%d, %m %0d, %Y";

        let stupid_date_string: &str =
            &date_stupid.format(format).to_string();

        println!("{}____{}____{}\n",
            //date_real.format(format),
            *res,
            stupid_date_string,
            (if *res != stupid_date_string {
                "ERROR"
            } else { "" }),
        );
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
            let date = idate!(n, true);
            println!("{}", date.format("%M %-d, %Y"));
        }
    }

    #[test]
    fn iter_2016() {
        let start = 42370;
        let end = 42735; 
        
        for n in start..=end {
            let date = idate!(n, true);
            println!("{}\t{}", date.format("%M %-d, %Y"), n);
        }
    }
    
    #[test]
    fn print_date_info() {
        let n = 42433; // Friday, March 4th, 2016
        let date = idate!(n, true);
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

    #[test]
    fn test_sheet() {
        let sheet: Vec<u32> = vec!(
            42369, 42765, 43728, 43769, 43770, 43776, 43805, 43839,
            43841, 43847, 43850, 43852, 43853, 43857, 43860, 43861,
            43909, 43913, 43920, 43934, 43935, 43937, 43950, 43951,
            43952, 43955, 43956, 43958, 43959, 43962, 43965, 43966,
            43969, 43973, 43976, 43979, 43980, 43982, 43983, 43986,
            43987, 43990, 43991, 43992, 43993, 43994, 43997, 43998,
            44000, 44001, 44002, 44004, 44005, 44006, 44007, 44008,
            44011, 44012, 44013, 44014, 44015, 44016, 44018, 44019,
            44020, 44021, 44022, 44025, 44026, 44027, 44028, 44029,
            44032, 44033, 44034, 44035, 44036, 44039, 44040, 44041,
            44042, 44043, 44046, 44047, 44048, 44049, 44050, 44053,
            44054, 44055, 44056, 44057, 44060, 44061, 44062, 44063,
            44064, 44067, 44068, 44069, 44070, 44071, 44074, 44075,
            44076, 44077, 44078, 44081, 44082, 44083, 44084, 44085,
            44088, 44089, 44090, 44091, 44092, 44095, 44096, 44097,
            44098, 44099, 44102, 44103, 44104, 44105, 44106, 44109,
            44110, 44111, 44112, 44113, 44116, 44117, 44118, 44119,
            44120, 44123, 44124, 44125, 44126, 44127, 44130, 44131,
            44132, 44133, 44134, 44135, 44137, 44138, 44139, 44140,
            44141, 44144, 44145, 44146, 44147, 44148, 44151, 44152,
            44153, 44154, 44155, 44158, 44159, 44160, 44161, 44162,
            44164, 44165, 44166, 44167, 44168, 44169, 44172, 44173,
            44174, 44175, 44176, 44179, 44180, 44181, 44183, 44186,
            44187, 44188, 44190, 44193, 44194, 44195, 44196, 44197,
            44200, 44202, 44203, 44204, 44207, 44208, 44209, 44210,
            44211, 44214, 44215, 44217, 44218, 44221, 44223, 44224,
            44225, 44227, 44228, 44229, 44230, 44231, 44232, 44236,
            44238, 44239, 44242, );

        for (index, value) in sheet.iter().enumerate() {
            print!("{}\t", value);
            let date = idate!(*value, true);
            print!("{}\n", date.format("%m %-d, %Y"));
        }
    }
}