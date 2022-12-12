use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

fn get_pos(t: (i32, i32), m: (i32, i32)) -> (i32, i32) {
    // same row
    if t.0 == m.0 {
        match m.1 - t.1 {
            n if n > 1 => (t.0, t.1 + 1),
            n if n < -1 => (t.0, t.1 - 1),
            _ => t,
        }
    // same column
    } else if t.1 == m.1 {
        match m.0 - t.0 {
            n if n > 1 => (t.0 + 1, t.1),
            n if n < -1 => (t.0 - 1, t.1),
            _ => t,
        }
    // diagonal
    } else if (t.0 - m.0).abs() > 1 || (t.1 - m.1).abs() > 1 {
        if (t.0 - m.0).abs() > 1 && (t.1 - m.1).abs() > 1 {
            (
                if m.0 > t.0 { t.0 + 1 } else { t.0 - 1 },
                if m.1 > t.1 { t.1 + 1 } else { t.1 - 1 },
            )
        } else if m.0 - t.0 > 1 {
            (t.0 + 1, m.1)
        } else if t.0 - m.0 > 1 {
            (t.0 - 1, m.1)
        } else if m.1 - t.1 > 1 {
            (m.0, t.1 + 1)
        } else {
            (m.0, t.1 - 1)
        }
    // distance 1, (t.0 - m.0).abs(), (t.1 - m.1).abs() == (1, 1)
    } else {
        t
    }
}

fn iterate(v: &Vec<(i32, i32)>, m: (i32, i32)) -> Vec<(i32, i32)> {
    let mut newvec = vec![m];
    let n = v.len();
    for i in 1..n {
        newvec.push(get_pos(v[i], newvec[i - 1]));
    }
    newvec
}

fn coverage(puzzle_lines: &[String], knots: usize) -> Result<usize, Box<dyn Error>> {
    let mut mat = HashSet::new();
    let mut rope = (0..knots).map(|_| (0, 0)).collect::<Vec<(_, _)>>();
    mat.insert(rope[knots - 1]);

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
                let m = (rope[0].0 + row, rope[0].1 + col);
                rope = iterate(&rope, m);
                mat.insert(rope[knots - 1]);
            }
        }
    }
    Ok(mat.len())
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    coverage(puzzle_lines, 2)
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    coverage(puzzle_lines, 10)
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
}
