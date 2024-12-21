use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let data = parse_data(reader)?;
        let mut valid_results = Vec::new();
        for line in data {
            let (result, nums) = line;
            let mut prev_options = Vec::new();
            prev_options.push(nums[0]);
            for i in 1..nums.len() {
                let mut new_prev_options = Vec::new();
                for prev_option in &prev_options {
                    new_prev_options.push(nums[i] + prev_option);
                    new_prev_options.push(nums[i] * prev_option);
                }
                prev_options = new_prev_options;
            }
            if prev_options.contains(&result){
                valid_results.push(result);
            }
        }

        let mut answer = 0;
        for result in valid_results {
            answer += result;
        }
        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {

        let data = parse_data(reader)?;
        let mut valid_results = Vec::new();
        for line in data {
            let (result, nums) = line;
            let mut prev_options = Vec::new();
            prev_options.push(nums[0]);
            for i in 1..nums.len() {
                let mut new_prev_options = Vec::new();
                for prev_option in &prev_options {
                    new_prev_options.push(nums[i] + prev_option);
                    new_prev_options.push(nums[i] * prev_option);
                    let mut prev_as_str = prev_option.to_string();
                    prev_as_str.push_str(nums[i].to_string().as_str());
                    new_prev_options.push(prev_as_str.parse::<usize>()?);
                }
                prev_options = new_prev_options;
            }
            if prev_options.contains(&result){
                valid_results.push(result);
            }
        }

        let mut answer = 0;
        for result in valid_results {
            answer += result;
        }
        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_data<R: BufRead>(reader: R) -> Result<Vec<(usize, Vec<usize>)>>{
    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let (result, nums) = line.split(": ").collect_tuple().unwrap();
        let nums: Vec<usize> = nums.split(' ').map(|s| s.parse::<usize>().unwrap()).collect();
        data.push((result.parse::<usize>()?, nums));
    }
    Ok(data)
}