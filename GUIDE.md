## Structs
Derive a `std::str::FromStr` impl as follows:
```
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
    
Each field needs to implement `std::str::FromStr` and each field identifier needs to correspond to a named capture group in the regex. Optional or repeating patterns in the regex are not supported yet.

### Nested structs
This also works recursively, e.g.:
```
#[derive(FromStr)]
#[adhoc(regex = r"^(?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)$")]
struct InnerRectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(FromStr)]
#[adhoc(regex = r"^#(?P<id>\d+) @ (?P<rect>.+)$")]
struct OuterRectangle {
    id: usize,
    rect: InnerRectangle,
}

let rect: OuterRectangle = "#123 @ 3,2: 5x4".parse().unwrap();
assert_eq!(123, rect.id);
assert_eq!(3, rect.rect.x);
assert_eq!(2, rect.rect.y);
assert_eq!(5, rect.rect.width);
assert_eq!(4, rect.rect.height);
```

### Using constructor functions to initialize fields
Sometimes it may be undesireable or impossible to add a custom `std::str::FromStr` implementation for a contained struct (the struct may be defined in a different crate, for example). In these cases it's also possible to use the `construct_with` attribute to provide a function to initialize the field:
```
struct InnerRectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl InnerRectangle {
    fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(FromStr)]
#[adhoc(regex = r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)$")]
struct OuterRectangle {
    id: usize,
    #[adhoc(construct_with = "InnerRectangle::new(x, y, width, height)")]
    rect: InnerRectangle,
}

let rect: OuterRectangle = "#123 @ 3,2: 5x4".parse().unwrap();
assert_eq!(123, rect.id);
assert_eq!(3, rect.rect.x);
assert_eq!(2, rect.rect.y);
assert_eq!(5, rect.rect.width);
assert_eq!(4, rect.rect.height);
```
Notes:
* The `construct_with` attribute must be a valid function call expression.
* Arguments can only be identifiers, e.g. the following don't work:
  * `#[adhoc(construct_with = "some_function(x / 2, y / 2)")]`
  * `#[adhoc(construct_with = "some_function(Some(x), y)")]`
* Each argument needs to correspond to a named capture group in the regex.
* It's also possible to provide a function without arguments, e.g.:
  * `#[adhoc(construct_with = "Default::default()")]`
