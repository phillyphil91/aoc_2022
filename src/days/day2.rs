const WIN: i8 = 6;
const DRAW: i8 = 3;
const LOSE: i8 = 0;

fn day2(elf: &str, helper: &str) -> i32 {
    #[derive(PartialEq, Eq)]
    enum RPSOptions {
        Rock(i8),
        Paper(i8),
        Scissor(i8),
        NotAnOption,
    }

    let elf_op_choice = match elf {
        "A" => RPSOptions::Rock(1),
        "B" => RPSOptions::Paper(2),
        "C" => RPSOptions::Scissor(3),
        _ => RPSOptions::NotAnOption,
    };

    let helper_op_choice = match helper {
        "X" => RPSOptions::Rock(1),
        "Y" => RPSOptions::Paper(2),
        "Z" => RPSOptions::Scissor(3),
        _ => RPSOptions::NotAnOption,
    };

    let points = if elf_op_choice != helper_op_choice {
        match elf_op_choice {
            RPSOptions::Rock(_) => match helper_op_choice {
                RPSOptions::Paper(val) => val + WIN,
                RPSOptions::Scissor(val) => val + LOSE,
                _ => 0,
            },
            RPSOptions::Paper(_) => match helper_op_choice {
                RPSOptions::Rock(val) => val + LOSE,
                RPSOptions::Scissor(val) => val + WIN,
                _ => 0,
            },
            RPSOptions::Scissor(_) => match helper_op_choice {
                RPSOptions::Rock(val) => val + WIN,
                RPSOptions::Paper(val) => val + LOSE,
                _ => 0,
            },
            RPSOptions::NotAnOption => 0,
        }
    } else {
        match helper_op_choice {
            RPSOptions::Paper(val) => val + DRAW,
            RPSOptions::Rock(val) => val + DRAW,
            RPSOptions::Scissor(val) => val + DRAW,
            RPSOptions::NotAnOption => 0,
        }
    };

    points as i32
}

fn day2_part2(elf: &str, helper: &str) -> i32 {

    #[derive(PartialEq, Eq)]
    enum RPSOutcome {
        Win,
        Draw,
        Lose,
    }

    #[derive(PartialEq, Eq)]
    enum RPSOptions {
        Rock(i8),
        Paper(i8),
        Scissor(i8),
        NotAnOption,
    }

    let elf_op_choice = match elf {
        "A" => RPSOptions::Rock(1),
        "B" => RPSOptions::Paper(2),
        "C" => RPSOptions::Scissor(3),
        _ => RPSOptions::NotAnOption,
    };

    let helper_op_choice = match helper {
        "X" => RPSOutcome::Lose,
        "Y" => RPSOutcome::Draw,
        "Z" => RPSOutcome::Win,
        _ => RPSOutcome::Win,
    };

    let points = match elf_op_choice {
            RPSOptions::Rock(_) => match helper_op_choice {
                RPSOutcome::Win => 2 + WIN,
                RPSOutcome::Draw => 1 + DRAW,
                RPSOutcome::Lose => 3 + LOSE,
            },
            RPSOptions::Paper(_) => match helper_op_choice {
                RPSOutcome::Win => 3 + WIN,
                RPSOutcome::Draw => 2 + DRAW,
                RPSOutcome::Lose => 1 + LOSE,
            },
            RPSOptions::Scissor(_) => match helper_op_choice {
                RPSOutcome::Win => 1 + WIN,
                RPSOutcome::Draw => 3 + DRAW,
                RPSOutcome::Lose => 2 + LOSE,
            },
            RPSOptions::NotAnOption => 0
    };

    points as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_part1() {
        let elf = "A";
        let helper = "Y";
        let result = day2(elf, helper);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_manual_part2() {
        let elf = "A";
        let helper = "Y";
        let result = day2_part2(elf, helper);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sample() {
        let input = r##"A Y
B X
C Z"##;

        let mut result: i32 = 0;

        for line in input.lines() {
            let mut choice = line.split_whitespace();
            let elf = choice.next().unwrap();
            let helper = choice.next().unwrap();
            result += day2_part2(elf, helper);
        }

        assert_eq!(result, 12);
    }

    #[test]
    fn test_full() {
        let input = include_str!("../../input/day2_a.txt");

        let mut result = 0;

        for line in input.lines() {
            let mut choice = line.split_whitespace();
            let elf = choice.next().unwrap();
            let helper = choice.next().unwrap();
            result += day2_part2(elf, helper);
        }

        assert_eq!(result, 9759);
    }
}
