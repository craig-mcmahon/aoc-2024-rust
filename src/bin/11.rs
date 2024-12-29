use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        run(reader, 25)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        run(reader, 75)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn run<R: BufRead>(reader: R, iterations: u8) -> Result<usize> {
    let line = reader.lines().flatten().collect::<String>();
    let line = line.split(' ').collect_vec().to_owned();
    let stones = line.iter().map(|s| s.parse::<usize>().unwrap()).collect_vec();
    let mut answer = 0;
    let mut handles = Vec::with_capacity(stones.len());

    for stone in stones.iter() {
        let thread_stone = stone.to_owned();
        handles.push(thread::spawn(move || -> usize {
            let mut result_map =  HashMap::<(usize, u8), usize>::new();
            blink(thread_stone, iterations, &mut result_map)
        }));
    }
    for handle in handles{
        answer += handle.join().unwrap()
    }

    Ok(answer)
}

fn blink(stone: usize, iteration: u8, results: &mut HashMap<(usize, u8), usize>) -> usize {
    if results.contains_key(&(stone, iteration)) {
        return results.get(&(stone, iteration)).unwrap().to_owned();
    }
    if iteration == 0 {
        return 1
    }
    let mut stones_count = 0;
    for stones in blink_number(stone) {
        stones_count += blink(stones, iteration - 1, results);
    }

    results.insert((stone, iteration), stones_count);
    stones_count
}
fn blink_number(stone: usize) -> Vec<usize>{
    if stone == 0 {
        return Vec::from([1]);
    }
    let str_stone = stone.to_string();
    if str_stone.len() % 2 == 0 {
        let part_1 = str_stone[0..str_stone.len()/2].to_string();
        let part_2 = str_stone[str_stone.len()/2..].to_string();
        return Vec::from([part_1.parse::<usize>().unwrap(), part_2.parse::<usize>().unwrap()]);
    }
    Vec::from([stone * 2024])
}