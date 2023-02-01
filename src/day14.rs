use std::cmp::min;
use std::fmt::{Display, Formatter, Write};
use std::ops::{RangeInclusive};
use itertools::Itertools;
use matrix::format::compressed::Variant;
use matrix::prelude::*;

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const SAMPLE_INPUT: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    fn parses_input() {
        let map = parse_scan(SAMPLE_INPUT);
        assert_eq!((494..=503, 0..=9), map.dim());
    }

    #[test]
    fn displays_nicely() {
        const EXPECTED: &str = "  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.
6 ..###...#.
7 ........#.
8 ........#.
9 #########.
";
        let map = parse_scan(SAMPLE_INPUT);
        assert_eq!(EXPECTED, format!("{map}"));
    }

    #[test]
    fn test_sand_falls_down_vertically() {
        let mut map = parse_scan(SAMPLE_INPUT);
        map.drop_sand();

        const EXPECTED: &str = "  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.
6 ..###...#.
7 ........#.
8 ......o.#.
9 #########.
";
        assert_eq!(EXPECTED, format!("{map}"));
    }

    #[test]
    fn test_sand_falls_down_vertically_then_left() {
        let mut map = parse_scan(SAMPLE_INPUT);
        map.drop_sand();
        map.drop_sand();

        const EXPECTED: &str = "  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.
6 ..###...#.
7 ........#.
8 .....oo.#.
9 #########.
";
        assert_eq!(EXPECTED, format!("{map}"));
    }

    #[test]
    fn test_sand_falls_down_vertically_then_left_then_right() {
        let mut map = parse_scan(SAMPLE_INPUT);
        map.drop_sand();
        map.drop_sand();
        map.drop_sand();

        const EXPECTED: &str = "  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.
6 ..###...#.
7 ........#.
8 .....ooo#.
9 #########.
";
        assert_eq!(EXPECTED, format!("{map}"));
    }

    #[test]
    fn test_sand_24() {
        let mut map = parse_scan(SAMPLE_INPUT);
        for _ in 0..24 {
            map.drop_sand();
        }

        const EXPECTED: &str = "  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ......o...
3 .....ooo..
4 ....#ooo##
5 ...o#ooo#.
6 ..###ooo#.
7 ....oooo#.
8 .o.ooooo#.
9 #########.
";
        assert_eq!(EXPECTED, format!("{map}"));
    }

    #[test]
    fn test_sand_drops_out_at_bottom() {
        let mut map = parse_scan(SAMPLE_INPUT);
        for _ in 0..250 {
            map.drop_sand();
        }

        const EXPECTED: &str = "  4     5  5
  9     0  0
  4     0  3
0 ......+...
1 ..........
2 ......o...
3 .....ooo..
4 ....#ooo##
5 ...o#ooo#.
6 ..###ooo#.
7 ....oooo#.
8 .o.ooooo#.
9 #########.
";
        assert_eq!(EXPECTED, format!("{map}"));
    }

    #[test]
    fn computes_units_until_full() {
        let mut map = parse_scan(SAMPLE_INPUT);
        assert_eq!(24, map.sand_until_full());
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Material {
    Air,
    Rock,
    Sand,
}

impl Element for Material {
    fn zero() -> Self {
        Material::Air
    }

    fn is_zero(&self) -> bool {
        self == &Material::Air
    }
}

struct Map {
    map: Compressed<Material>,
}

impl Map {
    pub(crate) fn dim(&self) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        let x_range = match self.map.iter().map(|p| { p.0 }).minmax() {
            itertools::MinMaxResult::NoElements => panic!("empty matrix"),
            itertools::MinMaxResult::OneElement(el) => el..=el,
            itertools::MinMaxResult::MinMax(min, max) => min..=max,
        };
        let y_range = match self.map.iter().map(|p| { p.1 }).minmax() {
            itertools::MinMaxResult::NoElements => panic!("empty matrix"),
            itertools::MinMaxResult::OneElement(el) => min(0, el)..=el,
            itertools::MinMaxResult::MinMax(min_rock, max) => min(0, min_rock)..=max,
        };
        (x_range, y_range)
    }

    fn sand_until_full(&mut self) -> u32 {
        let mut sand = 0;
        while self.drop_sand() {
            sand += 1;
        }
        sand
    }

    fn drop_sand(&mut self) -> bool {
        // determine where sand will come to rest by starting at the origin, and applying the rules]
        // until it comes to rest
        let mut sand_location = (500, 0);
        loop {
            // A unit of sand always falls down one step if possible.
            let (x, y) = sand_location;
            if !self.dim().1.contains(&(y + 1)) {
                // sand falls out the bottom
                return false
            }
            if self.map.get((x, y + 1)) == Material::Air {
                sand_location = (x, y + 1);
            } else if self.map.get((x - 1, y + 1)) == Material::Air {
                // If the tile immediately below is blocked (by rock or sand), the unit of sand
                // attempts to instead move diagonally one step down and to the left.
                sand_location = (x - 1, y + 1);
            } else if self.map.get((x + 1, y + 1)) == Material::Air {
                // If that tile is blocked, the unit of sand attempts to instead move diagonally
                // one step down and to the right.
                sand_location = (x + 1, y + 1);
            } else {
                // Sand keeps moving as long as it is able to do so, at each step trying to move
                // down, then down-left, then down-right. If all three possible destinations are
                // blocked, the unit of sand comes to rest and no longer moves
                break;
            }
        }
        // then update the map at the location to `Sand`
        self.map.set(sand_location, Material::Sand);
        true
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dimensions = self.dim();
        let y_digits = (*dimensions.1.end() as f32).log10() as usize + 1;
        let x_digits = (*dimensions.0.end() as f32).log10() as usize;
        for x in 0..=x_digits {
            f.write_str(" ".repeat(y_digits + 1).as_str())?;
            for y in dimensions.0.clone() {
                if y == *dimensions.0.start() || y % 10 == 0 || y == *dimensions.0.end() {
                    f.write_char(y.to_string().chars().nth(x).unwrap())?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        for y in dimensions.1.clone() {
            f.write_fmt(format_args!("{y:y_digits$} "))?;
            for x in dimensions.0.clone() {
                if self.map.get((x, y)) == Material::Rock {
                    f.write_char('#')?;
                } else if self.map.get((x, y)) == Material::Sand {
                    f.write_char('o')?;
                } else if (x, y) == (500, 0) {
                    f.write_char('+')?;
                } else {
                    f.write_char('.')?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_scan(input: &str) -> Map {
    let rock_points: Vec<(usize, usize)> = input.lines()
        .flat_map(|line: &str| {
            // dbg!(line);
            line.split("->")
                .map(str::trim)
                .flat_map(|point: &str| {
                    point.split(',')
                        .tuple_windows()
                        .map(|(x, y)| { (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()) })
                    // .inspect(|point| { dbg!(point); })
                })
                .tuple_windows()
                .flat_map(|(from, to)| {
                    if from.1 == to.1 {
                        let start = min(from.0, to.0);
                        let end = std::cmp::max(from.0, to.0);
                        (start..=end).map(|x| { (x, from.1) }).collect::<Vec<(usize, usize)>>()
                    } else {
                        let start = min(from.1, to.1);
                        let end = std::cmp::max(from.1, to.1);
                        (start..=end).map(|y| { (from.0, y) }).collect::<Vec<(usize, usize)>>()
                    }
                })
        })
        // need to clone to be able to get the bounds?
        .collect();
    let max_x = rock_points.iter().map(|p| { p.0 }).max().unwrap();
    let max_y = rock_points.iter().map(|p| { p.1 }).max().unwrap();
    let mut matrix = Compressed::new((max_x + 1, max_y + 1), Variant::Column);
    rock_points.iter().for_each(|&point| {
        matrix.set(point, Material::Rock);
    });
    Map {
        map: matrix
    }
}

pub(crate) fn part1(input: &str) -> u32 {
    let mut map = parse_scan(input);
    map.sand_until_full()
}
