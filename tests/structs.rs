extern crate adhoc_derive;

use adhoc_derive::FromStr;

#[test]
fn derive_standard_struct() {
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
}

#[test]
fn derive_nested_struct() {
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
}

#[test]
fn derive_tuple_struct() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^(?P<0>\d+) m/s$")]
    struct Velocity(u32);

    let vel: Velocity = "25 m/s".parse().unwrap();
    assert_eq!(25, vel.0);
}

#[test]
fn derive_tuple_struct_multiple_fields() {
    #[derive(FromStr)]
    #[adhoc(regex = r"^\((?P<0>\d+),(?P<1>\d+)\)$")]
    struct Tuple(u32, u32);

    let tuple: Tuple = "(12,13)".parse().unwrap();
    assert_eq!(12, tuple.0);
    assert_eq!(13, tuple.1);
}
