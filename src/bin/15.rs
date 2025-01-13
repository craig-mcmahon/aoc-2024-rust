use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

const TEST2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

#[derive(Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Debug)]
enum GridItem {
    Empty,
    Edge,
    Robot,
    Box,
}
impl Display for GridItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GridItem::Empty => write!(f, "."),
            GridItem::Edge => write!(f, "#"),
            GridItem::Robot => write!(f, "@"),
            GridItem::Box => write!(f, "0"),
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut grid, instructions, mut robot_pos) = parse_input(reader);

        println!("Starting {} instructions in pos: {},{}", instructions.len(), robot_pos.0, robot_pos.1);

        //print_grid(&grid);
        for instruction in instructions {
            robot_pos = do_move(robot_pos, &instruction, &mut grid);

            //print_grid(&grid);
        }

        let mut answer = 0;
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] == GridItem::Box {
                    answer += (100 * x) + y;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(2028, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(10092, part1(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
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

fn parse_input<R: BufRead>(reader: R) -> (Vec<Vec<GridItem>>, Vec<Instruction>, (usize, usize)) {
    let mut grid = Vec::new();
    let mut instructions = Vec::new();
    let mut part1 = true;
    let mut robot_pos = (0, 0);
    let mut x = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            part1 = false;
            continue;
        }
        if part1 {
            let mut row = Vec::new();
            let mut y = 0;
            for c in line.chars() {
                let grid_item = match c {
                    '.' => GridItem::Empty,
                    '#' => GridItem::Edge,
                    'O' => GridItem::Box,
                    '@' => {
                        robot_pos = (x, y);
                        GridItem::Robot
                    },
                    _ => panic!("Unexpected grid character: {}", c),
                };
                y += 1;
                row.push(grid_item);
            }
            grid.push(row);
        } else {
            for c in line.chars() {
                let instruction = match c {
                    '^' => Instruction::Up,
                    'v' => Instruction::Down,
                    '>' => Instruction::Right,
                    '<' => Instruction::Left,
                    _ => panic!("Invalid instruction character"),
                };
                instructions.push(instruction);
            }
        }
        x += 1;
    }
    (grid, instructions, robot_pos)
}


fn can_move(from_pos: (usize, usize), instruction: &Instruction, grid: &Vec<Vec<GridItem>>) -> bool {
    let (new_x, new_y) = get_new_pos(from_pos, &instruction);
    if grid[new_x][new_y] == GridItem::Edge {
        return false;
    }
    if grid[new_x][new_y] == GridItem::Empty {
        return true;
    }

    can_move((new_x, new_y), instruction, grid)
}

fn get_new_pos((x, y): (usize, usize), instruction: &Instruction) -> (usize, usize) {
    match instruction {
        Instruction::Up => (x-1, y),
        Instruction::Down => (x+1, y),
        Instruction::Left => (x, y-1),
        Instruction::Right => (x, y+1),
    }
}

fn do_move((x, y): (usize, usize), instruction: &Instruction, mut grid: &mut Vec<Vec<GridItem>>) -> (usize, usize) {
    //println!("Moving with instruction: {:?} From: {},{}", instruction, x, y);
    if !can_move((x, y), &instruction, grid) {
        return (x, y);
    }
    let (new_x, new_y) = get_new_pos((x, y), &instruction);
    let moved_item = grid[x][y].clone();
    grid[x][y] = GridItem::Empty;
    if grid[new_x][new_y] != GridItem::Empty {
        do_move((new_x, new_y), instruction, &mut grid);
    }
    grid[new_x][new_y] = moved_item;
    (new_x, new_y)
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<GridItem>>) {
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            print!("{}", grid[x][y]);
        }
        println!();
    }
}