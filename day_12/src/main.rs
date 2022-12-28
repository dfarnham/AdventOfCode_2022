use general::{get_args, read_data_lines, reset_sigpipe};
use ndarray::*;
use std::collections::{BTreeSet, VecDeque};
use std::error::Error;
use std::io::{self, Write};

fn get_grid(data: &[String]) -> (Array2<usize>, (usize, usize), (usize, usize)) {
    // row parsing rules for lines in data
    let get_row = |s: &str| {
        s.chars()
            .map(|c| (c as usize) - 'A' as usize)
            .collect::<Vec<_>>()
    };

    // use data[0] to size the new Array2
    let mut grid = Array::from_elem((0, data[0].len()), 0);

    // process data[..]
    for line in data {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap()
    }

    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..grid.nrows() {
        for j in 0..grid.ncols() {
            if grid[[i, j]] == 'S' as usize - 'A' as usize {
                start = (i, j);
                grid[[i, j]] = 'a' as usize - 'A' as usize
            } else if grid[[i, j]] == 'E' as usize - 'A' as usize {
                end = (i, j);
                grid[[i, j]] = 'z' as usize - 'A' as usize
            }
        }
    }
    assert!(start != end);
    (grid, start, end)
}

fn neighbor_indices(m: &Array2<usize>, p: (usize, usize)) -> Vec<(usize, usize)> {
    let (i, j) = p;
    let maxval = m[[i, j]] + 1;
    let mut indices = vec![];

    // above
    if i > 0 && m[[i - 1, j]] <= maxval {
        indices.push((i - 1, j))
    }

    // left
    if j > 0 && m[[i, j - 1]] <= maxval {
        indices.push((i, j - 1))
    }

    // below
    if i < m.nrows() - 1 && m[[i + 1, j]] <= maxval {
        indices.push((i + 1, j))
    }

    // right
    if j < m.ncols() - 1 && m[[i, j + 1]] <= maxval {
        indices.push((i, j + 1))
    }

    indices
}

fn solve(m: &Array2<usize>, s: (usize, usize), e: (usize, usize), part: usize) -> usize {
    let mut visited = BTreeSet::<(usize, usize)>::new();

    // insert the starting position into the queue
    let mut q = VecDeque::new();
    if part == 1 {
        q.push_back((s, 0))
    } else {
        // insert the indicies of all 'm' values matching the value at the
        // starting indicies (all indicies with value 'a' in this puzzle)
        for i in 0..m.nrows() {
            for j in 0..m.ncols() {
                if m[[i, j]] == m[[s.0, s.1]] {
                    q.push_back(((i, j), 0))
                }
            }
        }
    }

    while !q.is_empty() {
        let (p, d) = q.pop_front().expect("bug");
        if !visited.contains(&p) {
            visited.insert(p);
            if p == e {
                return d;
            }

            for xy in neighbor_indices(m, p).iter().copied() {
                q.push_back((xy, d + 1))
            }
        }
    }
    panic!("oops")
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let (mat, s, e) = get_grid(puzzle_lines);
    Ok(solve(&mat, s, e, 1))
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let (mat, s, e) = get_grid(puzzle_lines);
    Ok(solve(&mat, s, e, 2))
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    writeln!(stdout, "Answer Part 1 = {}", part1(&puzzle_lines)?)?;
    writeln!(stdout, "Answer Part 2 = {}", part2(&puzzle_lines)?)?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
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
        assert_eq!(part1(&puzzle_lines)?, 31);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 350);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 29);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 349);
        Ok(())
    }
}
