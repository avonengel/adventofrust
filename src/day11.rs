use std::collections::VecDeque;
use std::ops::{Add, Mul};

use indoc::indoc;
use itertools::Itertools;
use num_integer::Integer;
use regex::Regex;

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    use indoc::indoc;
    use itertools::Itertools;

    use crate::day11::{Monkey, monkey_business, parse_monkeys};

    const SAMPLE_INPUT: &str = indoc! {"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
        "};

    #[test]
    fn test_parses_monkey() {
        let monkey = Monkey::new(&SAMPLE_INPUT.lines().take(6).join("\n"));
        assert_eq!(monkey.id, 0);
        assert_eq!(monkey.items, vec![79_u32.into(), 98_u32.into()]);
        assert_eq!((monkey.operation)(1_u32.into()), 19_u32.into());
        assert_eq!((monkey.operation)(2_u32.into()), (2_u32 * 19_u32).into());
        assert_eq!(monkey.test_divisor, 23_u32.into());
        assert_eq!(monkey.true_target, 2);
        assert_eq!(monkey.false_target, 3);
    }

    #[test]
    fn test_parses_multiple_monkeys() {
        let mut monkeys = parse_monkeys(SAMPLE_INPUT);
        let last = monkeys.pop().unwrap();
        assert_eq!(last.id, 3);
        assert_eq!(last.items, vec![74_u32.into()]);
        assert_eq!((last.operation)(1_u32.into()), 4_u32.into());
        assert_eq!((last.operation)(2_u32.into()), 5_u32.into());
        assert_eq!(last.test_divisor, 17_u32.into());
        assert_eq!(last.true_target, 0);
        assert_eq!(last.false_target, 1);
    }

    #[test]
    fn test_throw() {
        let mut monkeys = parse_monkeys(SAMPLE_INPUT);
        let modulus = monkeys.iter().map(|m| { m.test_divisor }).product();
        let (item, target) = monkeys[2].throw(&modulus, true).unwrap();
        assert_eq!(target, 1);
        assert_eq!(item, 2080_u32.into());
        let (item, target) = monkeys[2].throw(&modulus, true).unwrap();
        assert_eq!(target, 3);
        assert_eq!(item, 1200_u32.into());
        let (item, target) = monkeys[2].throw(&modulus, true).unwrap();
        assert_eq!(target, 3);
        assert_eq!(item, 3136_u32.into());
        assert_eq!(monkeys[2].throw(&modulus, true), None);
    }

    #[test]
    fn test_round() {
        let mut monkeys = parse_monkeys(SAMPLE_INPUT);
        super::round(&mut monkeys, true);
        assert_eq!(monkeys[0].items, vec![20_u32.into(), 23_u32.into(), 27_u32.into(), 26_u32.into()]);
        assert_eq!(monkeys[1].items, vec![2080_u32.into(), 25_u32.into(), 167_u32.into(), 207_u32.into(), 401_u32.into(), 1046_u32.into()]);
        assert!(monkeys[2].items.is_empty());
        assert!(monkeys[3].items.is_empty());
    }

    #[test]
    fn test_round_2() {
        let mut monkeys = parse_monkeys(SAMPLE_INPUT);
        super::round(&mut monkeys, true);
        super::round(&mut monkeys, true);
        assert_eq!(monkeys[0].items, vec![695_u32.into(), 10_u32.into(), 71_u32.into(), 135_u32.into(), 350_u32.into()]);
        assert_eq!(monkeys[1].items, vec![43_u32.into(), 49_u32.into(), 58_u32.into(), 55_u32.into(), 362_u32.into()]);
        assert!(monkeys[2].items.is_empty());
        assert!(monkeys[3].items.is_empty());
    }

    #[test]
    fn test_round_20() {
        let mut monkeys = parse_monkeys(SAMPLE_INPUT);
        for _ in 0..20 {
            super::round(&mut monkeys, true);
        }
        assert_eq!(monkeys[0].items, vec![10_u32.into(), 12_u32.into(), 14_u32.into(), 26_u32.into(), 34_u32.into()]);
        assert_eq!(monkeys[1].items, vec![245_u32.into(), 93_u32.into(), 53_u32.into(), 199_u32.into(), 115_u32.into()]);
        assert!(monkeys[2].items.is_empty());
        assert!(monkeys[3].items.is_empty());
    }

    #[test]
    fn test_item_counts() {
        let mut monkeys = parse_monkeys(SAMPLE_INPUT);
        for _ in 0..20 {
            super::round(&mut monkeys, true);
        }
        assert_eq!(monkeys[0].item_count, 101);
        assert_eq!(monkeys[1].item_count, 95);
        assert_eq!(monkeys[2].item_count, 7);
        assert_eq!(monkeys[3].item_count, 105);
    }

    #[test]
    fn test_monkey_business() {
        assert_eq!(monkey_business(SAMPLE_INPUT, 20, true), 10605);
    }

    #[test]
    fn test_monkey_business2() {
        assert_eq!(monkey_business(SAMPLE_INPUT, 10_000, false), 52166 * 52013);
    }

    #[bench]
    fn bench_monkey_business(b: &mut Bencher) {
        b.iter(|| {
            monkey_business(SAMPLE_INPUT, 20, true)
        })
    }
    #[bench]
    fn bench_monkey_business2(b: &mut Bencher) {
        b.iter(|| {
            monkey_business(SAMPLE_INPUT, 10_000, false)
        })
    }
}

struct Monkey {
    id: u32,
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test_divisor: u64,
    true_target: u32,
    false_target: u32,
    item_count: u64,
}

const MONKEY_FORMAT: &str = indoc! {"
Monkey (\\d+):
  Starting items: (.*)
  Operation: new = old (\\+|\\*) (old|\\d+)
  Test: divisible by (\\d+)
    If true: throw to monkey (\\d+)
    If false: throw to monkey (\\d+)
"};

impl Monkey {
    fn new(input: &str) -> Monkey {
        let regex = Regex::new(MONKEY_FORMAT).unwrap();
        // dbg!(&input);
        let captures = regex.captures(input).unwrap();
        // dbg!(&captures);
        let starting_items = captures[2].split(", ").map(|i| { i.parse::<u64>().unwrap() }).collect();
        Monkey {
            id: captures[1].parse::<u32>().unwrap(),
            items: starting_items,
            operation: Monkey::parse_operation(&captures[3], &captures[4]),
            test_divisor: captures[5].parse::<u64>().unwrap(),
            true_target: captures[6].parse::<u32>().unwrap(),
            false_target: captures[7].parse::<u32>().unwrap(),
            item_count: 0,
        }
    }

    fn parse_operation(operation: &str, operand: &str) -> Box<dyn Fn(u64) -> u64> {
        let fun = match operation {
            "*" => u64::mul,
            "+" => u64::add,
            _ => panic!("unmatched operation: {:?}", operation),
        };
        let option = operand.parse::<u64>();
        Box::new(move |old: u64| {
            fun(old, option.clone().unwrap_or(old))
        })
    }

    fn throw(&mut self, modulus: &u64, decrease_worry_level: bool) -> Option<(u64, u32)> {
        let mut worry_level = self.items.pop_front()?;
        self.item_count += 1;
        worry_level = (self.operation)(worry_level) % modulus;
        if decrease_worry_level {
            worry_level /= 3;
        }
        let target = if worry_level.is_multiple_of(&self.test_divisor) {
            self.true_target
        } else {
            self.false_target
        };
        Some((worry_level, target))
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.lines().filter(|&l| { !l.is_empty() }).chunks(6).into_iter().map(|chunk| {
        chunk.collect_vec().join("\n")
    }).map(|s| { Monkey::new(&s) }).collect()
}


fn round(monkeys: &mut Vec<Monkey>, decrease_worry_level: bool) {
    let modulus = monkeys.iter().map(|m| { m.test_divisor }).product();
    for i in 0..monkeys.len() {
        while let Some((item, target)) = monkeys[i].throw(&modulus, decrease_worry_level) {
            // println!("Monkey {} throws {} at monkey {}", i, item, target);
            monkeys[target as usize].items.push_back(item);
        }
    }
}

pub fn monkey_business(input: &str, rounds: u32, decrease_worry_level: bool) -> u64 {
    let mut monkeys = parse_monkeys(input);
    for _ in 1..=rounds {
        round(&mut monkeys, decrease_worry_level);
    }
    // dbg!(&monkeys.iter().map(|m| { m.item_count }).collect::<Vec<u64>>());
    monkeys.iter().map(|m| { m.item_count }).sorted().rev().take(2).product()
}
