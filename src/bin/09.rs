use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";
const EMPTY_SPACE: usize = 99999;
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let working = build_data(reader);
        let (fragmented_filesystem, mut filesystem_working) = build_fragmented_filesystem(&working);
        let mut defragged_filesystem = Vec::new();
        for block in fragmented_filesystem {
            if block != EMPTY_SPACE {
                if filesystem_working.len() == 0 {
                    break;
                }
                defragged_filesystem.push(filesystem_working.remove(0));
                continue;
            }
            match filesystem_working.pop() {
                Some(val) => defragged_filesystem.push(val),
                None => break,
            };
        }
        Ok(calculate_answer(defragged_filesystem))
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let working = build_data(reader);
        let (mut filesystem, _) = build_fragmented_filesystem(&working);
        let mut indexed_size = HashMap::new();
        for (length, _, index) in &working {
            indexed_size.insert(index.clone(), length.clone());
        }
        let mut empty_count: u32 = 0;
        println!("indexed - {:?}", indexed_size);
        println!("filesystem - {:?}", filesystem);
        for end_index in (1..indexed_size.len()).rev() {
            let end_index_len = indexed_size.get(&end_index).unwrap();
            let mut prev_block = EMPTY_SPACE;
            empty_count = 0;
            for block_index in 0..filesystem.len() {
                let block = filesystem[block_index];

                if block != EMPTY_SPACE {
                    if empty_count >= *end_index_len {
                        // Remove existing value from end
                        filesystem = filesystem.iter().map(|x| {
                            if *x == end_index {
                                return EMPTY_SPACE;
                            }
                            *x
                        }).collect_vec();
                        for ei in (0..*end_index_len) {
                            filesystem[block_index - (empty_count as usize) + (ei as usize)] = end_index;
                        }
                        break;

                    }

                    if block == end_index {
                        // If block = one we are moving it break, don't want to move it backwards
                        break;
                    }
                    prev_block = block;
                    empty_count = 0;
                    continue;
                }
                if prev_block != EMPTY_SPACE {
                    empty_count += 1;
                }
            }

        }

        println!("File system: {:?}", filesystem);

        Ok(calculate_answer(filesystem))
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn build_data<R: BufRead>(reader: R) -> Vec<(u32, u32, usize)> {
    let data = reader.lines().flatten().collect::<String>();
    println!("{:?}", data);
    let mut space = false;
    let mut index:usize = 0;
    let mut working = Vec::new();
    let mut prev_char = '0';
    for char in data.chars() {
        if space {
            prev_char = char;
            space = false;
            continue;
        }
        working.push((char.to_digit(10).unwrap().to_owned(), prev_char.to_digit(10).unwrap().to_owned(), index));
        space = true;
        index +=1;
    }
    working
}

fn build_fragmented_filesystem(working: &Vec<(u32, u32, usize)>) -> (Vec<usize>, Vec<usize>) {
    let mut fragmented_filesystem:Vec<usize> = Vec::new();
    let mut filesystem_working = Vec::new();
    for (size, gap, index) in working {
        for _i in 0..gap.to_owned() {
            fragmented_filesystem.push(EMPTY_SPACE);
        }
        for _i in 0..size.to_owned() {
            fragmented_filesystem.push(index.to_owned());
            filesystem_working.push(index.to_owned());
        }
    }
    (fragmented_filesystem, filesystem_working)
}

fn calculate_answer(defragged_filesystem: Vec<usize>) -> usize {
    let mut answer = 0;
    for i in 0..defragged_filesystem.len() {
        if defragged_filesystem[i] == EMPTY_SPACE {
            continue;
        }
        answer += i * defragged_filesystem[i];
    }
    answer
}