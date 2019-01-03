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

    let rect: Rectangle = "#123 @ 3,2: 5x4".parse::<Rectangle>().unwrap();
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

    let rect: OuterRectangle = "#123 @ 3,2: 5x4".parse::<OuterRectangle>().unwrap();
    assert_eq!(123, rect.id);
    assert_eq!(3, rect.rect.x);
    assert_eq!(2, rect.rect.y);
    assert_eq!(5, rect.rect.width);
    assert_eq!(4, rect.rect.height);
}

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

    let rect: OuterRectangle = "#123 @ 3,2: 5x4".parse::<OuterRectangle>().unwrap();
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

    let rect: Rectangle = "3,2: 5x4".parse::<Rectangle>().unwrap();
    assert_eq!(0, rect.id);
    assert_eq!(3, rect.x);
    assert_eq!(2, rect.y);
    assert_eq!(5, rect.width);
    assert_eq!(4, rect.height);
}
