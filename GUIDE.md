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

### Tuple structs
For tuple structs the capture groups need to be explicitly numbered, where the number corresponds to the order of the fields:
```
#[derive(FromStr)]
#[adhoc(regex = r"^\((?P<0>\d+),(?P<1>\d+)\)$")]
struct Tuple(u32, u32);

let tuple: Tuple = "(12,13)".parse().unwrap();
assert_eq!(12, tuple.0);
assert_eq!(13, tuple.1);
```

## Using `construct_with` attribute to initialize fields
Sometimes it may be undesireable or impossible to add a custom `std::str::FromStr` implementation for a contained struct (the struct may be defined in a different crate, for example). Other times, you may want to pre-process the values extracted from the regex, before you initialize a field. In these cases it's also possible to use the `construct_with` attribute to provide an expression to initialize the field:

### Example: use `construct_with` to initialize nested struct
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
### Example: use `construct_with` to initialize array
```
#[derive(FromStr)]
#[adhoc(regex = r"^numbers: (?P<a>\d+), (?P<b>\d+), (?P<c>\d+), (?P<d>\d+)$")]
struct Array {
    #[adhoc(construct_with = "[a, b, c, d]")]
    arr: [u8; 4],
}

let a: Array = "numbers: 4, 8, 15, 16".parse().unwrap();
assert_eq!([4, 8, 15, 16], a.arr);
```
### Example: use `construct_with` to compute a value to initialize field
```
#[derive(FromStr)]
#[adhoc(regex = r"^sum from (?P<start>\d+) to (?P<stop>\d+)$")]
struct Sum {
    #[adhoc(construct_with = "(start: u32..stop: u32).sum()")]
    sum: u32,
}

let sum: Sum = "sum from 1 to 10".parse().unwrap();
assert_eq!(45, sum.sum);
```
### Notes
* Each "leaf identifier" (e.g. function arguments, but not e.g. function names) needs to correspond to a named capture group in the regex.
* This only works for somewhat "simple" expressions, e.g. array syntax, function calls, tuples, binary/unary operations, if/else expressions etc. More complex expressions, especially those that create new local bindings (e.g. loops, match expressions, closures, `let` statements in blocks, etc.), are not possible at the moment.
* Refer to [tests/construct_with.rs](https://github.com/df5602/adhoc_derive/blob/master/tests/construct_with.rs) for more examples of possible initializer expressions.

### Use type ascription syntax to help with type inference
Sometimes it's not possible to infer the receiver type from the given expression. In these cases, an identifier can be explicitly annotated with a type:
```
#[derive(FromStr)]
#[adhoc(regex = r"^(?P<a>\d+) \+ (?P<b>\d+)$")]
struct Sum {
    #[adhoc(construct_with = "a: u8 + b: u8")]
    sum: u8,
}

let sum: Sum = "12 + 15".parse().unwrap();
assert_eq!(27, sum.sum);
```

### Special case: `&str`
The implementation is depending on the fact that each receiver type implements `std::str::FromStr`. This is not the case for `&str`. If you run into the error "the trait `std::str::FromStr` is not implemented for `&str`", use type ascription to signal to the macro that the type is `&str`. In this case, the macro will not try to parse a `&str` from a `&str`. Instead, you can use the `&str` directly:
```
fn add_subject(subj: &str) -> String {
    let mut s = String::from("Hello, ");
    s.push_str(subj);
    s
}

#[derive(FromStr)]
#[adhoc(regex = r"^Hello: (?P<subject>.+)$")]
struct HelloSubject {
    #[adhoc(construct_with = "add_subject(subject: &str)")]
    s: String,
}

let hello: HelloSubject = "Hello: World".parse().unwrap();
assert_eq!("Hello, World", hello.s);
