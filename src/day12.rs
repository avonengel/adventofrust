use std::cmp::min;

use matrix::Size;
use matrix::format::Conventional;

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use matrix::Size;

    const SAMPLE_INPUT: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    fn parses_input_matrix() {
        let height_map = super::parse_input(SAMPLE_INPUT);
        // dbg!(&height_map);
        assert_eq!(height_map.start, (0, 0));
        assert_eq!(height_map.end, (2, 5));
        assert_eq!(height_map.map.dimensions(), (5, 8));
        assert_eq!(height_map.map.rows, 5);
        assert_eq!(height_map.map.columns, 8);
        assert_eq!(height_map.map[(0, 0)], "abcdefghijklmnopqrstuvwxyz".find('a').unwrap());
        assert_eq!(height_map.map[(0, 1)], "abcdefghijklmnopqrstuvwxyz".find('a').unwrap());
        assert_eq!(height_map.map[(0, 2)], "abcdefghijklmnopqrstuvwxyz".find('b').unwrap());

        assert_eq!(height_map.map[(0, 7)], "abcdefghijklmnopqrstuvwxyz".find('m').unwrap());
        assert_eq!(height_map.map[(2, 5)], "abcdefghijklmnopqrstuvwxyz".find('z').unwrap());
        assert_eq!(height_map.map[(4, 7)], "abcdefghijklmnopqrstuvwxyz".find('i').unwrap());
    }

    #[test]
    fn shortest_path() {
        let mut height_map = super::parse_input(SAMPLE_INPUT);
        assert_eq!(height_map.shortest_path_form_start((0, 1)), 1);
        assert_eq!(height_map.shortest_path_form_start((1, 1)), 2);
        assert_eq!(height_map.shortest_path_form_start(height_map.end), 31);
    }

    #[test]
    fn shortest_path_anywhere() {
        let mut height_map = super::parse_input(SAMPLE_INPUT);
        assert_eq!(height_map.shortest_path_anywhere(height_map.end), 29);
    }
}

pub(crate) fn steps_to_signal(input: &str) -> u32 {
    let mut height_map = parse_input(input);
    height_map.shortest_path_form_start(height_map.end)
}

pub(crate) fn shortest_hike(input: &str) -> u32 {
    let mut height_map = parse_input(input);
    height_map.shortest_path_anywhere(height_map.end)
}

#[derive(Debug)]
struct HeightMap {
    start: (usize, usize),
    end: (usize, usize),
    map: Conventional<usize>,
    costs: Conventional<u32>,
}

impl HeightMap {
    pub(crate) fn shortest_path_form_start(&mut self, target: (usize, usize)) -> u32 {
        self.shortest_path(target)
    }

    fn shortest_path_anywhere(&mut self, target: (usize, usize)) -> u32 {
        for row in 0..self.map.rows {
            for col in 0..self.map.columns {
                if self.height((row, col)) == 0 {
                    self.costs[(row, col)] = 0;
                }
            }
        }
        self.shortest_path(target)
    }
    fn shortest_path(&mut self, target: (usize, usize)) -> u32 {
        self.costs[self.start] = 0;
        loop {
            let mut changed = false;
            // println!("================================================");
            // for row in 0..self.map.rows {
            //     for col in 0..self.map.columns {
            //         print!("{:>2} ", self.costs[(row, col)]);
            //     }
            //     println!();
            // }
            for row in 0..self.map.rows {
                for col in 0..self.map.columns {
                    // dbg!((row, col));
                    let position = (row, col);
                    let new_cost = self.compute_cost(position);
                    if new_cost != self.costs[position] {
                        changed = true;
                        self.costs[position] = new_cost;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        self.costs[target]
    }
    fn compute_cost(&self, position: (usize, usize)) -> u32 {
        let mut result = self.costs[position];
        let (row, column) = position;
        // print!("cost at {:?}: {}", position, result);
        if row > 0 && self.costs[(row - 1, column)] < u32::MAX
            && self.height((row - 1, column)) >= self.height(position) - 1 {
            result = min(result, 1 + self.costs[(row - 1, column)]);
            // print!("; north({})", result);
        }
        if column > 0 && self.costs[(row, column - 1)] < u32::MAX
            && self.height((row, column - 1)) >= self.height(position) - 1 {
            result = min(result, 1 + self.costs[(row, column - 1)]);
            // print!("; west({})", result);
        }
        let south = (row + 1, column);
        if row + 1 < self.costs.rows && self.costs[south] < u32::MAX
            && self.height(south) >= self.height(position) - 1 {
            result = min(result, 1 + self.costs[south]);
            // print!("; south({})", result);
        }
        let east = (row, column + 1);
        if column + 1 < self.costs.columns && self.costs[east] < u32::MAX
            && self.height(east) >= self.height(position) - 1 {
            result = min(result, 1 + self.costs[east]);
            // print!("; east({})", result);
        }
        // println!(", final {}", result);
        result
    }

    fn height(&self, position: (usize, usize)) -> isize {
        self.map[position] as isize
    }
}


fn parse_input(input: &str) -> HeightMap {
    let mut lines = input.lines().peekable();
    let cols = lines.peek().map_or(0, |l| { l.len() });
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut matrix = Conventional::new((input.lines().count(), cols));
    input.chars().filter(|&chr| { chr != '\n' }).enumerate().for_each(|(c_idx, chr)| {
        let position = (c_idx / cols, c_idx % cols);
        if let Some(height) = "abcdefghijklmnopqrstuvwxyz".find(chr) {
            matrix[position] = height;
        } else if chr == 'S' {
            start = position;
            matrix[position] = "abcdefghijklmnopqrstuvwxyz".find('a').unwrap();
        } else if chr == 'E' {
            end = position;
            matrix[position] = "abcdefghijklmnopqrstuvwxyz".find('z').unwrap();
        } else {
            panic!("Unknown char found {}", chr);
        }
    });
    let mut cost = Conventional::new(matrix.dimensions());
    cost.fill(u32::MAX);
    HeightMap {
        start,
        end,
        map: matrix,
        costs: cost,
    }
}
