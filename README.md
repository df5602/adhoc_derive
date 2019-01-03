### adhoc_derive
-----
Experimental: Derive FromStr impl based on regex provided via attribute

### Usage
Add the following in your Cargo.toml:
```
[dependencies]
adhoc_derive = { git = "https://github.com/df5602/adhoc_derive" }
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
