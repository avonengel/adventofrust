#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2 "};

    #[test]
    fn part1_counts_start() {
        assert_eq!(super::part1(""), 1);
    }

    #[test]
    fn part1_moves_right() {
        assert_eq!(super::part1("R 3"), 3);
    }

    #[test]
    fn part1_moves_up() {
        assert_eq!(super::part1("U 3"), 3);
    }

    #[test]
    fn part1_moves_down() {
        assert_eq!(super::part1("D 3"), 3);
    }

    #[test]
    fn part1_moves_left() {
        assert_eq!(super::part1("L 3"), 3);
    }

    #[test]
    fn part1_moves_diagonal_up_right() {
        let s = SAMPLE_INPUT.lines().take(2).fold(String::new(), |a, b| a + b + "\n");
        assert_eq!(super::part1(&s), 7);
    }

    #[test]
    fn part1_moves_diagonal_up_left() {
        let s = SAMPLE_INPUT.lines().take(3).fold(String::new(), |a, b| a + b + "\n");
        assert_eq!(super::part1(&s), 9);
    }

    #[test]
    fn part1_moves_diagonal_down_right() {
        let s = SAMPLE_INPUT.lines().take(5).fold(String::new(), |a, b| a + b + "\n");
        assert_eq!(super::part1(&s), 10);
    }

    #[test]
    fn part1_moves_diagonal_left_down() {
        let s = SAMPLE_INPUT.lines().take(7).fold(String::new(), |a, b| a + b + "\n");
        assert_eq!(super::part1(&s), 13);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(SAMPLE_INPUT), 13);
    }


    const PART2_SAMPLE: &str = indoc! {"R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20"};

    #[test]
    fn part2() {
        assert_eq!(super::part2(PART2_SAMPLE), 36);
    }
}

pub(crate) fn part2(input: &str) -> usize {
    let mut knots = vec![(0, 0); 10];
    let mut visited = Vec::new();

    visited.push(knots.last().unwrap().to_owned());
    for instruction in input
        .lines()
        .map(str::trim)
        .filter(|l| { !l.is_empty() }) {
        let direction_str = &instruction[0..1];
        let step_dir = match direction_str {
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => panic!("unsupported direction: {:?}", direction_str)
        };
        // dbg!(&instruction);
        let steps = instruction[2..].parse::<usize>().unwrap();
        for _ in 0..steps {
            let head = (knots[0].0 + step_dir.0, knots[0].1 + step_dir.1);
            knots[0] = head;
            for i in 1..knots.len() {
                knots[i] = new_knot_position(&knots[i - 1], &knots[i]);
            }
            let tail = knots[knots.len() - 1];
            if !visited.contains(&tail) {
                visited.push(tail);
            }
        }
    }
    // dbg!(&visited);
    visited.len()
}

pub(crate) fn part1(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = Vec::new();
    visited.push(tail);
    for instruction in input
        .lines()
        .map(str::trim)
        .filter(|l| { !l.is_empty() }) {
        let direction_str = &instruction[0..1];
        let step_dir = match direction_str {
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => panic!("unsupported direction: {:?}", direction_str)
        };
        // dbg!(&instruction);
        let steps = instruction[2..].parse::<usize>().unwrap();
        for _ in 0..steps {
            head = (head.0 + step_dir.0, head.1 + step_dir.1);
            tail = new_knot_position(&head, &tail);
            if !visited.contains(&tail) {
                visited.push(tail);
            }
        }
    }
    // dbg!(&visited);
    visited.len()
}

fn new_knot_position(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let distance = (head.0 - tail.0, head.1 - tail.1);
    match distance {
        // head over tail
        (0, 0) => tail.clone(),
        // right
        (2, 0) => (tail.0 + 1, tail.1),
        (1, 0) => tail.clone(),
        // left
        (-2, 0) => (tail.0 - 1, tail.1),
        (-1, 0) => tail.clone(),
        // up
        (0, 2) => (tail.0, tail.1 + 1),
        (0, 1) => tail.clone(),
        // down
        (0, -2) => (tail.0, tail.1 - 1),
        (0, -1) => tail.clone(),
        // diagonal right up
        (1, 1) => tail.clone(),
        (1, 2) => (tail.0 + 1, tail.1 + 1),
        (2, 1) => (tail.0 + 1, tail.1 + 1),
        (2, 2) => (tail.0 + 1, tail.1 + 1),
        // diagonal left up
        (-1, 1) => tail.clone(),
        (-2, 1) => (tail.0 - 1, tail.1 + 1),
        (-1, 2) => (tail.0 - 1, tail.1 + 1),
        (-2, 2) => (tail.0 - 1, tail.1 + 1),
        // diagonal right down
        (1, -1) => tail.clone(),
        (2, -1) => (tail.0 + 1, tail.1 - 1),
        (1, -2) => (tail.0 + 1, tail.1 - 1),
        (2, -2) => (tail.0 + 1, tail.1 - 1),
        // diagonal left down
        (-1, -1) => tail.clone(),
        (-2, -1) => (tail.0 - 1, tail.1 - 1),
        (-1, -2) => (tail.0 - 1, tail.1 - 1),
        (-2, -2) => (tail.0 - 1, tail.1 - 1),
        _ => panic!("unsupported move: {:?}", distance),
    }
}
