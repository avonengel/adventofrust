use std::cmp::max;
use matrix::prelude::*;

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"30373
        25512
        65332
        33549
        35390"};


    const MIN_INPUT: &str = indoc! {"303
        255
        653"};

    #[test]
    fn test_visible_trees_min() {
        assert_eq!(super::count_visible_trees(MIN_INPUT), 9);
    }

    #[test]
    fn test_visible_trees() {
        assert_eq!(super::count_visible_trees(SAMPLE_INPUT), 21);
    }

    #[test]
    fn test_scenic_score() {
        assert_eq!(super::highest_scenic_score(SAMPLE_INPUT), 8);
    }
}

pub(crate) fn count_visible_trees(input: &str) -> u32 {
    let (size, tree_matrix) = parse_tree_matrix(input);
    // dbg!(&tree_matrix);
    let mut visible_count = (size * 2 + (size - 2) * 2) as u32;
    for r in 1..tree_matrix.rows - 1 {
        for c in 1..tree_matrix.columns - 1 {
            if visible_left(&tree_matrix, (r, c))
                || visible_right(&tree_matrix, (r, c))
                || visible_bottom(&tree_matrix, (r, c))
                || visible_top(&tree_matrix, (r, c))
            {
                visible_count += 1;
            }
        }
    }
    visible_count
}

fn parse_tree_matrix(input: &str) -> (usize, Conventional<u32>) {
    let mut iterator = input.lines().peekable();
    let size = iterator.peek().unwrap().len();
    let mut tree_matrix: Conventional<u32> = Conventional::new(size);
    iterator.enumerate().for_each(|(line_idx, line)| {
        line.chars().enumerate().for_each(|(char_idx, chr)| {
            tree_matrix[(line_idx, char_idx)] = chr.to_digit(10).unwrap();
        })
    });
    (size, tree_matrix)
}

fn visible_left(tree_matrix: &Conventional<u32>, (row, col): (usize, usize)) -> bool {
    for c in 0..col {
        if tree_matrix[(row, c)] >= tree_matrix[(row, col)] {
            return false;
        }
    }
    true
}

fn visible_top(tree_matrix: &Conventional<u32>, (row, col): (usize, usize)) -> bool {
    for r in 0..row {
        if tree_matrix[(r, col)] >= tree_matrix[(row, col)] {
            return false;
        }
    }
    true
}

fn visible_right(tree_matrix: &Conventional<u32>, (row, col): (usize, usize)) -> bool {
    for c in col + 1..tree_matrix.columns {
        if tree_matrix[(row, c)] >= tree_matrix[(row, col)] {
            return false;
        }
    }
    true
}

fn visible_bottom(tree_matrix: &Conventional<u32>, (row, col): (usize, usize)) -> bool {
    for r in row + 1..tree_matrix.rows {
        if tree_matrix[(r, col)] >= tree_matrix[(row, col)] {
            return false;
        }
    }
    true
}

pub(crate) fn highest_scenic_score(input: &str) -> u32 {
    let (size, tree_matrix) = parse_tree_matrix(input);

    let mut scenic_score = 0;
    for r in 1..size {
        for c in 1..size {
            scenic_score = max(scenic_score, compute_scenic_score(&tree_matrix, (r, c)));
        }
    }
    scenic_score
}

fn compute_scenic_score(tree_matrix: &Conventional<u32>, (row, col): (usize, usize)) -> u32 {
    let mut trees_left = 0;
    for c in (0..col).rev() {
        trees_left += 1;
        if tree_matrix[(row, c)] >= tree_matrix[(row, col)] {
            break;
        }
    }
    let mut trees_top = 0;
    for r in (0..row).rev() {
        trees_top += 1;
        if tree_matrix[(r, col)] >= tree_matrix[(row, col)] {
            break;
        }
    }
    let mut trees_right = 0;
    for c in col + 1..tree_matrix.columns {
        trees_right += 1;
        if tree_matrix[(row, c)] >= tree_matrix[(row, col)] {
            break;
        }
    }
    let mut trees_bottom = 0;
    for r in row + 1..tree_matrix.rows {
        trees_bottom += 1;
        if tree_matrix[(r, col)] >= tree_matrix[(row, col)] {
            break;
        }
    }
    // dbg!((row, col), trees_left, trees_right, trees_top, trees_bottom);
    trees_top * trees_bottom * trees_left * trees_right
}
