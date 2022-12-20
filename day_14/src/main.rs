use general::{get_args, read_data_lines, reset_sigpipe, trim_split_on};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn get_data(data: &[String]) -> HashSet<(usize, usize)> {
    let mut rocks = HashSet::new();
    for line in data {
        let coords = line
            .split_whitespace()
            .filter(|s| *s != "->")
            .map(|s| trim_split_on::<usize>(s, ',').unwrap())
            .collect::<Vec<_>>();
        for p in coords.windows(2) {
            let (x1, y1) = (p[0][0], p[0][1]);
            let (x2, y2) = (p[1][0], p[1][1]);

            if x1 == x2 {
                for i in y1.min(y2)..=y1.max(y2) {
                    rocks.insert((x1, i));
                }
            } else {
                assert!(y1 == y2);
                for i in x1.min(x2)..=x1.max(x2) {
                    rocks.insert((i, y1));
                }
            }
        }
    }
    rocks
}

fn solve(puzzle_lines: &[String], part: usize) -> Result<usize, Box<dyn Error>> {
    let rocks = get_data(puzzle_lines);
    let mut max_depth = *rocks.iter().map(|(_, y)| y).max().expect("solution");
    if part == 2 {
        max_depth += 2;
    }

    let start = (500, 0);
    let mut p = start;
    let mut blockers = rocks.clone();
    while p.1 < max_depth {
        let y = p.1 + 1;

        let floor_test = match part {
            1 => true,
            _ => y != max_depth,
        };

        if floor_test && !blockers.contains(&(p.0, y)) {
            p = (p.0, y);
        } else if floor_test && !blockers.contains(&(p.0 - 1, y)) {
            p = (p.0 - 1, y);
        } else if floor_test && !blockers.contains(&(p.0 + 1, y)) {
            p = (p.0 + 1, y);
        } else {
            blockers.insert(p);
            if p == start {
                break;
            }
            p = start;
        }
    }

    Ok(blockers.len() - rocks.len())
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solve(puzzle_lines, 1)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    solve(puzzle_lines, 2)
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
        assert_eq!(part1(&puzzle_lines)?, 24);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 610);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 93);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 27194);
        Ok(())
    }
}
