use std::ops::Shl;

#[derive(Debug)]
pub struct BitField {
    one_bits: Vec<u32>,
    line_count: usize,
    input: String,
}

enum BitCriteria {
    LeastCommon,
    MostCommon,
}

impl BitField {
    pub fn new(input: &str) -> BitField {
        let line_length = input
            .lines()
            .next()
            .map(|l| l.len())
            .expect("must contain at least one line");
        let mut one_bits = vec![0; line_length];
        let mut line_count = 0;

        for (idx, line) in input.lines().enumerate() {
            line_count = idx + 1;
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
        BitField {
            one_bits,
            line_count,
            input: input.to_string(),
        }
    }

    fn gamma_rate(&self) -> u32 {
        self.one_bits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, ones)| {
                if ones > &(self.line_count as u32 / 2) {
                    return 1u32.shl(i);
                }
                0
            })
            .sum()
    }

    fn epsilon_rate(&self) -> u32 {
        self.one_bits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, ones)| {
                if ones <= &(self.line_count as u32 / 2) {
                    return 1u32.shl(i);
                }
                0
            })
            .sum()
    }

    pub fn power_consumption(&self) -> u32 {
        self.epsilon_rate() * self.gamma_rate()
    }

    fn oxygen_generator_rating(&self) -> u32 {
        self.filter_by_bit_criteria(BitCriteria::MostCommon)
    }

    fn co2_scrubber_rating(&self) -> u32 {
        self.filter_by_bit_criteria(BitCriteria::LeastCommon)
    }

    fn filter_by_bit_criteria(&self, criterion: BitCriteria) -> u32 {
        let mut candidates = self.input.lines().collect::<Vec<&str>>();
        for (pos, _) in self.one_bits.iter().enumerate() {
            let ones_count = count_ones_at(&candidates, pos);

            let bit_state = match criterion {
                BitCriteria::MostCommon => {
                    if ones_count < (candidates.len() as f64 / 2f64).ceil() as usize {
                        '0'
                    } else {
                        '1'
                    }
                }
                BitCriteria::LeastCommon => {
                    if ones_count >= (candidates.len() as f64 / 2f64).ceil() as usize {
                        '0'
                    } else {
                        '1'
                    }
                }
            };

            candidates
                .retain(|&line| line.chars().nth(pos).unwrap() == bit_state);
            if candidates.len() == 1 {
                return u32::from_str_radix(candidates[0], 2).unwrap();
            }
        }
        panic!("did not reduce candidates to just 1")
    }

    pub fn life_support_rating(&self) -> u32 {
        self.oxygen_generator_rating() * self.co2_scrubber_rating()
    }
}

fn count_ones_at(candidates: &[&str], position: usize) -> usize {
    candidates
        .iter()
        .filter(|&line| line.chars().nth(position).unwrap() == '1')
        .count()
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
        let bit_field = BitField::new(&input);
        assert_eq!(3959450, bit_field.power_consumption());
    }

    #[test]
    fn test_life_support_rating_from_file() {
        let input = crate::read_file_content("src/day3/input");
        let bit_field = BitField::new(&input);
        assert_eq!(7440311, bit_field.life_support_rating());
    }

    #[test]
    fn test_oxygen_generator_rating() {
        assert_eq!(
            0b10111,
            BitField::new(SAMPLE_INPUT).oxygen_generator_rating()
        )
    }

    #[test]
    fn test_co2_scrubber_rating() {
        assert_eq!(0b01010, BitField::new(SAMPLE_INPUT).co2_scrubber_rating())
    }

    #[test]
    fn test_life_support_rating() {
        assert_eq!(230, BitField::new(SAMPLE_INPUT).life_support_rating())
    }
}
