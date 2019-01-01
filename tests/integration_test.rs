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
}