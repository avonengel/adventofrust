use std::ops::Shl;

#[derive(Debug)]
struct BitField {
    one_bits: Vec<u32>,
    line_count: usize,
}

impl BitField {
    fn new(input: &str) -> BitField {
        let line_length = input.lines()
            .next()
            .map(|l| { l.len() })
            .expect("must contain at least one line");
        let mut one_bits = vec![0; line_length];
        let mut line_count = 0;

        for (idx, line) in input.lines().enumerate() {
            line_count = idx+1;
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
        BitField { one_bits, line_count }
    }

    fn gamma_rate(&self) -> u32 {
        self.one_bits.iter().rev().enumerate().map(|(i, ones)| {
            if ones > &(self.line_count as u32 / 2) {
                return 1u32.shl(i)
            }
            0
        }).sum()
    }

    fn epsilon_rate(&self) -> u32 {
        self.one_bits.iter().rev().enumerate().map(|(i, ones)| {
            if ones <= &(self.line_count as u32 / 2) {
                return 1u32.shl(i)
            }
            0
        }).sum()
    }

    fn power_consumption(&self) -> u32 {
        self.epsilon_rate() * self.gamma_rate()
    }
}

pub fn power_consumption(input: &str) -> u32 {
    let bit_field = BitField::new(input);
    bit_field.power_consumption()
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
        assert_eq!(0b10110, BitField::new(SAMPLE_INPUT).gamma_rate())
    }

    #[test]
    fn test_epsilon_rate() {
        assert_eq!(0b01001, BitField::new(SAMPLE_INPUT).epsilon_rate())
    }

    #[test]
    fn test_power_consumption() {
        assert_eq!(198, BitField::new(SAMPLE_INPUT).power_consumption())
    }

    #[test]
    fn test_power_consumption_from_file() {
        let input = crate::read_file_content("src/day3/input");
        assert_eq!(3959450, power_consumption(&input));
    }
}