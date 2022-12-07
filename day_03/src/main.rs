use general::read_data_lines;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

// clap arg parser
mod argparse;

fn value(set: &HashSet<char>) -> u64 {
    set.iter()
        .map(|c| match c.is_lowercase() {
            true => *c as u64 - 'a' as u64 + 1,
            false => *c as u64 - 'A' as u64 + 27,
        })
        .sum::<u64>()
}

fn part1(array: &[String]) -> Result<u64, Box<dyn Error>> {
    Ok(array
        .iter()
        .map(|line| {
            let set1: HashSet<char> = line.chars().take(line.len() / 2).collect();
            let set2: HashSet<char> = line.chars().skip(line.len() / 2).collect();
            value(&set1.intersection(&set2).copied().collect())
        })
        .sum::<u64>())
}

fn part2(array: &[String]) -> Result<u64, Box<dyn Error>> {
    let mut total = 0;
    let mut set = HashSet::new();
    for (i, line) in array.iter().enumerate() {
        let line_set: HashSet<char> = line.chars().collect();
        set = match set.is_empty() {
            true => line_set.clone(),
            false => set.intersection(&line_set).copied().collect(),
        };
        if (i + 1) % 3 == 0 {
            total += value(&set);
            set.clear();
        }
    }
    Ok(total)
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    general::reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = argparse::get_args();

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
        let data = get_data("input-example");
        assert_eq!(part1(&data)?, 157);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual");
        assert_eq!(part1(&data)?, 7742);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-example");
        assert_eq!(part2(&data)?, 70);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual");
        assert_eq!(part2(&data)?, 2276);
        Ok(())
    }
}
