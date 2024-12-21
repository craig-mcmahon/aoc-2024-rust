use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
        let data = reader.lines().flatten().collect::<String>();
        for (_, [a, b]) in re.captures_iter(&data).map(|c| c.extract()) {
            answer += (a.parse::<usize>()? * b.parse::<usize>()?);
        }

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {:?}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
        let data = reader.lines().flatten().collect::<String>();
        let lines = data.split("do()");
        for line in lines {
            // Each line is a do, so split, anything after the first part will be a don't
            let inner_split = line.split("don\'t").collect::<Vec<_>>();
            for (_, [a, b]) in re.captures_iter(&inner_split[0]).map(|c| c.extract()) {
                answer += (a.parse::<usize>()? * b.parse::<usize>()?);
            }
        }

        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
