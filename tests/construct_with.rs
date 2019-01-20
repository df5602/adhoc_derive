extern crate adhoc_derive;

use adhoc_derive::FromStr;

#[test]
fn derive_nested_struct_construct_with() {
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
}

#[test]
fn derive_struct_with_default_initialization() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^(?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)$")]
    struct Rectangle {
        #[adhoc(construct_with = "Default::default()")]
        id: usize,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    }

    let rect: Rectangle = "3,2: 5x4".parse().unwrap();
    assert_eq!(0, rect.id);
    assert_eq!(3, rect.x);
    assert_eq!(2, rect.y);
    assert_eq!(5, rect.width);
    assert_eq!(4, rect.height);
}

#[test]
fn construct_with_method_call() {
    struct Adder;

    impl Adder {
        fn add(&self, a: u8, b: u8) -> u8 {
            a + b
        }
    }

    static ADDER: Adder = Adder;

    #[derive(FromStr)]
    #[adhoc(regex = r"^add one to (?P<a>\d+)$")]
    struct AddOne {
        #[adhoc(construct_with = "ADDER.add(a, 1)")]
        num: u8,
    }

    let a: AddOne = "add one to 15".parse().unwrap();
    assert_eq!(16, a.num);
}

#[test]
fn construct_with_arrays() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^numbers: (?P<a>\d+), (?P<b>\d+), (?P<c>\d+), (?P<d>\d+)$")]
    struct Array {
        #[adhoc(construct_with = "[a, b, c, d]")]
        arr: [u8; 4],
    }

    let a: Array = "numbers: 4, 8, 15, 16".parse().unwrap();
    assert_eq!([4, 8, 15, 16], a.arr);
}

#[test]
fn construct_with_arrays_nested_function_calls() {
    fn add(a: u8, b: u8) -> u8 {
        a + b
    }

    #[derive(FromStr)]
    #[adhoc(regex = r"^numbers: (?P<a>\d+), (?P<b>\d+), (?P<c>\d+), (?P<d>\d+)$")]
    struct Array {
        #[adhoc(construct_with = "[add(a, b), add(c, d)]")]
        arr: [u8; 2],
    }

    let a: Array = "numbers: 4, 8, 15, 16".parse().unwrap();
    assert_eq!([12, 31], a.arr);
}

#[test]
fn construct_with_tuple() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^(?P<a>\d+)/(?P<b>\d+)$")]
    struct Tuple {
        #[adhoc(construct_with = "(a, b)")]
        tup: (u8, u8),
    }

    let t: Tuple = "15/16".parse().unwrap();
    assert_eq!((15, 16), t.tup);
}

#[test]
fn construct_with_binary_expr() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^(?P<a>\d+) \+ (?P<b>\d+)$")]
    struct Binary {
        #[adhoc(construct_with = "a: u8 + b: u8")]
        sum: u8,
    }

    let bin: Binary = "12 + 15".parse().unwrap();
    assert_eq!(27, bin.sum);
}

#[test]
fn construct_with_unary_expr() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^minus (?P<a>\d+)$")]
    struct Unary {
        #[adhoc(construct_with = "-a")]
        negated: i8,
    }

    let unary: Unary = "minus 50".parse().unwrap();
    assert_eq!(-50, unary.negated);
}

#[test]
fn construct_with_literal_1() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^add one to (?P<a>\d+)$")]
    struct AddOne {
        #[adhoc(construct_with = "a: u8 + 1")]
        num: u8,
    }

    let add_one: AddOne = "add one to 15".parse().unwrap();
    assert_eq!(16, add_one.num);
}

#[test]
fn construct_with_literal_2() {
    fn add(a: u8, b: u8) -> u8 {
        a + b
    }

    #[derive(FromStr)]
    #[adhoc(regex = r"^add one to (?P<a>\d+)$")]
    struct AddOne {
        #[adhoc(construct_with = "add(a, 1)")]
        num: u8,
    }

    let add_one: AddOne = "add one to 15".parse().unwrap();
    assert_eq!(16, add_one.num);
}

#[test]
fn construct_with_cast() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^number is (?P<a>\d+)$")]
    struct Cast {
        #[adhoc(construct_with = "a: u8 as i8")]
        cast: i8,
    }

    let cast: Cast = "number is 15".parse().unwrap();
    assert_eq!(15, cast.cast);
}

#[test]
fn construct_with_if_else() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^maximum of (?P<a>\d+) and (?P<b>\d+)$")]
    struct Max {
        #[adhoc(construct_with = "if a: u8 > b: u8 { a } else { b }")]
        max: u8,
    }

    let max: Max = "maximum of 23 and 42".parse().unwrap();
    assert_eq!(42, max.max);
}

#[test]
fn construct_with_if_else_bool() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^maybe (?P<a>true|false)\?$")]
    struct Maybe {
        #[adhoc(construct_with = "if a { true } else { false }")]
        val: bool,
    }

    let maybe: Maybe = "maybe true?".parse().unwrap();
    assert_eq!(true, maybe.val);
}

#[test]
fn construct_with_if_else_option() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^maybe (?P<a>-?\d+)\?$")]
    struct Maybe {
        #[adhoc(construct_with = "if a: i8 > 0 { Some(a) } else { None }")]
        val: Option<u8>,
    }

    let maybe: Maybe = "maybe 15?".parse().unwrap();
    assert_eq!(Some(15), maybe.val);
    let or_not: Maybe = "maybe -26?".parse().unwrap();
    assert_eq!(None, or_not.val);
}

#[test]
fn construct_with_block() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^number is (?P<a>\d+)$")]
    struct Number {
        #[adhoc(construct_with = "{ a }")]
        num: u32,
    }

    let number: Number = "number is 6".parse().unwrap();
    assert_eq!(6, number.num);
}

#[test]
fn construct_with_range() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^sum from (?P<start>\d+) to (?P<stop>\d+)$")]
    struct Sum {
        #[adhoc(construct_with = "(start: u32..stop: u32).sum()")]
        sum: u32,
    }

    let sum: Sum = "sum from 1 to 10".parse().unwrap();
    assert_eq!(45, sum.sum);
}

#[test]
fn construct_with_ref() {
    fn add(a: &u8, b: &u8) -> u8 {
        *a + *b
    }

    #[derive(FromStr)]
    #[adhoc(regex = r"^(?P<a>\d+) \+ (?P<b>\d+)$")]
    struct Add {
        #[adhoc(construct_with = "add(&a, &b)")]
        num: u8,
    }

    let a: Add = "1 + 2".parse().unwrap();
    assert_eq!(3, a.num);
}

#[test]
fn construct_with_struct_colons() {
    struct Inner {
        a: u8,
        b: u8,
    }

    #[derive(FromStr)]
    #[adhoc(regex = r"^Inner: (?P<a>\d+), (?P<b>\d+); Outer: (?P<c>\d+)$")]
    struct Outer {
        #[adhoc(construct_with = "Inner { a: a, b: b }")]
        inner: Inner,
        c: u8,
    }

    let outer: Outer = "Inner: 3, 4; Outer: 5".parse().unwrap();
    assert_eq!(3, outer.inner.a);
    assert_eq!(4, outer.inner.b);
    assert_eq!(5, outer.c);
}

#[test]
fn construct_with_struct_no_colons() {
    struct Inner {
        a: u8,
        b: u8,
    }

    #[derive(FromStr)]
    #[adhoc(regex = r"^Inner: (?P<a>\d+), (?P<b>\d+); Outer: (?P<c>\d+)$")]
    struct Outer {
        #[adhoc(construct_with = "Inner { a, b }")]
        inner: Inner,
        c: u8,
    }

    let outer: Outer = "Inner: 3, 4; Outer: 5".parse().unwrap();
    assert_eq!(3, outer.inner.a);
    assert_eq!(4, outer.inner.b);
    assert_eq!(5, outer.c);
}

#[test]
fn construct_with_struct_rest() {
    #[derive(Default)]
    struct Inner {
        a: u8,
        b: u8,
        d: u8,
    }

    #[derive(FromStr)]
    #[adhoc(regex = r"^Inner: (?P<a>\d+), (?P<b>\d+); Outer: (?P<c>\d+)$")]
    struct Outer {
        #[adhoc(construct_with = "Inner { a, b, ..Default::default() }")]
        inner: Inner,
        c: u8,
    }

    let outer: Outer = "Inner: 3, 4; Outer: 5".parse().unwrap();
    assert_eq!(3, outer.inner.a);
    assert_eq!(4, outer.inner.b);
    assert_eq!(5, outer.c);
    assert_eq!(0, outer.inner.d);
}

#[test]
fn construct_with_paren() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^number is (?P<a>\d+)$")]
    struct Number {
        #[adhoc(construct_with = "( a )")]
        num: u32,
    }

    let number: Number = "number is 6".parse().unwrap();
    assert_eq!(6, number.num);
}

#[test]
fn construct_with_try_expr() {
    fn try_parse(input: u8) -> Result<u8, String> {
        if input > 127 {
            return Err(String::from("too large"));
        }
        Ok(input)
    }

    #[derive(FromStr)]
    #[adhoc(regex = r"^number is (?P<a>\d+)$")]
    struct Try {
        #[adhoc(construct_with = "try_parse(a)?")]
        inner: u8,
    }

    let t: Try = "number is 65".parse().unwrap();
    assert_eq!(65, t.inner);
}

#[test]
fn construct_with_ref_str() {
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
}

#[test]
fn construct_with_tuple_struct() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^(?P<vel>\d+) (?P<unit>.+)$")]
    struct Velocity(
        #[adhoc(
            construct_with = r#"if unit: &str == "m/s" { vel } else if unit: &str == "km/h" { (vel: f32 / 3.6) as u32 } else { 0 } "#
        )]
        u32,
    );

    let vel_ms: Velocity = "50 m/s".parse().unwrap();
    let vel_kmh: Velocity = "50 km/h".parse().unwrap();
    assert_eq!(50, vel_ms.0);
    assert_eq!(13, vel_kmh.0);
}

#[test]
fn construct_with_tuple_struct_mixed() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^(?P<0>\d+): (?P<a>\d+) \+ (?P<b>\d+)$")]
    struct Foo(u32, #[adhoc(construct_with = "a: u32 + b: u32")] u32);

    let foo: Foo = "2: 4 + 5".parse().unwrap();
    assert_eq!(2, foo.0);
    assert_eq!(9, foo.1);
}

#[test]
fn construct_with_enum() {
    #[derive(Debug, PartialEq, FromStr)]
    enum Expression {
        #[adhoc(regex = "^$")]
        Empty,
        #[adhoc(regex = r"^(?P<0>\d+)$")]
        Number(u32),
        #[adhoc(regex = r"^(?P<a>\d+)\+(?P<b>\d+)$")]
        Sum(#[adhoc(construct_with = "a: u32 + b: u32")] u32),
        #[adhoc(regex = r"^max\((?P<op1>\d+),(?P<op2>\d+)\)$")]
        Max {
            op1: u32,
            op2: u32,
            #[adhoc(construct_with = "if op1: u32 > op2: u32 { op1 } else { op2 }")]
            max: u32,
        },
    }

    let empty: Expression = "".parse().unwrap();
    assert_eq!(Expression::Empty, empty);

    let number: Expression = "143".parse().unwrap();
    assert_eq!(Expression::Number(143), number);

    let sum: Expression = "15+16".parse().unwrap();
    assert_eq!(Expression::Sum(31), sum);

    let max: Expression = "max(3,4)".parse().unwrap();
    assert_eq!(
        Expression::Max {
            op1: 3,
            op2: 4,
            max: 4
        },
        max
    );
}
