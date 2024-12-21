use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;
use std::result::Result::Ok;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = build_2d_vec(reader)?;
        let (mut x, mut y) = find_starting_point(&grid)?;
        let mut direction = Direction::UP;
        let mut visited = Vec::new();

        loop {
            visited.push((x, y));
            match move_guard(x, y, direction, &grid) {
                Ok((new_x, new_y, new_direction)) => {
                    x = new_x;
                    y = new_y;
                    direction = new_direction;
                }
                Err(_) => {
                    break;
                }
            };
        }

        Ok(visited.iter().unique().count())
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = build_2d_vec(reader)?;
        let (starting_x, starting_y) = find_starting_point(&grid)?;
        // Bit hacky but only takes a few of seconds to run
        let max_loop_counter = 10000;
        let mut answer = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != '.' {
                    continue;
                }

                let mut new_grid = grid.clone();
                new_grid[i][j] = '#';

                let mut direction = Direction::UP;
                let mut loop_counter = 0;
                let mut x = starting_x;
                let mut y = starting_y;
                for _ in 0..max_loop_counter {
                    loop_counter+=1;
                    match move_guard(x, y, direction, &new_grid) {
                        Ok((new_x, new_y, new_direction)) => {
                            x = new_x;
                            y = new_y;
                            direction = new_direction;
                        }
                        Err(_) => {
                            break;
                        }
                    };
                }
                if loop_counter == max_loop_counter {
                    answer += 1;
                }

            }
        }


        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn find_starting_point(grid: &Vec<Vec<char>>) -> Result<(isize, isize)> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                return Ok((i as isize, j as isize));
            }
        }
    }
    panic!("No starting point found!!!");
}


fn move_guard(x: isize, y: isize, direction: Direction, grid: &Vec<Vec<char>>) -> Result<(isize, isize, Direction), Error> {

    let (new_x, new_y) = match direction {
        Direction::UP => (x - 1, y),
        Direction::DOWN => (x + 1, y),
        Direction::LEFT => (x, y - 1),
        Direction::RIGHT => (x, y + 1),
    };
    if new_x < 0 || new_y < 0 || new_x >= grid.len() as isize || new_y >= grid[0].len() as isize {
        return Err(Error::msg(format_err!("OOB - {},{} - {:?}", new_x, new_y, direction)));
    }
    if grid[new_x as usize][new_y as usize] == '#' {
        let new_direction = match direction {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN =>  Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
        };
        return Ok((x, y, new_direction));
    }
    Ok((new_x, new_y, direction))
}
