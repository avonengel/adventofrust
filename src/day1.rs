#[cfg(test)]
mod tests {
    use indoc::indoc;
    const SAMPLE_INPUT: &str = indoc! {"199
    200
    208
    210
    200
    207
    240
    269
    260
    263
    "};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}