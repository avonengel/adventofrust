#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn parses_single_line() {
        assert_eq!(super::priority_sum(SAMPLE_INPUT.lines().next().unwrap()), 16)
    }
    #[test]
    fn parses_sample_input() {
        assert_eq!(super::priority_sum(SAMPLE_INPUT), 157)
    }

    #[test]
    fn computes_badge_priority_sum() {
        assert_eq!(super::badge_priority_sum(SAMPLE_INPUT), 70)
    }
}

pub(crate) fn priority_sum(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let priority_char = find_priority_char(line);
        sum += char_to_priority(priority_char);
    }
    sum
}

fn char_to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn find_priority_char(line: &str) -> char {
    let (left, right) = line.split_at(line.len() / 2);
    for left_char in left.chars() {
        if right.contains(left_char) {
            return left_char
        }
    }
    panic!("did not find priority char")
}

pub(crate) fn badge_priority_sum(input: &str) -> u32 {
    let mut sum = 0;
    let mut lines = input.lines();
    while let Some(first_line) = lines.next() {
        let second_line = lines.next().unwrap();
        let third_line = lines.next().unwrap();
        for c in first_line.chars() {
            if second_line.contains(c) && third_line.contains(c) {
                sum += char_to_priority(c);
                break
            }
        }
    }
    sum
}
