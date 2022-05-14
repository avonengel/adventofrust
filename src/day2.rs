use regex::Regex;

pub fn position_depth_product(input: &str) -> u32 {
    let mut depth = 0;
    let mut position = 0;
    let pattern: Regex = Regex::new(r"(forward|down|up) (\d+)").unwrap();

    for capture in pattern.captures_iter(input) {
        let value = capture[2].parse::<u32>().unwrap();
        match &capture[1] {
            "forward" => position += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("encountered unknown command: {}", &capture[1]),
        }
    }
    depth * position
}

pub fn position_depth_product_with_aim(input: &str) -> u32 {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    let pattern: Regex = Regex::new(r"(forward|down|up) (\d+)").unwrap();

    for capture in pattern.captures_iter(input) {
        let value = capture[2].parse::<u32>().unwrap();
        match &capture[1] {
            "forward" => {
                position += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("encountered unknown command: {}", &capture[1]),
        }
    }
    depth * position
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
    "};

    #[test]
    fn it_computes_depth_position_product_of_sample() {
        assert_eq!(super::position_depth_product(SAMPLE_INPUT), 150)
    }

    #[test]
    fn it_computes_depth_position_product_of_input() {
        let contents = crate::read_file_content("src/day2/input");
        assert_eq!(super::position_depth_product(&contents), 1989014);
    }

    #[test]
    fn it_computes_depth_position_product_with_aim_of_sample() {
        assert_eq!(super::position_depth_product_with_aim(SAMPLE_INPUT), 900)
    }

    #[test]
    fn it_computes_depth_position_product_with_aim_of_input() {
        let contents = crate::read_file_content("src/day2/input");
        assert_eq!(
            super::position_depth_product_with_aim(&contents),
            2006917119
        );
    }
}
