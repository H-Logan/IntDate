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
