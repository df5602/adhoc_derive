extern crate adhoc_derive;

use adhoc_derive::FromStr;

#[test]
fn derive_enum_no_fields() {
    #[derive(Debug, PartialEq, FromStr)]
    enum Foo {
        #[adhoc(regex = "^bar$")]
        Bar,
        #[adhoc(regex = "^baz$")]
        Baz,
        #[adhoc(regex = "^quux$")]
        Quux,
    }

    let bar: Foo = "bar".parse().unwrap();
    assert_eq!(Foo::Bar, bar);
    let baz: Foo = "baz".parse().unwrap();
    assert_eq!(Foo::Baz, baz);
    let quux: Foo = "quux".parse().unwrap();
    assert_eq!(Foo::Quux, quux);
}

#[test]
fn derive_enum_unnamed_fields() {
    #[derive(Debug, PartialEq, FromStr)]
    enum Foo {
        #[adhoc(regex = r"^bar (?P<0>\d+)$")]
        Bar(u32),
        #[adhoc(regex = r"^baz (?P<0>\d+) (?P<1>\d+)$")]
        Baz(u32, u32),
    }

    let bar: Foo = "bar 15".parse().unwrap();
    assert_eq!(Foo::Bar(15), bar);
    let baz: Foo = "baz 4 8".parse().unwrap();
    assert_eq!(Foo::Baz(4, 8), baz);
}

#[test]
fn derive_enum_named_fields() {
    #[derive(Debug, PartialEq, FromStr)]
    enum Foo {
        #[adhoc(regex = r"^bar (?P<b>\d+)$")]
        Bar { b: u32 },
        #[adhoc(regex = r"^baz (?P<a>\d+) (?P<b>\d+)$")]
        Baz { a: u32, b: u32 },
    }

    let bar: Foo = "bar 15".parse().unwrap();
    assert_eq!(Foo::Bar { b: 15 }, bar);
    let baz: Foo = "baz 4 8".parse().unwrap();
    assert_eq!(Foo::Baz { a: 4, b: 8 }, baz);
}

#[test]
fn derive_enum_mixed_fields() {
    #[derive(Debug, PartialEq, FromStr)]
    enum Foo {
        #[adhoc(regex = r"^bar$")]
        Bar,
        #[adhoc(regex = r"^baz (?P<0>\d+)$")]
        Baz(u32),
        #[adhoc(regex = r"^quux (?P<a>\d+) (?P<b>\d+)$")]
        Quux { a: u32, b: u32 },
    }

    let bar: Foo = "bar".parse().unwrap();
    assert_eq!(Foo::Bar, bar);
    let baz: Foo = "baz 15".parse().unwrap();
    assert_eq!(Foo::Baz(15), baz);
    let quux: Foo = "quux 4 8".parse().unwrap();
    assert_eq!(Foo::Quux { a: 4, b: 8 }, quux);
}
