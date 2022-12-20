use general::{get_args, read_data_lines, reset_sigpipe};
use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, Write};

#[macro_use]
extern crate json;

// using json::JsonValue to consume the input of nested lists
//
// the subset of JsonValue used to test for a number, list, or create a list
//    JsonValue.is_number()
//    JsonValue.is_array()
//    macro array![] to create a new list

type List = json::JsonValue;

// consume the input data, returning a Vec of List
fn get_data(data: &[String]) -> Vec<(List, List)> {
    let mut nums = vec![];
    for line in data {
        if !line.is_empty() {
            nums.push(json::parse(line).expect("unparsable List"))
        }
    }
    assert!(nums.len() % 2 == 0);

    let mut pairs = vec![];
    for i in (0..nums.len()).step_by(2) {
        pairs.push((nums[i].clone(), nums[i + 1].clone()))
    }
    pairs
}

// return the JsonValue as an unsigned integer
fn jint(n: &json::JsonValue) -> u64 {
    match *n {
        json::JsonValue::Number(x) => {
            let f: f64 = x.into();
            f as u64
        }
        _ => panic!("{}", format!("{}: not a JsonValue::Number", n.dump())),
    }
}

fn compare(left: &List, right: &List) -> Ordering {
    if left.is_number() && right.is_number() {
        jint(left).cmp(&jint(right))
    } else if left.is_array() && right.is_array() {
        for i in 0..left.len().max(right.len()) {
            if left[i].is_null() && right[i].is_null() {
                return Ordering::Equal;
            } else if left[i].is_null() {
                return Ordering::Less;
            } else if right[i].is_null() {
                return Ordering::Greater;
            }

            let ordering = compare(&left[i], &right[i]);
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        Ordering::Equal
    } else if left.is_number() {
        compare(&array![jint(left)], right)
    } else {
        assert!(right.is_number());
        compare(left, &array![jint(right)])
    }
}

fn part1(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    Ok(get_data(puzzle_lines)
        .iter()
        .enumerate()
        .map(|(i, p)| (i, compare(&p.0, &p.1)))
        .filter(|(_, c)| *c == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum())
}

fn part2(puzzle_lines: &[String]) -> Result<usize, Box<dyn Error>> {
    let marker1 = array!([[2]]);
    let marker2 = array!([[6]]);
    let mut packets = vec![marker1.clone(), marker2.clone()];
    for p in get_data(puzzle_lines).iter() {
        packets.push(p.0.clone());
        packets.push(p.1.clone());
    }
    packets.sort_by(compare);

    Ok(packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| *p == marker1 || *p == marker2)
        .map(|(i, _)| i + 1)
        .product())
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
        assert_eq!(part1(&puzzle_lines)?, 4734);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-example");
        assert_eq!(part2(&puzzle_lines)?, 140);
        Ok(())
    }

    #[test]
    fn part2_actual() -> Result<(), Box<dyn Error>> {
        let puzzle_lines = get_data("input-actual");
        assert_eq!(part2(&puzzle_lines)?, 21836);
        Ok(())
    }
}
