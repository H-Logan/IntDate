# IntDate
This crate converts integers to dates, similar to how it's done in Excel.

Ex. `41,946 => November 3rd, 2014`

# Adding library to your project
Simply add the following line to your `Cargo.toml` file:  
```
[dependencies]
intdate = { git = "https://github.com/H-Logan/IntDate" }
```

## Basic Usage
```Rust
extern crate intdate;
use intdate::IntDate;

fn main() {
    let num = 41_946; // Monday, November 3rd, 2014
    let int_date = IntDate::new(num, true);
    let date = int_date.get_date_info();
    
    println!("{}", date.format("%D, %M %0d, %Y"));
}
```

## With `idate!` macro
```Rust
#[macro_use]
extern crate intdate;
use intdate::IntDate;

fn main() {
    let num = 41_946; // Monday, November 3rd, 2014
    let date = idate!(num, true);
    
    println!("{}", date.format("%D, %M %0d, %Y"));
}

```

Output: `Monday, November 03, 2014`

---

## `DateInfo::format()`

In the example above, `date.format()` is used when printing the output  
from a `DateInfo` struct. Here are the expressions used for this function:  

|Expr|Output|
|---|---|
|`%Y`|Four-digit year (eg. `1999`)
|`%y`|Two-digit year (eg. `99`)
|`%M`|Full month name (eg. `October`)
|`%m`|Shortened month name (eg. `Oct`)
|`%0m`|Zero-padded month number (eg. `03`)
|`%-m`|Non-zero-padded month number (eg. `3`)
|`%D`|Full weekday name (eg. `Saturday`)
|`%d`|Shortened weekday name (eg. `Thu`)
|`%0d`|Zero-padded day of month (eg. `03`)
|`%-d`|Non-zero-padded day of month (eg. `3`)
|`%.d`|Suffixed day of month (eg. `16th`)
|`%j`|Non-zero-padded day of year (eg. `69`)
|`%0j`|Zero-padded day of year (eg. `069`)
