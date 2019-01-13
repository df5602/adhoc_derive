use regex_syntax::ast::{parse::Parser, Error, ErrorKind};

/// Prefixes the names of explicitly numbered capture groups (e.g. (?P<0>)) with `__` to make them valid capture group names
pub fn replace_numbered_capture_groups(regex: &mut String) -> Result<(), Error> {
    loop {
        let mut parser = Parser::new();
        let error_offset = match parser.parse(&regex) {
            Ok(_) => return Ok(()),
            Err(e) => {
                if *e.kind() == ErrorKind::GroupNameInvalid {
                    e.span().start.offset
                } else {
                    // Other parse error, abort
                    return Err(e);
                }
            }
        };
        regex.insert_str(error_offset, "__");
    }
}

#[cfg(test)]
mod test_replace {
    use super::*;

    #[test]
    fn no_capture_groups() {
        let mut regex = String::from("foo");
        let original_regex = regex.clone();
        replace_numbered_capture_groups(&mut regex).unwrap();
        assert_eq!(original_regex, regex);
    }

    #[test]
    fn named_capture_group_not_numbered() {
        let mut regex = String::from(r"^(?P<id>\d+)$");
        let original_regex = regex.clone();
        replace_numbered_capture_groups(&mut regex).unwrap();
        assert_eq!(original_regex, regex);
    }

    #[test]
    fn named_capture_group_numbered() {
        let mut regex = String::from(r"^(?P<0>\d+)$");
        replace_numbered_capture_groups(&mut regex).unwrap();
        assert_eq!(r"^(?P<__0>\d+)$", regex);
    }

    #[test]
    fn named_capture_groups_multiple_numbered() {
        let mut regex = String::from(r"^(?P<0>\d+): (?P<1>\d+)$");
        replace_numbered_capture_groups(&mut regex).unwrap();
        assert_eq!(r"^(?P<__0>\d+): (?P<__1>\d+)$", regex);
    }

    #[test]
    fn named_capture_groups_mixed() {
        let mut regex = String::from(r"^(?P<0>\d+): (?P<a>\d+)$");
        replace_numbered_capture_groups(&mut regex).unwrap();
        assert_eq!(r"^(?P<__0>\d+): (?P<a>\d+)$", regex);
    }

    #[test]
    fn named_capture_groups_nested() {
        let mut regex = String::from(r"^(?P<2>(?P<0>\d+): (?P<a>\d+))$");
        replace_numbered_capture_groups(&mut regex).unwrap();
        assert_eq!(r"^(?P<__2>(?P<__0>\d+): (?P<a>\d+))$", regex);
    }

    #[test]
    fn named_capture_groups_fake_group() {
        let mut regex = String::from(r"^(?P<2>\(?P<0>\d+\): (?P<a>\d+))$");
        replace_numbered_capture_groups(&mut regex).unwrap();
        assert_eq!(r"^(?P<__2>\(?P<0>\d+\): (?P<a>\d+))$", regex);
    }
}
