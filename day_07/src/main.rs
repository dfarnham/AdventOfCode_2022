use general::{get_args, read_data_lines, reset_sigpipe};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug)]
enum Item {
    Dir(PathBuf),
    File(usize),
}

fn build_filesystem_view(commands: &[String]) -> Result<BTreeMap<String, Item>, Box<dyn Error>> {
    let mut fs = BTreeMap::new();

    let mut root = PathBuf::new();
    root.push("/");
    let mut current = root.clone();
    fs.insert(root.display().to_string(), Item::Dir(root));

    for line in commands {
        let mut path = PathBuf::new();
        path.push(current.clone());

        if line.starts_with("$ cd") {
            let arg = &line[5..];
            if arg == ".." {
                path.pop();
            } else {
                path.push(arg);
            }
            current = match fs.get(&path.to_string_lossy().to_string()) {
                Some(Item::Dir(path)) => path.to_path_buf(),
                _ => return Err(Box::from("unknown directory")),
            };
        } else if line.starts_with("$ ls") {
        } else {
            let listing = line.split_whitespace().collect::<Vec<_>>();
            if listing[0] == "dir" {
                path.push(listing[1]);
                fs.insert(path.display().to_string(), Item::Dir(path));
            } else {
                path.push(listing[1]);
                fs.insert(
                    path.display().to_string(),
                    Item::File(listing[0].parse::<usize>()?),
                );
            }
        }
    }

    Ok(fs)
}

fn get_dir_size(dir: &str, fs: &BTreeMap<String, Item>) -> usize {
    fs.iter()
        .filter(|(_, item)| matches!(item, Item::File(_)))
        .filter(|(path, _)| path.starts_with(dir))
        .map(|(_, item)| match item {
            Item::File(size) => *size,
            _ => panic!("impossible - already filtered on matches!"),
        })
        .sum::<usize>()
}

fn get_dir_sizes(fs: &BTreeMap<String, Item>) -> Vec<usize> {
    fs.iter()
        .filter(|(_, item)| matches!(item, Item::Dir(_)))
        .map(|(path, _)| get_dir_size(&(path.to_owned() + "/"), fs))
        .collect::<Vec<_>>()
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let fs = build_filesystem_view(puzzle_lines)?;
    Ok(get_dir_sizes(&fs)
        .iter()
        .filter(|s| *s <= &100000)
        .sum::<usize>())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let fs = build_filesystem_view(puzzle_lines)?;
    let free = 70000000 - get_dir_size("/", &fs);
    Ok(get_dir_sizes(&fs)
        .iter()
        .filter(|s| free + *s >= 30000000)
        .copied()
        .min()
        .expect("no solution"))
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
        assert_eq!(part1(&puzzle_lines)?, 95437);
        Ok(())
    }

    #[test]
    fn part1_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part1(&puzzle_lines)?, 1778099);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 24933642);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 1623571);
        Ok(())
    }
}
