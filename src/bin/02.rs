use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        'lines: for line in reader.lines() {
            let line = line?;
            let list: Vec<&str> = line.split(' ').collect();

            let mut previous = list[0].parse::<usize>()?;
            let second = list[1].parse::<usize>()?;
            let ascending;
            if previous == second {
                continue;
            } else if previous > second {
                ascending = false;
            } else {
                ascending = true;
            }

            for level in list.iter().skip(1) {
                let level = level.parse::<usize>()?;
                let abs_diff = level.abs_diff(previous);
                if abs_diff < 1 || abs_diff > 3 {
                    continue 'lines;
                }
                if ascending && level < previous {
                    continue 'lines;
                }
                if !ascending && level > previous {
                    continue 'lines;
                }

                previous = level;
            }
            answer += 1;
        }

        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;
            let list: Vec<&str> = line.split(' ').collect();

            let (success, failed_index) = compare_line(&list)?;
            if success {
                answer += 1;
                continue;
            }

            // Test again with failed at index removed
            let mut list_removed = list.clone();
            list_removed.remove(failed_index);
            let (success, _) = compare_line(&list_removed)?;
            if success {
                answer += 1;
                continue;
            }


            // Test again with previous failed at index removed
            let mut list_removed = list.clone();
            list_removed.remove(failed_index - 1);
            let (success, _) = compare_line(&list_removed)?;
            if success {
                answer += 1;
            }

        }

        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn compare(n1: usize, n2: usize, ascending: bool) -> bool {
    let abs_diff = n1.abs_diff(n2);
    if abs_diff < 1 || abs_diff > 3 {
        return false;
    }
    if ascending && n2 < n1 {
        return false;
    }
    if !ascending && n2 > n1 {
        return false;
    }
    true
}

fn compare_line(list: &Vec<&str>) -> Result<(bool, usize)> {

    let mut previous = list[0].parse::<usize>()?;
    let second = list[1].parse::<usize>()?;
    let ascending;
    if previous == second {
        return Ok((false, 1));
    }

    if previous > second {
        ascending = false;
    } else {
        ascending = true;
    }

    for (i, level) in list.iter().skip(1).enumerate() {
        let level = level.parse::<usize>()?;
        if !compare(previous, level, ascending) {
            return Ok((false,i+1));
        }

        previous = level;
    }
    Ok((true, 0))
}