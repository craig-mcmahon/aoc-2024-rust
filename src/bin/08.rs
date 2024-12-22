use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        generate_answer(reader, false)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        generate_answer(reader, true)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn get_antenna_locations(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut result = HashMap::new();
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == '.' {
                continue;
            }
            if !result.contains_key(&grid[x][y]) {
                result.insert(grid[x][y], Vec::new());
            }
            let mut vec = result.get(&grid[x][y]).unwrap().to_vec();
            vec.push((x, y));
            result.insert(grid[x][y], vec);
        }
    }
    result
}

fn print_grid(grid: &Vec<Vec<char>>, antinode_locations: &Vec<(usize, usize)>) {
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if antinode_locations.contains(&(x, y)) {
                print!("#");
                continue;
            }
            if grid[x][y] != '.' {
                print!("{}", grid[x][y]);
                continue;
            }
            print!(".");
        }
        println!();
    }
}

fn add_antinode(x1: usize, y1: usize, x2: usize, y2: usize, mut antinodes: Box<Vec<(usize, usize)>>, max_x: usize, max_y: usize) -> Box<Vec<(usize, usize)>> {
    let (xn1, xn2, _, _) = calculate_xy_val(x1, x2);
    let (yn1, yn2, _, _) = calculate_xy_val(y1, y2);
    if xn1 < max_x && yn1 < max_y {
        antinodes.push((xn1, yn1));
    }
    if xn2 < max_x && yn2 < max_y {
        antinodes.push((xn2, yn2));
    }

    antinodes
}

fn generate_answer<R: BufRead>(reader: R, part2: bool) -> Result<usize> {
    let data = build_2d_vec(reader)?;
    let antenna_locations = get_antenna_locations(&data);
    let mut checked = Vec::new();
    let mut antinodes = Box::new(Vec::new());
    let max_x = data.len();
    let max_y = data[0].len();
    for (_char, locations) in antenna_locations {
        let locations = locations.clone();
        for (x1, y1) in &locations {
            for (x2, y2) in &locations {
                if (x1, y1) == (x2, y2) {
                    continue;
                }
                if checked.contains(&(*x1, *y1, *x2, *y2)) {
                    continue;
                }
                checked.push((*x1, *y1, *x2, *y2));
                checked.push((*x2, *y2, *x1, *y1));
                if !part2 {
                    antinodes = add_antinode(*x1, *y1, *x2, *y2, antinodes, max_x, max_y);
                } else {
                    antinodes = add_antinode_part2(*x1, *y1, *x2, *y2, antinodes, max_x, max_y);
                }
            }
        }

    }
    print_grid(&data, &antinodes);
    let answer = antinodes.iter().unique().count();
    Ok(answer)
}

fn calculate_xy_val(v1: usize, v2: usize)-> (usize, usize, bool, usize) {
    let diff = v1.abs_diff(v2);
    let (n1, n2, less);
    if v1 < v2 {
        n1 = v1.wrapping_sub(diff);
        n2 = v2.wrapping_add(diff);
        less = true;
    } else {
        n1 = v1.wrapping_add(diff);
        n2 = v2.wrapping_sub(diff);
        less = false;
    }
    (n1, n2, less, diff)
}


fn add_antinode_part2(x1: usize, y1: usize, x2: usize, y2: usize, mut antinodes: Box<Vec<(usize, usize)>>, max_x: usize, max_y: usize) -> Box<Vec<(usize, usize)>> {
    antinodes.push((x1, y1));
    antinodes.push((x2, y2));

    let (xn1, xn2, less_x, diff_x) = calculate_xy_val(x1, x2);
    let (yn1, yn2, less_y, diff_y) = calculate_xy_val(y1, y2);
    if xn1 < max_x && yn1 < max_y {
        antinodes.push((xn1, yn1));
    }

    if xn2 < max_x && yn2 < max_y {
        antinodes.push((xn2, yn2));
    }

    antinodes = add_new_till_oob(xn1, yn1, diff_x, diff_y, less_x, less_y, antinodes, max_x, max_y);
    add_new_till_oob(xn2, yn2, diff_x, diff_y, !less_x, !less_y, antinodes, max_x, max_y)
}

fn add_new_till_oob(x: usize, y: usize, dx: usize, dy: usize, less_x: bool, less_y: bool, mut antinodes: Box<Vec<(usize, usize)>>, max_x: usize, max_y: usize)-> Box<Vec<(usize, usize)>> {
    let (nx, ny);
    if less_x {
        nx = x.wrapping_sub(dx);
    } else {
        nx = x.wrapping_add(dx);
    }
    if less_y {
        ny = y.wrapping_sub(dy);
    } else {
        ny = y.wrapping_add(dy);
    }
    if nx >= max_x || ny >= max_y {
        return antinodes;
    }
    antinodes.push((nx, ny));

    add_new_till_oob(nx, ny, dx, dy, less_x, less_y, antinodes, max_x, max_y)
}