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
    input.lines().map(|line| {
        match line.split_once(",") {
            Some((first, second)) => is_fully_contained(first, second),
            _ => panic!("Could not parse line: {:?}", line)
        }
    })
        .filter(|x| { *x })
        .count() as u32
}

pub(crate) fn overlapping_pairs(input: &str) -> u32 {
    input.lines().map(|line| {
        match line.split_once(",") {
            Some((first, second)) => overlaps(first, second),
            _ => panic!("Could not parse line: {:?}", line)
        }
    })
        .filter(|x| { *x })
        .count() as u32
}

fn overlaps(first: &str, second: &str) -> bool {
    let left_range = parse_range(first);
    let right_range = parse_range(second);
    return if left_range.1 < right_range.0 {
        false
    } else if left_range.0 > right_range.1 {
        false
    } else {
        true
    };
}

fn is_fully_contained(first: &str, second: &str) -> bool {
    let left_range = parse_range(first);
    let right_range = parse_range(second);
    return if left_range.0 >= right_range.0 && left_range.1 <= right_range.1 {
        true
    } else if left_range.0 <= right_range.0 && left_range.1 >= right_range.1 {
        true
    } else {
        false
    };
}

fn parse_range(range: &str) -> (u32, u32) {
    let (first, second) = range.split_once("-").unwrap();
    (first.parse().unwrap(), second.parse().unwrap())
}