pub fn count_increases(text: &str) -> u32 {
    let mut previous: u32 = u32::MAX;
    let mut increases = 0;
    for line in text.lines() {
        let current = line.parse::<u32>().unwrap();
        if previous < current {
            increases += 1;
        }
        previous = current;
    }
    increases
}

pub fn count_sliding_window_increases(text: &str) -> u32 {
    let mut previous: u32 = u32::MAX;
    let mut increases = 0;
    let numbers: Vec<u32> = text
        .lines()
        .map(|x| -> u32 { x.parse::<u32>().unwrap() })
        .collect();
    for slice in numbers.windows(3) {
        let current = slice.iter().sum();
        if previous < current {
            increases += 1;
        }
        previous = current;
    }
    increases
}

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
    fn it_counts_increases() {
        assert_eq!(super::count_increases(SAMPLE_INPUT), 7)
    }

    #[test]
    fn it_counts_increases_from_file() {
        let contents = crate::read_file_content("src/day1/input");
        assert_eq!(super::count_increases(&contents), 1616);
    }

    #[test]
    fn it_counts_sliding_window_increases() {
        assert_eq!(super::count_sliding_window_increases(SAMPLE_INPUT), 5)
    }

    #[test]
    fn it_counts_sliding_window_increases_from_file() {
        let contents = crate::read_file_content("src/day1/input");
        assert_eq!(super::count_sliding_window_increases(&contents), 1645)
    }
}
