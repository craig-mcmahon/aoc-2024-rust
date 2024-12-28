use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

#[derive(Debug)]
struct Height {
    value: u8,
}

impl TrailPoint {
    fn from_grid(x: usize, y: usize, grid: &Vec<Vec<Height>>) -> Self {
        TrailPoint {x, y, height: grid[x][y].value}
    }
}

#[derive(Debug)]
struct TrailPoint {
    x: usize,
    y: usize,
    height: u8,
}


impl From<char> for Height {
    fn from(value: char) -> Self {
        Height{value: value.to_digit(10).unwrap().to_owned() as u8}
    }
}
impl From<Height> for u8 {
    fn from(value: Height) -> Self {
        value.value
    }
}
const PEAK: u8 = 9;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<Height>> = build_2d_vec(reader)?;

        let mut scores = Vec::new();
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y].value == 0 {
                    let mut visited = Vec::new();
                    let score = calculate_trails(TrailPoint {x, y, height: 0}, &grid, &mut visited);
                    if score.is_some() {
                        scores.push(score.unwrap());
                    }
                }
            }
        }

        let mut answer = 0;
        for score in scores {
            answer += score;
        }
        Ok(answer)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<Height>> = build_2d_vec(reader)?;

        let mut scores = Vec::new();
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y].value == 0 {
                    let score = calculate_trails_part2(TrailPoint {x, y, height: 0}, &grid);
                    if score.is_some() {
                        scores.push(score.unwrap());
                    }
                }
            }
        }

        let mut answer = 0;
        for score in scores {
            answer += score;
        }
        Ok(answer)    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn calculate_trails(trail_point: TrailPoint, grid: &Vec<Vec<Height>>, visited: &mut Vec<(usize, usize)>) -> Option<usize> {
    if visited.contains(&(trail_point.x, trail_point.y)) {
        return None;
    }
    if trail_point.height == PEAK {
        visited.push((trail_point.x, trail_point.y));
        return Some(1);
    }
    let max_x = grid.len();
    let max_y = grid[0].len();
    let mut score = 0;
    if trail_point.x > 0 {
        let new_trail_point = TrailPoint::from_grid(trail_point.x - 1, trail_point.y, grid);
        if new_trail_point.height == (trail_point.height + 1) {
            score += calculate_trails(new_trail_point, grid, visited).unwrap_or_else(|| 0);
        }
    }
    if trail_point.y > 0 {
        let new_trail_point = TrailPoint::from_grid(trail_point.x, trail_point.y - 1, grid);
        if new_trail_point.height == (trail_point.height + 1) {
            score += calculate_trails(new_trail_point, grid, visited).unwrap_or_else(|| 0);
        }
    }
    if trail_point.x < max_x - 1 {
        let new_trail_point = TrailPoint::from_grid(trail_point.x + 1, trail_point.y, grid);
        if new_trail_point.height == (trail_point.height + 1) {
            score += calculate_trails(new_trail_point, grid, visited).unwrap_or_else(|| 0);
        }
    }
    if trail_point.y < max_y - 1 {
        let new_trail_point = TrailPoint::from_grid(trail_point.x , trail_point.y + 1, grid);
        if new_trail_point.height == (trail_point.height + 1) {
            score += calculate_trails(new_trail_point, grid, visited).unwrap_or_else(|| 0);
        }
    }
    if score > 0 {
        return Some(score);
    }
    None
}


fn calculate_trails_part2(trail_point: TrailPoint, grid: &Vec<Vec<Height>>) -> Option<usize> {
    if trail_point.height == PEAK {
        return Some(1);
    }
    let max_x = grid.len();
    let max_y = grid[0].len();
    let mut score = 0;
    if trail_point.x > 0 {
        let new_trail_point = TrailPoint::from_grid(trail_point.x - 1, trail_point.y, grid);
        if new_trail_point.height == (trail_point.height + 1) {
            score += calculate_trails_part2(new_trail_point, grid).unwrap_or_else(|| 0);
        }
    }
    if trail_point.y > 0 {
        let new_trail_point = TrailPoint::from_grid(trail_point.x, trail_point.y - 1, grid);
        if new_trail_point.height == (trail_point.height + 1) {
            score += calculate_trails_part2(new_trail_point, grid).unwrap_or_else(|| 0);
        }
    }
    if trail_point.x < max_x - 1 {
        let new_trail_point = TrailPoint::from_grid(trail_point.x + 1, trail_point.y, grid);
        if new_trail_point.height == (trail_point.height + 1) {
            score += calculate_trails_part2(new_trail_point, grid).unwrap_or_else(|| 0);
        }
    }
    if trail_point.y < max_y - 1 {
        let new_trail_point = TrailPoint::from_grid(trail_point.x , trail_point.y + 1, grid);
        if new_trail_point.height == (trail_point.height + 1) {
            score += calculate_trails_part2(new_trail_point, grid).unwrap_or_else(|| 0);
        }
    }
    if score > 0 {
        return Some(score);
    }
    None
}