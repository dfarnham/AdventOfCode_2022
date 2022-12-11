use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn get_pos(h: (i32, i32), t: (i32, i32), p: (i32, i32)) -> (i32, i32) {
    // same row
    if t.0 == p.0 {
        match p.1 - t.1 {
            n if n > 1 => (t.0, t.1 + 1),
            n if n < -1 => (t.0, t.1 - 1),
            _ => t,
        }
    // same column
    } else if t.1 == p.1 {
        match p.0 - t.0 {
            n if n > 1 => (t.0 + 1, t.1),
            n if n < -1 => (t.0 - 1, t.1),
            _ => t,
        }
    // > 1 row or column, move to old head
    } else if p.0 > t.0 && p.0 - t.0 > 1
        || t.0 > p.0 && t.0 - p.0 > 1
        || p.1 > t.1 && p.1 - t.1 > 1
        || t.1 > p.1 && t.1 - p.1 > 1
    {
        h
    } else {
        t
    }
}

fn coverage(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let mut mat = HashSet::new();
    let mut h = (0, 0);
    let mut t = (0, 0);
    mat.insert(t);
    for line in puzzle_lines {
        let mut cmd = line.split_whitespace();
        if let (Some(direction), Some(distance)) = (cmd.next(), cmd.next()) {
            let distance = distance.parse::<usize>()?;
            let mut row = 0;
            let mut col = 0;
            match direction {
                "R" => col = 1,
                "L" => col = -1,
                "U" => row = 1,
                _ => row = -1,
            }
            for _ in 0..distance {
                let p = (h.0 + row, h.1 + col);
                t = get_pos(h, t, p);
                mat.insert(t);
                h = p;
            }
        }
    }
    Ok(mat.len())
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    coverage(puzzle_lines)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    //coverage(puzzle_lines)
    Ok(0)
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
        assert_eq!(part1(&puzzle_lines)?, 13);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 6357);
        Ok(())
    }

    /*
    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 0);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 0);
        Ok(())
    }
    */
}
