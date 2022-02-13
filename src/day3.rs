use std::ops::{BitAnd, Shr};

pub fn gamma_rate(input: &str) -> i32 {
    let line_length = input.lines()
        .next()
        .map(|l| { l.len() })
        .expect("must contain at least one line");
    let mut one_bits = vec![0; line_length];
    let numbers = input.lines().map(|line| { u32::from_str_radix(line, 2).unwrap() });
    let mut count = 0;

    // TODO: clean up this ugly mess. Do we really have to parse to int? back and forth, even! :(
    for number in numbers {
        count += 1;
        // println!("number {:3}: {:0width$b}", count, number, width = line_length);
        // print!("{:12}", "");
        for idx in 0..line_length {
            if number.shr(idx).bitand(1u32) > 0 {
                one_bits[idx] += 1;
                // print!("+");
            } else {
                // print!(" ");
            }

        }
        // println!();
    }
    let r : String = one_bits.iter().rev().map(|ones| {
        if ones > &(count / 2) {
            "1"
        } else {
            "0"
        }
    }).collect();
    // println!("binary string is {}", r);
    i32::from_str_radix(&r, 2).unwrap()
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