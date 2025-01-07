use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

#[derive(Debug)]
struct Robot{
    pos: (u8, u8),
    vel: (i8, i8),
}

impl Robot{
    fn tick(&mut self, ticks: i8, width: isize, height: isize){
        let (cur_pos_x, cur_pos_y) = self.pos;
        let (vel_x, vel_y) = self.vel;
        let mut new_pos_x = cur_pos_x as isize;
        let mut new_pos_y = cur_pos_y as isize;
        for _ in 1..=ticks {
            new_pos_x = new_pos_x + vel_x as isize;
            new_pos_y = new_pos_y + vel_y as isize;
            if new_pos_x < 0 {
                new_pos_x = new_pos_x + width;
            } else if new_pos_x >= width {
                new_pos_x = new_pos_x - width;
            }
            if new_pos_y < 0 {
                new_pos_y = new_pos_y + height;
            } else if new_pos_y >= height {
                new_pos_y = new_pos_y - height;
            }
        }
        self.pos = (new_pos_x as u8, new_pos_y as u8);
    }

    fn quadrant(&self, half_width: u8, half_height: u8) -> Option<usize> {
        let (x, y) = self.pos;
        if x == half_width || y == half_height {
            return None;
        }
        if (x < half_width) && (y < half_height) {
            return Some(0);
        }
        if x < half_width {
            return Some(2);
        }
        if y < half_height {
            return Some(1);
        }
        Some(3)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, width: u8, height: u8) -> Result<usize> {
        let mut robots = parse_robots(reader)?;
        move_robots(&mut robots, 100, width as isize, height as isize);
        let mut quadrants = [0,0,0,0];
        let half_width = width / 2;
        let half_height = height / 2;
        for robot in &robots {
            let quadrant = robot.quadrant(half_width, half_height);
            if quadrant.is_none() {
                continue;
            }
            quadrants[quadrant.unwrap()] += 1;
        }
        //print_grid(&robots, width, height);

        let mut answer = 1;
        for quadrant in &quadrants {
            answer *= quadrant;
        }

        Ok(answer)
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103)?);
    println!("Result = {}", result);
    // 216797152 = too low

    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_robots<R: BufRead>(reader: R) -> Result<Vec<Robot>> {
    let mut robots = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let (pos, vel) = line.split_whitespace().collect_tuple().unwrap();
        let (pos_x, pos_y) = pos[2..].split_once(',').unwrap();
        let (vel_x, vel_y) = vel[2..].split_once(',').unwrap();
        let pos = (pos_x.parse()?, pos_y.parse()?);
        let vel = (vel_x.parse()?, vel_y.parse()?);
        robots.push(Robot{pos, vel})
    }
    Ok(robots)
}

fn move_robots(robots: &mut Vec<Robot>, ticks: i8, width: isize, height: isize){
    for robot in robots.iter_mut() {
        robot.tick(ticks, width, height);
    }
}

#[allow(dead_code)]
fn print_grid(robots: &Vec<Robot>, width: u8, height: u8) {
    for y in 0..height {
        for x in 0..width {
            let mut count = 0;
            for robot in robots {
                if robot.pos == (x, y) {
                    count += 1;
                }
            }
            if count > 0 {
                print!("{}", count);
            }else {
                print!(".");
            }
        }
        println!();
    }
}