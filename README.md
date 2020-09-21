# DISCLAIMER:
This branch of `IntDate` is used for testing possibly breaking features or
changes to the library. This contains code that will likely not compile or run.

# IntDate
This crate converts integers to dates, similar to how it's done in Excel.

Ex. `42,432 => March 3rd, 2016`

## Basic Usage
```Rust
fn main() {
    let num = 42_433; // Friday, March 4th, 2016
    let int_date = IntDate::new(num);
    let date = int_date.get_date_info();
    
    println!("{}", date.format("ISO 8601 Standard Date Format: %Y-%0m-%0d"));
}
```

## With `idate!` macro
```Rust
fn main() {
    let num = 42_433; // Friday, March 4th, 2016
    let date = idate!(num);
    
    println!("{}", date.format("ISO 8601 Standard Date Format: %Y-%0m-%0d"));
}
```

Output: `ISO 8601 Standard Date Format: 2016-03-03`
