#[derive(Debug)]
pub struct BingoGame {
    numbers_drawn: Vec<u32>,
    boards: Vec<BingoBoard>,
}

#[derive(Debug)]
struct BingoBoard {
    numbers: [[u32; 5]; 5],
}

impl BingoBoard {
    fn rounds_until_win(&self, numbers_drawn: &Vec<u32>) -> usize {
        for count in 5..numbers_drawn.len() {
            let current_draw = &numbers_drawn[0..count];
            for row in self.numbers.iter() {
                if row.iter().filter(|n| current_draw.contains(n)).count() == 5 {
                    return count;
                }
            }
            for col in 0..5 {
                let mut matched = 0;
                for row in self.numbers.iter() {
                    if current_draw.contains(&row[col]) {
                        matched += 1;
                    }
                }
                if matched == 5 {
                    return count;
                }
            }
        }
        usize::MAX
    }

    fn score(&self, win_draw: &[u32]) -> u32 {
        let sum_unmarked: u32 = self.numbers.iter().flatten().filter(|n| !win_draw.contains(n)).sum();
        sum_unmarked * win_draw.last().unwrap()
    }
}

impl BingoGame {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines().filter(|l| !l.is_empty());
        let numbers_drawn: Vec<u32> = lines.next().unwrap().split(",").map(|nbr| { nbr.parse().unwrap() }).collect();

        let mut boards = Vec::new();
        loop {
            let mut board_numbers = [[0; 5]; 5];
            let mut lines_taken = 0;
            for (line_idx, line) in lines.by_ref().take(5).enumerate() {
                lines_taken += 1;
                let mut numbers = line.split_whitespace();
                for number_idx in 0..5 {
                    board_numbers[line_idx][number_idx] = numbers.next().unwrap().parse().unwrap();
                }
            }
            if lines_taken < 5 {
                break
            }
            boards.push(BingoBoard { numbers: board_numbers })
        }
        BingoGame { numbers_drawn, boards }
    }

    pub fn first_winner_score(&self) -> u32 {
        let mut winning_score = 0;
        let mut min_winning_rounds = usize::MAX;

        for board in self.boards.iter() {
            let win_rounds = board.rounds_until_win(&self.numbers_drawn);
            if win_rounds < min_winning_rounds {
                min_winning_rounds = win_rounds;
                winning_score = board.score(&self.numbers_drawn[0..win_rounds]);
            }
        }
        winning_score
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const SAMPLE_INPUT: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "};


    #[test]
    fn test_parser() {
        let game = BingoGame::new(SAMPLE_INPUT);
        assert_eq!(7, *game.numbers_drawn.first().unwrap());
        assert_eq!(1, *game.numbers_drawn.last().unwrap());
        assert_eq!(3, game.boards.len());
        assert_eq!(22, game.boards.first().unwrap().numbers[0][0]);
        assert_eq!(19, game.boards.first().unwrap().numbers[4][4]);

        assert_eq!(3, game.boards.get(1).unwrap().numbers[0][0]);
        assert_eq!(6, game.boards.get(1).unwrap().numbers[4][4]);

        assert_eq!(14, game.boards.last().unwrap().numbers[0][0]);
        assert_eq!(7, game.boards.last().unwrap().numbers[4][4]);
    }

    #[test]
    fn test_first_winner_score() {
        let game = BingoGame::new(SAMPLE_INPUT);
        assert_eq!(4512, game.first_winner_score());
    }
}