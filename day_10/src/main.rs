use general::{get_args, read_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

fn get_cycles(puzzle_lines: &[String]) -> Result<Vec<(usize, i32)>, Box<dyn Error>> {
    let mut clock: usize = 0;
    let mut register = 1;
    let mut cycles = vec![];

    for line in puzzle_lines {
        clock += 1;
        let mut cmd = line.split_whitespace();
        if let (Some(instr), Some(value)) = (cmd.next(), cmd.next()) {
            match instr {
                "addx" => {
                    cycles.push((clock, register));
                    clock += 1;
                    register += value.parse::<i32>()?;
                }
                _ => return Err(Box::from(format!("Unknown instr: {instr}"))),
            };
        }
        cycles.push((clock, register));
    }
    Ok(cycles)
}

fn signal_strength(puzzle_lines: &[String]) -> Result<i32, Box<dyn Error>> {
    Ok(get_cycles(puzzle_lines)?
        .windows(2)
        .filter(|state| state[1].0 == 20 || ((state[1].0 as i32) - 20) % 40 == 0)
        .map(|state| (state[1].0 as i32) * state[0].1)
        .sum::<i32>())
}

fn part1(puzzle_lines: &[String]) -> Result<i32, Box<dyn Error>> {
    signal_strength(puzzle_lines)
}

fn part2(puzzle_lines: &[String]) -> Result<i32, Box<dyn Error>> {
    signal_strength(puzzle_lines)
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

    // ==============================================================

    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<String> {
        let file = std::path::PathBuf::from(filename);
        read_data_lines(Some(&file)).unwrap()
    }

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part1(&puzzle_lines)?, 13140);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 15220);
        Ok(())
    }

    /*
    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 1);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example2");
        assert_eq!(part2(&puzzle_lines)?, 36);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 2627);
        Ok(())
    }
    */
}
