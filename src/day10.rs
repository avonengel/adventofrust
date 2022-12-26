use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use crate::day10::Instruction::*;

#[cfg(test)]
mod tests {
    extern crate test;

    // use test::Bencher;

    use indoc::indoc;
    use itertools::Itertools;
    use super::*;

    const SAMPLE_INPUT: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    // #[test]
    // fn test_first_signal_strength() {
    //     assert_eq!(420, signal_strength(
    //         &SAMPLE_INPUT.lines().take(20).join("\n")
    //     ))
    // }
    #[test]
    fn parses_first_line() {
        let cathode_ray = CathodeRay::new(&SAMPLE_INPUT.lines().take(1).join("\n"));
        assert_eq!(cathode_ray.instructions, vec![Addx(15)]);
    }

    #[test]
    fn parses_noop() {
        let cathode_ray = CathodeRay::new("noop");
        assert_eq!(cathode_ray.instructions, vec![Noop]);
    }

    #[test]
    fn parses_instructions() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.instructions.len(), 146);
    }

    #[test]
    fn computes_x_during_cycle_1() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.register_x_during_cycle(1), 1)
    }

    #[test]
    fn computes_x_during_cycle_2() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.register_x_during_cycle(2), 1)
    }

    #[test]
    fn computes_x_during_cycle_3() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.register_x_during_cycle(3), 16)
    }

    #[test]
    fn computes_x_during_cycle_4() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.register_x_during_cycle(4), 16)
    }

    #[test]
    fn computes_signal_strength() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.register_x_during_cycle(20) * 20, 420);
    }

    #[test]
    fn computes_signal_strength_60() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.register_x_during_cycle(60) * 60, 1140);
    }

    #[test]
    fn computes_signal_strength_100() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.signal_strength_at_cycle(100), 1800);
    }

    #[test]
    fn computes_signal_strength_140() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.signal_strength_at_cycle(140), 2940);
    }

    #[test]
    fn computes_signal_strength_180() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.signal_strength_at_cycle(180), 2880);
    }

    #[test]
    fn computes_signal_strength_220() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.signal_strength_at_cycle(220), 3960);
    }

    #[test]
    fn computes_sum_of_signla_strengths() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.sum_interesting_signal_strengths(), 13140);
    }

    const SAMPLE_SCREEN: &str = indoc! {"
        ##..##..##..##..##..##..##..##..##..##..
        ###...###...###...###...###...###...###.
        ####....####....####....####....####....
        #####.....#####.....#####.....#####.....
        ######......######......######......####
        #######.......#######.......#######.....
    "};

    #[test]
    fn renders_screen() {
        let cathode_ray = CathodeRay::new(SAMPLE_INPUT);
        assert_eq!(cathode_ray.render_screen(), SAMPLE_SCREEN);
    }
}

struct CathodeRay {
    instructions: Vec<Instruction>,
}

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value.starts_with("noop") {
            Noop
        } else {
            Addx(value[5..].parse::<i32>().expect("could not parse addx value"))
        }
    }
}

impl CathodeRay {
    pub(crate) fn new(input: &str) -> CathodeRay
    {
        CathodeRay {
            instructions: input.lines().map(Instruction::from).collect(),
        }
    }
    pub(crate) fn register_x_during_cycle(&self, cycle: u32) -> i32 {
        if cycle <= 1 {
            return 1;
        }
        self.instructions.iter().fold_while((0, 1), |state, instr| {
            // println!("cycles: {}, x: {}; {instr:?}", state.0, state.1);
            if state.0 + 1 >= cycle - 1 {
                Done((state.0 + 1, state.1))
            } else {
                match instr {
                    Addx(val) => Continue((state.0 + 2, state.1 + val)),
                    Noop => Continue((state.0 + 1, state.1)),
                }
            }
        }).into_inner().1
    }
    pub fn signal_strength_at_cycle(&self, cycle: u32) -> i32 {
        cycle as i32 * self.register_x_during_cycle(cycle)
    }
    pub(crate) fn sum_interesting_signal_strengths(&self) -> i32 {
        [20, 60, 100, 140, 180, 220].map(|cycle| { self.signal_strength_at_cycle(cycle) }).iter().sum()
    }
    pub(crate) fn render_screen(&self) -> String {
        let mut result = String::with_capacity(6 * 40);
        for n in 1..=6 * 40 {
            let x = self.register_x_during_cycle(n);
            // println!("Sprite position : {}", sprite_position(&x));
            let pos = ((n - 1) % 40) as i32;
            // println!("During cycle {n:3}: CRT draws pixel in position {pos}");
            if (pos - 1..=pos + 1).contains(&x) {
                result.push('#');
            } else {
                result.push('.');
            }
            if n > 0 && n % 40 == 0 {
                result.push('\n');
            }
        }
        result
    }
}

// fn sprite_position(x: &i32) -> String {
//     (0..40).map(|p| {
//         if (p - 1..=p + 1).contains(x) {
//             '#'
//         } else {
//             '.'
//         }
//     }).collect()
// }

pub(crate) fn signal_strength(input: &str) -> i32 {
    CathodeRay::new(input).sum_interesting_signal_strengths()
}

pub fn print(input: &str) -> String {
    CathodeRay::new(input).render_screen()
}