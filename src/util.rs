pub(crate) fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub(crate) fn last_part<'a>(value: &'a str) -> &'a str {
    let index = value.rfind('/').map_or_else(|| 0, |idx| idx + 1);
    &value[index..value.len()]
}

pub(crate) fn path_last_two_parts<'a>(value: &'a str) -> &'a str {
    let first_idx = value.rfind('/').unwrap_or(value.len());
    let second_idx = value[0..first_idx]
        .rfind('/')
        .map_or_else(|| 0, |idx| idx + 1);
    &value[second_idx..value.len()]
}

#[cfg(test)]
mod test {
    use super::last_part;
    use super::path_last_two_parts;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[case("/", "")]
    #[case("/a", "a")]
    #[case("a/b", "b")]
    fn last_part_test(#[case] value: &str, #[case] expected: &str) {
        assert_eq!(last_part(value), expected);
    }

    #[rstest]
    #[case("", "")]
    #[case("/", "/")]
    #[case("/a", "/a")]
    #[case("a/b", "a/b")]
    #[case("/a/b", "a/b")]
    #[case("c/a/b", "a/b")]
    fn last_last_two_parts_test(#[case] value: &str, #[case] expected: &str) {
        assert_eq!(path_last_two_parts(value), expected);
    }
}
