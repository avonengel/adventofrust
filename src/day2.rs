use crate::day2::Symbol::{Rock, Paper, Scissors};
use crate::day2::Outcome::*;

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::day2::Symbol::*;

    const SAMPLE_INPUT: &str = indoc! {"A Y
        B X
        C Z
    "};

    #[test]
    fn parses_rock_rock() {
        assert_eq!(super::parse("A X"), (Rock, Rock))
    }

    #[test]
    fn parses_paper_paper() {
        assert_eq!(super::parse("B Y"), (Paper, Paper))
    }

    #[test]
    fn parses_scissors_scissors() {
        assert_eq!(super::parse("C Z"), (Scissors, Scissors))
    }
    #[test]
    fn computes_total_score() {
        assert_eq!(super::total_score(SAMPLE_INPUT), 15)
    }
    #[test]
    fn computes_total_score2() {
        assert_eq!(super::total_score2(SAMPLE_INPUT), 12)
    }
}

pub(crate) fn total_score(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (opponent, mine) = parse(line);
        sum += shape_score(&mine);
        sum += match_score(&opponent, &mine);
    }
    sum
}
pub(crate) fn total_score2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (opponent, outcome) = parse2(line);
        sum += shape_score2(&opponent,&outcome);
        sum += match_score2(&outcome);
    }
    sum
}

fn match_score2(outcome: &Outcome) -> u32 {
    match outcome {
        Lose => 0,
        Draw => 3,
        Win => 6,
    }
}

fn match_score(opponent: &Symbol, mine: &Symbol) -> u32 {
    if opponent == mine {
        return 3
    }
    match (opponent, mine) {
        (Rock, Paper) => 6,
        (Paper, Rock) => 0,
        (Rock, Scissors) => 0,
        (Scissors, Rock) => 6,
        (Paper, Scissors) => 6,
        (Scissors, Paper) => 0,
        _ => panic!()
    }
}

fn shape_score(symbol: &Symbol) -> u32 {
    match symbol {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn shape_score2(opponent:&Symbol, outcome: &Outcome) -> u32 {
    if outcome == &Draw {
        return shape_score(opponent);
    }
    match (opponent, outcome) {
        (Rock, Lose) => shape_score(&Scissors),
        (Rock, Win) => shape_score(&Paper),
        (Paper, Lose) => shape_score(&Rock),
        (Paper, Win) => shape_score(&Scissors),
        (Scissors, Lose) => shape_score(&Paper),
        (Scissors, Win) => shape_score(&Rock),
        _ => panic!()
    }
}

fn parse(line: &str) -> (Symbol, Symbol) {
    let mut chars = line.chars();
    let opponent = match chars.next() {
        Some('A') => Rock,
        Some('B') => Paper,
        Some('C') => Scissors,
        Some(ref value) => panic!("Could not match opponent's symbol: {value:?}"),
        _ => panic!("could not match opponent's symbol")
    };
    let mine = match chars.nth(1) {
        Some('X') => Rock,
        Some('Y') => Paper,
        Some('Z') => Scissors,
        Some(ref value) => panic!("Could not match my symbol: {value:?}"),
        _ => panic!("could not match opponent's symbol")
    };
    (opponent, mine)
}

fn parse2(line: &str) -> (Symbol, Outcome) {
    let mut chars = line.chars();
    let opponent = match chars.next() {
        Some('A') => Rock,
        Some('B') => Paper,
        Some('C') => Scissors,
        Some(ref value) => panic!("Could not match opponent's symbol: {value:?}"),
        _ => panic!("could not match opponent's symbol")
    };
    let outcome = match chars.nth(1) {
        Some('X') => Lose,
        Some('Y') => Draw,
        Some('Z') => Win,
        Some(ref value) => panic!("Could not match my symbol: {value:?}"),
        _ => panic!("could not match opponent's symbol")
    };
    (opponent, outcome)
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}
