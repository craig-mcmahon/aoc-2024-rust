use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut l1, mut l2) = build_lists(reader)?;
        l1.sort();
        l2.sort();

        let mut answer = 0;
        for (i, list1_item) in l1.iter().enumerate() {
            answer += list1_item.abs_diff(l2[i]);
        }

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (mut l1, l2) = build_lists(reader)?;

        let mut answer = 0;
        for l1_item in l1 {
            let mut count = 0;
            for l2_item in &l2 {
                if l1_item == l2_item.clone() {
                    count += 1;
                }
            }
            answer += (count * l1_item);
        }

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}


fn build_lists<R: BufRead>(reader: R) -> Result<(Vec<usize>, Vec<usize>)> {
    let mut l1: Vec<usize> = Vec::new();
    let mut l2: Vec<usize> = Vec::new();
    for line in reader.lines() {
        let split_line = line?;
        let v: Vec<&str> = split_line.split(' ').collect();

        l1.push(v[0].parse::<usize>()?);
        l2.push(v[3].parse::<usize>()?);
    }
    Ok((l1, l2))
}