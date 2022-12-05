use regex::Regex;

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    "};

    #[test]
    fn can_move_single_items() {
        assert_eq!(super::crate_message(SAMPLE_INPUT), "CMZ")
    }
    #[test]
    fn can_move_single_items2() {
        assert_eq!(super::crate_message2(SAMPLE_INPUT), "MCD")
    }
}


pub(crate) fn crate_message(input: &str) -> String {
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instruction_index = input.find("move").unwrap();
    let (raw_stacks, raw_instructions) = input.split_at(instruction_index);
    let mut stacks = parse_stacks(raw_stacks);
    for instruction in raw_instructions.lines().filter(|l| { l.contains("move") }) {
        // dbg!(&instruction);
        let captures = move_re.captures(&instruction).unwrap();
        let count = &captures[1].parse::<usize>().unwrap();
        let from = &captures[2].parse::<usize>().unwrap();
        let to = &captures[3].parse::<usize>().unwrap();
        // dbg!(&instruction, &count, &from, &to, &stacks);
        for _ in 1..=*count {
            // dbg!("moving");
            let crate_str = stacks[*from - 1].pop().unwrap();
            stacks[*to - 1].push(crate_str);
        }
        // dbg!(&stacks);
    }
    stacks.iter().map(|vec| { *vec.last().unwrap() }).collect()
}

pub(crate) fn crate_message2(input: &str) -> String {
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instruction_index = input.find("move").unwrap();
    let (raw_stacks, raw_instructions) = input.split_at(instruction_index);
    let mut stacks = parse_stacks(raw_stacks);
    for instruction in raw_instructions.lines().filter(|l| { l.contains("move") }) {
        // dbg!(&instruction);
        let captures = move_re.captures(&instruction).unwrap();
        let count = &captures[1].parse::<usize>().unwrap();
        let from = &captures[2].parse::<usize>().unwrap();
        let to = &captures[3].parse::<usize>().unwrap();
        // dbg!(&instruction, &count, &from, &to, &stacks);

        // println!("{}", instruction);
        let i = stacks[*from - 1].len();

        // FIXME wtf .. such a mess!
        for _ in 0..*count {
            // dbg!(&stacks[*from - 1]);
            let val = stacks[*from - 1].remove(i -count);
            stacks[*to - 1].push(val);
        }
    }
    stacks.iter().map(|vec| { *vec.last().unwrap() }).collect()
}

fn parse_stacks(raw_stacks: &str) -> Vec<Vec<&str>> {
    let stack_count = raw_stacks.lines().map(str::len).max().unwrap() / 4 + 1;
    let mut stacks: Vec<Vec<&str>> = vec![Vec::new(); stack_count];
    for layer in raw_stacks.lines().filter(|l| { !l.is_empty() && l.contains("[") }) {
        let stack_count = layer.len() / 4 + 1;
        // dbg!(stack_count);
        for stack_idx in 0..stack_count {
            let crate_str = &layer[stack_idx * 4..stack_idx * 4 + 3];
            // dbg!(crate_str);
            if crate_str.contains("[") {
                stacks[stack_idx].insert(0, &crate_str[1..2]);
            }
        }
        // dbg!(&stacks);
    }
    stacks
}
