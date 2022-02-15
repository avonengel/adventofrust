use std::ops::Shl;

fn gamma_rate(input: &str) -> u32 {
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
            return 1u32.shl(i)
        }
        0
    }).sum()
}

fn epsilon_rate(input: &str) -> u32 {
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
        if ones <= &(count / 2) {
            return 1u32.shl(i)
        }
        0
    }).sum()
}


pub fn power_consumption(input: &str) -> u32 {
    epsilon_rate(input) * gamma_rate(input)
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

    #[test]
    fn test_epsilon_rate() {
        assert_eq!(0b01001, epsilon_rate(SAMPLE_INPUT))
    }

    #[test]
    fn test_power_consumption() {
        assert_eq!(198, power_consumption(SAMPLE_INPUT))
    }

    #[test]
    fn test_power_consumption_from_file() {
        let input = crate::read_file_content("src/day3/input");
        assert_eq!(3959450, power_consumption(&input));
    }
}