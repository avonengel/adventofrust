use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

use regex::Regex;

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const SAMPLE_INPUT: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    #[test]
    fn parses_int_only_input() {
        let packet = Packet::new(SAMPLE_INPUT.lines().next().unwrap());
        let vec1 = vec![Packet::Int(1), Packet::Int(1), Packet::Int(3), Packet::Int(1), Packet::Int(1)];
        assert_eq!(packet, Packet::List(vec1));
    }

    #[test]
    fn parses_nested_list() {
        let packet = Packet::new(SAMPLE_INPUT.lines().nth(3).unwrap());
        let vec1 = vec![Packet::List(vec![Packet::Int(1)]), Packet::List(vec![Packet::Int(2), Packet::Int(3), Packet::Int(4)])];
        assert_eq!(packet, Packet::List(vec1));
    }

    #[test]
    fn parses_deeply_nested_list() {
        let packet = Packet::new("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let p = Packet::List(vec![
            Packet::Int(1),
            Packet::List(vec![
                Packet::Int(2),
                Packet::List(vec![
                    Packet::Int(3),
                    Packet::List(vec![
                        Packet::Int(4),
                        Packet::List(vec![
                            Packet::Int(5),
                            Packet::Int(6),
                            Packet::Int(7),
                        ]),
                    ]),
                ]),
            ]),
            Packet::Int(8),
            Packet::Int(9),
        ]);
        assert_eq!(packet, p);
    }

    #[test]
    fn compares_int_lists() {
        assert!(Packet::new("[1,1,3,1,1]") < Packet::new("[1,1,5,1,1]"));
    }

    #[test]
    fn compares_mixed_lists() {
        assert!(Packet::new("[[1],[2,3,4]]") < Packet::new("[[1],4]"));
        assert!(Packet::new("[[1],4]") > Packet::new("[[1],[2,3,4]]"));
    }
}


#[derive(PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(l) => {
                f.debug_list().entries(l.iter()).finish()
                // write!(f, "[{}]", l.join(", "))
            }
            Packet::Int(i) => {
                write!(f, "{i}")
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self, &other) {
            (Packet::Int(self_int), Packet::Int(other_int)) => {
                println!("Compare {self_int} vs {other_int}");
                self_int.partial_cmp(other_int)
            }
            (Packet::List(self_list), Packet::List(other_list)) => {
                println!("Compare {self_list:?} vs {other_list:?}");
                self_list.partial_cmp(other_list)
            }
            (Packet::List(_), Packet::Int(other_int)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Int(*other_int)]))
            }
            (Packet::Int(self_int), Packet::List(_)) => {
                Packet::List(vec![Packet::Int(*self_int)]).partial_cmp(other)
            }
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Less
    }

    fn le(&self, other: &Self) -> bool {
        let ordering = self.partial_cmp(other).unwrap();
        ordering == Ordering::Less || ordering == Ordering::Equal
    }

    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Greater
    }

    fn ge(&self, other: &Self) -> bool {
        let ordering = self.partial_cmp(other).unwrap();
        ordering == Ordering::Greater || ordering == Ordering::Equal
    }
}


impl Packet {
    fn new(input: &str) -> Packet {
        let mut stack: Vec<Packet> = Vec::new();
        let regex = Regex::new(r"(\[|]|\d+)").unwrap();
        for find in regex.captures_iter(input) {
            match find.get(1).unwrap().as_str() {
                "[" => stack.push(Packet::List(Vec::new())),
                "]" => {
                    let sub_packet = stack.pop().unwrap();
                    if stack.is_empty() {
                        return sub_packet;
                    } else {
                        match stack.last_mut().unwrap() {
                            Packet::List(packets) => {
                                packets.push(sub_packet);
                            }
                            Packet::Int(_) => panic!("found Packet::Int as last item on stack")
                        }
                    }
                }
                s => {
                    match stack.last_mut().unwrap() {
                        Packet::List(packets) => {
                            packets.push(Packet::Int(s.parse::<u32>().unwrap()));
                        }
                        Packet::Int(_) => panic!("found Packet::Int as last item on stack")
                    }
                }
            }
        }
        panic!("unbalanced input");
    }
}