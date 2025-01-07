use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

#[derive(Debug)]
struct Game {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}
const A_COST: usize = 3;
const B_COST: usize = 1;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let games = build_games(reader, 0)?;
        let mut answer = 0;

        for game in games {
            if let Some(tokens) = calculate_tokens(&game) {
                answer += tokens;
            }
        }
        Ok(answer)
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    //println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     let games = build_games(reader, 10000000000000)?;
    //     let mut answer = 0;
    //
    //     for game in games {
    //         if let Some(tokens) = calculate_tokens(&game) {
    //             answer += tokens;
    //         }
    //     }
    //     Ok(answer)
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

fn build_games<R: BufRead>(reader: R, prize_difficulty: usize) -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let mut lines =  reader.lines().flatten();
    loop {
        let button_a = parse_line(lines.next().unwrap())?;
        let button_b = parse_line(lines.next().unwrap())?;
        let (prize_x, prize_y) = parse_line(lines.next().unwrap())?;
        games.push(Game{button_a, button_b, prize: (prize_x+prize_difficulty, prize_y+prize_difficulty)});
        if lines.next().is_none() {
            break;
        }
    }

    Ok(games)
}

fn parse_line(line: String) -> Result<(usize, usize)> {
    let (_, coords_part) = line.split(": ").collect_tuple().unwrap();
    let (x, y) = coords_part.split(", ").collect_tuple().unwrap();
    let x = x[2..].to_string().parse::<usize>()?;
    let y = y[2..].to_string().parse::<usize>()?;
    Ok((x, y))
}

fn calculate_tokens(game: &Game) -> Option<usize> {
    let (prize_x, prize_y) = game.prize;
    let (a_x, a_y) = game.button_a;
    let (b_x, b_y) = game.button_b;
    let mut success = Vec::new();
    let max_a = (prize_x / a_x).max(prize_y / a_y);
    let max_b = (prize_x / b_x).max(prize_y / b_y);
    for a in (0..=max_a).rev() {
        for b in (0..=max_b).rev() {
            let x = (a_x * a) + (b_x * b);
            let y = (a_y * a) + (b_y * b);
            if x == prize_x && y == prize_y {
                success.push((a, b));
                break;
            }
            if x < prize_x || y < prize_y {
                break;
            }
        }
    }
    if success.len() == 0 {
        return None;
    }

    let mut min_score = 9999usize;
    for (a, b) in success {
        let score = (a * A_COST) + (b * B_COST);
        if score < min_score {
            min_score = score;
        }
    }
    Some(min_score)
}