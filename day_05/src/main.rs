use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Write};

fn build_stacks(array: &[String]) -> BTreeMap<usize, Vec<char>> {
    let mut stacks = BTreeMap::new();
    for line in array.iter() {
        if line.starts_with(" 1") {
            break;
        }
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                //let v = stacks.entry(i).or_insert(vec![]);
                let v: &mut Vec<char> = stacks.entry(i).or_default();
                v.insert(0, c);
            }
        }
    }
    stacks
}

fn move_crates(
    array: &[String],
    stacks: &BTreeMap<usize, Vec<char>>,
) -> Result<String, Box<dyn Error>> {
    let n = stacks.values().map(|v| v.len()).max().ok_or("max error")?;
    let mut stacks = stacks.clone();
    for line in array.iter().skip(n + 2) {
        let instructions = line
            .split_whitespace()
            .collect::<Vec<_>>();
        let count = instructions[1].parse::<usize>()?;
        let source = instructions[3].parse::<usize>()? - 1;
        let destination = instructions[5].parse::<usize>()? - 1;

        for _ in 0..count {
            if let Some(a) = stacks.get_mut(&source) {
                let value = a.pop().ok_or("pop() error")?;
                if let Some(b) = stacks.get_mut(&destination) {
                    b.push(value);
                }
            }
        }
    }
    Ok(stacks
        .values()
        .map(|v| v.last().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(""))
}

/*
fn part1(array: &[String]) -> Result<u64, Box<dyn Error>> {
    Ok(0)
}

fn part2(array: &[String]) -> Result<u64, Box<dyn Error>> {
    Ok(0)
}
*/

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

    // ==============================================================

    let stacks = build_stacks(&puzzle_lines);
    writeln!(
        stdout,
        "Answer Part 1 = {}",
        move_crates(&puzzle_lines, &stacks)?
    )?;
    //writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines)?)?;
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
        let stacks = build_stacks(&puzzle_lines);
        assert_eq!(move_crates(&puzzle_lines, &stacks)?, "CMZ");
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        let stacks = build_stacks(&puzzle_lines);
        assert_eq!(move_crates(&puzzle_lines, &stacks)?, "CVCWCRTVQ");
        Ok(())
    }

    /*
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
    */
}
