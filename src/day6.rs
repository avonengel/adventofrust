#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_start_packet_offset() {
        assert_eq!(super::unique_characters_offset(SAMPLE_INPUT, 4), 7)
    }

    #[test]
    fn test_more_examples1() {
        assert_eq!(super::unique_characters_offset("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    }

    #[test]
    fn test_more_examples2() {
        assert_eq!(super::unique_characters_offset("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    }

    #[test]
    fn test_more_examples3() {
        assert_eq!(super::unique_characters_offset("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    }

    #[test]
    fn test_more_examples4() {
        assert_eq!(super::unique_characters_offset("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }
}

pub(crate) fn unique_characters_offset(input: &str, unique_chars: usize) -> usize {
    input.chars().enumerate().scan(Vec::with_capacity(unique_chars), |state, (idx, c)| {
        // dbg!(&c, &state);

        if let Some(pos) = state.iter().position(|&ch| { ch == c }) {
            // *state = state.split_off(pos + 1)
            state.drain(..pos + 1);
        }
        state.push(c);
        // dbg!(&state);
        if state.len() == unique_chars {
            Some(idx + 1)
        } else {
            Some(0)
        }
    // }).inspect(|i| {
        // dbg!(i);
    }).find(|&x| {
        x > 0
    }).unwrap()
    // 0
}
