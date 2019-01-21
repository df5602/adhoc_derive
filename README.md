# adhoc_derive &emsp; [![](http://meritbadge.herokuapp.com/adhoc_derive)](https://crates.io/crates/adhoc_derive)

Experimental: Derive FromStr impl based on regex provided via attribute

-----

### Usage
Add the following in your Cargo.toml:
```
[dependencies]
adhoc_derive = "0.1.1"
lazy_static = "1.2.0"
regex = "1.1.0"
```

Then, you can derive a `std::str::FromStr` impl as follows:
```
use adhoc_derive::FromStr;

#[derive(FromStr)]
#[adhoc(regex = r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)$")]
struct Rectangle {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

let rect: Rectangle = "#123 @ 3,2: 5x4".parse().unwrap();
assert_eq!(123, rect.id);
assert_eq!(3, rect.x);
assert_eq!(2, rect.y);
assert_eq!(5, rect.width);
assert_eq!(4, rect.height);
```

In general, each field of the struct needs to implement `std::str::FromStr` and each field identifier needs to correspond to a named capture group in the regex.

For enums, you need to annotate each variant with a regex. The first regex that matches (usually the only one) determines which variant is instantiated:
```
#[derive(Debug, PartialEq, FromStr)]
enum Expression {
    #[adhoc(regex = r"^$")]
    Empty,
    #[adhoc(regex = r"^(?P<0>\d+)$")]
    Number(i32),
    #[adhoc(regex = r"^(?P<a>\d+)\+(?P<b>\d+)$")]
    Sum(#[adhoc(construct_with = "a: i32 + b: i32")] i32),
    #[adhoc(regex = r"^(?P<a>\d+)-(?P<b>\d+)$")]
    Difference(#[adhoc(construct_with = "a: i32 - b: i32")] i32),
}

let empty: Expression = "".parse().unwrap();
assert_eq!(Expression::Empty, empty);

let number: Expression = "4".parse().unwrap();
assert_eq!(Expression::Number(4), number);

let sum: Expression = "8+15".parse().unwrap();
assert_eq!(Expression::Sum(23), sum);

let difference: Expression = "16-23".parse().unwrap();
assert_eq!(Expression::Difference(-7), difference);
```

For a more comprehensive guide, refer to [GUIDE.md](GUIDE.md)

### Limitations
This crate is experimental and has a lot of rough edges. In no particular order:
* Doesn't work with generic structs out of the box (it should work if you add the required trait bounds yourself)
* Not yet implemented: optional patterns, i.e. (...)? => `Option<...>`
* Not yet implemented: repeating patterns, i.e. (...)* => `Vec<...>`
* Error handling and especially reporting is basically non-existent
* Provided regex is not validated at compile-time
