extern crate adhoc_derive;

use adhoc_derive::FromStr;

#[test]
fn smoke_test() {
    #[derive(Default, FromStr)]
    #[adhoc(regex = r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)$")]
    struct Rectangle {
        id: usize,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    }

    let rect: Rectangle = "#123 @ 3,2: 5x4".parse::<Rectangle>().unwrap();
    assert_eq!(123, rect.id);
    assert_eq!(3, rect.x);
    assert_eq!(2, rect.y);
    assert_eq!(5, rect.width);
    assert_eq!(4, rect.height);
}