use std::ops::RangeInclusive;

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn finds_fully_contained_pairs() {
        assert_eq!(super::fully_contained_pairs(SAMPLE_INPUT), 2)
    }

    #[test]
    fn finds_overlapping_pairs() {
        assert_eq!(super::overlapping_pairs(SAMPLE_INPUT), 4)
    }
}

pub(crate) fn fully_contained_pairs(input: &str) -> u32 {
    map_count_assignments(input, is_fully_contained)
}

pub(crate) fn map_count_assignments(input: &str, predicate: fn(&str, &str) -> bool) -> u32 {
    input.lines().map(|line| {
        match line.split_once(',') {
            Some((first, second)) => predicate(first, second),
            _ => panic!("Could not parse line: {line:?}")
        }
    })
        .filter(|x| { *x })
        .count() as u32
}

pub(crate) fn overlapping_pairs(input: &str) -> u32 {
    map_count_assignments(input, overlaps)
}

fn overlaps(first: &str, second: &str) -> bool {
    let left_range = parse_range(first);
    let right_range = parse_range(second);
    !(left_range.end() < right_range.start() || left_range.start() > right_range.end())
}

fn is_fully_contained(first: &str, second: &str) -> bool {
    let left_range = parse_range(first);
    let right_range = parse_range(second);
    left_range.start() >= right_range.start() && left_range.end() <= right_range.end()
        || left_range.start() <= right_range.start() && left_range.end() >= right_range.end()
}

fn parse_range(range: &str) -> RangeInclusive<u32> {
    let (first, second) = range.split_once('-').unwrap();
    first.parse().unwrap()..=second.parse().unwrap()
}
