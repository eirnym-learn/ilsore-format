pub(crate) fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub(crate) trait LastPart {
    fn last_part(&self) -> &Self;
    fn last_two_parts(&self) -> &Self;
}

impl LastPart for str {
    fn last_part(&self) -> &str {
        let index = self.rfind('/').map_or_else(|| 0, |idx| idx + 1);
        &self[index..self.len()]
    }

    fn last_two_parts(&self) -> &str {
        let first_idx = self.rfind('/').unwrap_or(self.len());
        let second_idx = self[0..first_idx]
            .rfind('/')
            .map_or_else(|| 0, |idx| idx + 1);
        &self[second_idx..self.len()]
    }
}

#[cfg(test)]
mod test {
    use super::LastPart;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[case("/", "")]
    #[case("/a", "a")]
    #[case("a/b", "b")]
    fn last_part_test(#[case] value: &str, #[case] expected: &str) {
        assert_eq!(value.last_part(), expected);
    }

    #[rstest]
    #[case("", "")]
    #[case("/", "/")]
    #[case("/a", "/a")]
    #[case("a/b", "a/b")]
    #[case("/a/b", "a/b")]
    #[case("c/a/b", "a/b")]
    fn last_last_two_parts_test(#[case] value: &str, #[case] expected: &str) {
        assert_eq!(value.last_two_parts(), expected);
    }
}
