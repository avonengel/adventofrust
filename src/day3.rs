use std::ops::Shl;

pub fn gamma_rate(input: &str) -> i32 {
    let line_length = input.lines()
        .next()
        .map(|l| { l.len() })
        .expect("must contain at least one line");
    let mut one_bits = vec![0; line_length];
    let mut count = 0;

    for line in input.lines() {
        count += 1;
        for (idx, char) in line.chars().enumerate() {
            match char {
                '1' => one_bits[idx] += 1,
                '0' => continue,
                _ => {
                    panic!("Expected input to contain only '0' and '1', got {}", char)
                }
            }
        }
    }
    one_bits.iter().rev().enumerate().map(|(i, ones)| {
        if ones > &(count / 2) {
            return 1.shl(i)
        }
        0
    }).sum()

}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn test_gamma_rate() {
        assert_eq!(0b10110, gamma_rate(SAMPLE_INPUT))
    }
}