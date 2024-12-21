use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const SEARCH: [char;4] = ['X', 'M', 'A', 'S'];

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let data = build_word_search(reader)?;
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                if data[i][j] != 'X' {
                    continue;
                }
                answer += search(&data, i, j);
            }
        }

        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let data = build_word_search(reader)?;
        // reducing the loop size to avoid having to check boundaries
        for i in 1..(data.len()-1) {
            for j in 1..(data[i].len()-1) {
                if data[i][j] != 'A' {
                    continue;
                }
                let success = match (data[i-1][j-1], data[i+1][j+1], data[i-1][j+1], data[i+1][j-1]) {
                    ('M', 'S', 'M', 'S') => true,
                    ('S', 'M', 'S', 'M') => true,
                    ('M', 'S', 'S', 'M') => true,
                    ('S', 'M', 'M', 'S') => true,
                    (_) => false,
                };
                if success {
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn build_word_search<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>> {
    let mut data = Vec::new();
    for line in reader.lines() {
        let line2 = line?;
        let mut l3 = Vec::new();

        for l2 in line2.chars() {
            l3.push(l2);
        }
        data.push(l3);
    }
    Ok(data)
}

fn search(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut found = 0;
    if search_forwards(&grid, i, j) {
        found += 1;
    }
    if search_backwards(&grid, i, j) {
        found += 1;
    }
    if search_up(&grid, i, j) {
        found += 1;
    }
    if search_down(&grid, i, j) {
        found += 1;
    }

    if search_forwards_up(&grid, i, j) {
        found += 1;
    }
    if search_forwards_down(&grid, i, j) {
        found += 1;
    }

    if search_backwards_down(&grid, i, j) {
        found += 1;
    }

    if search_backwards_up(&grid, i, j) {
        found += 1;
    }

    found
}

fn search_forwards(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if grid.len() < (i+4) {
        return false;
    }

    for (offset, char) in SEARCH.iter().enumerate() {
        if grid[i + offset][j] != *char {
            return false;
        }
    }
    true
}
fn search_backwards(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 3 {
        return false;
    }

    for (offset, char) in SEARCH.iter().enumerate() {
        if grid[i - offset][j] != *char {
            return false;
        }
    }
    true
}
fn search_up(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j < 3 {
        return false;
    }

    for (offset, char) in SEARCH.iter().enumerate() {
        if grid[i][j-offset] != *char {
            return false;
        }
    }
    true
}
fn search_down(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if grid[i].len() < (j+4) {
        return false;
    }

    for (offset, char) in SEARCH.iter().enumerate() {
        if grid[i][j+offset] != *char {
            return false;
        }
    }
    true
}

fn search_forwards_up(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if grid.len() < (i+4) {
        return false;
    }
    if j < 3 {
        return false;
    }

    for (offset, char) in SEARCH.iter().enumerate() {
        if grid[i + offset][j - offset] != *char {
            return false;
        }
    }
    true
}
fn search_backwards_up(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 3 {
        return false;
    }
    if j < 3 {
        return false;
    }

    for (offset, char) in SEARCH.iter().enumerate() {
        if grid[i - offset][j - offset] != *char {
            return false;
        }
    }
    true
}
fn search_forwards_down(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if grid.len() < (i+4) {
        return false;
    }
    if grid[i].len() < (j+4) {
        return false;
    }

    for (offset, char) in SEARCH.iter().enumerate() {
        if grid[i + offset][j + offset] != *char {
            return false;
        }
    }
    true
}
fn search_backwards_down(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 3 {
        return false;
    }
    if grid[i].len() < (j+4) {
        return false;
    }

    for (offset, char) in SEARCH.iter().enumerate() {
        if grid[i - offset][j + offset] != *char {
            return false;
        }
    }
    true
}
