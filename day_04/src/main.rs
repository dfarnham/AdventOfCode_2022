use general::{read_data_lines, trim_split_on};
use std::error::Error;
use std::io::{self, Write};
use std::ops::RangeInclusive;

// clap arg parser
mod argparse;

fn ranges(array: &[String]) -> Vec<(RangeInclusive<u64>, RangeInclusive<u64>)> {
    array
        .iter()
        .map(|line| trim_split_on::<String>(line, ',').unwrap())
        .map(|start_end| {
            (
                trim_split_on::<u64>(&start_end[0], '-').unwrap(),
                trim_split_on::<u64>(&start_end[1], '-').unwrap(),
            )
        })
        .map(|p| (p.0[0]..=p.0[1], p.1[0]..=p.1[1]))
        .collect()
}

fn part1(array: &[String]) -> Result<u64, Box<dyn Error>> {
    Ok(ranges(array)
        .iter()
        .filter(|r| {
            r.1.contains(r.0.start()) && r.1.contains(r.0.end()) || r.0.contains(r.1.start()) && r.0.contains(r.1.end())
        })
        .count() as u64)
}

fn part2(array: &[String]) -> Result<u64, Box<dyn Error>> {
    Ok(ranges(array)
        .iter()
        .filter(|r| {
            r.1.contains(r.0.start()) || r.1.contains(r.0.end()) || r.0.contains(r.1.start()) || r.0.contains(r.1.end())
        })
        .count() as u64)
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
        assert_eq!(part1(&data)?, 2);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual");
        assert_eq!(part1(&data)?, 487);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-example");
        assert_eq!(part2(&data)?, 4);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let data = get_data("input-actual");
        assert_eq!(part2(&data)?, 849);
        Ok(())
    }
}
