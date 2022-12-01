#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn it_finds_most_calories() {
        assert_eq!(super::most_calories(SAMPLE_INPUT), 24_000)
    }
    #[test]
    fn it_finds_top_three_calories() {
        assert_eq!(super::top3_calories(SAMPLE_INPUT), 45_000)
    }
}

pub(crate) fn most_calories(input: &str) -> u32 {
    let mut result = 0;

    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            sum = 0
        } else {
            sum += line.parse::<u32>().unwrap();
        }
        if sum > result {
            result = sum;
        }
    }
    return result;
}

pub(crate) fn top3_calories(input: &str) -> u32 {
    let mut result = [0, 0, 0];

    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            for (idx, r) in result.iter().enumerate() {
                if r < &sum {
                    result[idx] = sum;
                    result.sort();
                    println!("{:?}", result);
                    break;
                }
            }
            sum = 0
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }
    for (idx, r) in result.iter().enumerate() {
        if r < &sum {
            result[idx] = sum;
            result.sort();
            println!("{:?}", result);
            break;
        }
    }
    return result.iter().sum();
}
