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
        println!("{map}");
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
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dimensions = self.dim();
        let y_digits = (*dimensions.1.end() as f32).log10() as usize;
        for y in dimensions.1.clone() {
            f.write_fmt(format_args!("{y:y_digits$} "))?;
            for x in dimensions.0.clone() {
                if self.map.get((x, y)) == Material::Rock {
                    f.write_char('#')?;
                } else if self.map.get((x, y)) == Material::Sand {
                    f.write_char('0')?;
                } else if (x, y) == (500, 0) {
                    f.write_char('+')?;
                } else {
                    f.write_char('.')?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn parse_scan(input: &str) -> Map {
    let rock_points: Vec<(usize, usize)> = input.lines()
        .flat_map(|line: &str| {
            // dbg!(line);
            line.split("->")
                .map(str::trim)
                .flat_map(|point: &str| {
                    point.split(",")
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
