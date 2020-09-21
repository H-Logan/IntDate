#![allow(unused)]

pub mod dateinfo;
pub mod intdate;
pub mod month;
pub mod weekday;

pub use self::intdate::IntDate;

// Excel counts 1900 as a leap year, which is stupid.
// therefore, to make this compatible with Excel for
// January through March of 1900, $stupid must be true.

#[macro_export]
macro_rules! idate {
    ($num:expr, $is_stupid:expr) => {
        IntDate::new($num, $is_stupid).get_date_info()
    }
}

fn main() {
    let n = 42369;
    let date = idate!(n, true);
    println!("{}", date.format("%m %-d, %Y - %D"));
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