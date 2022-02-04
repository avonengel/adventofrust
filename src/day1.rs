
#[allow(dead_code)]
fn count_increases(text: &str) -> u32 {
    let mut previous: u32 = u32::MAX;
    let mut increases = 0;
    for line in text.lines() {
        let current = line.parse::<u32>().unwrap();
        if previous < current {
            increases = increases + 1;
        }
        previous = current;
    }
    return increases;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use std::fs::File;
    use std::io::prelude::*;
    
    const SAMPLE_INPUT: &str = indoc! {"199
    200
    208
    210
    200
    207
    240
    269
    260
    263
    "};

    #[test]
    fn it_counts_increases() {
        assert_eq!(super::count_increases(SAMPLE_INPUT), 7)
    }

    #[test]
    fn it_counts_increases_from_file() {
        let mut file = File::open("src/day1/input").unwrap();
        let mut contents = String::new();
        assert!(file.read_to_string(&mut contents).is_ok());
        assert_eq!(super::count_increases(&contents), 1616);
    }
}