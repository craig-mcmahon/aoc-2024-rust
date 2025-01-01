use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
AAAA
BBCD
BBCC
EEEC
";
const TEST2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
const TEST3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const TEST4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

const TEST5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";


#[derive(Debug, PartialEq, Clone)]
struct Region {
    plant_type: char,
    coords: Vec<(usize, usize)>,
}
impl Region {
    fn calculate_price(&self, grid: &Vec<Vec<char>>) -> usize {
        self.calculate_perimeter(grid) * self.coords.len()
    }
    fn calculate_perimeter(&self, grid: &Vec<Vec<char>>) -> usize {
        let mut perimeter = 0;
        for (x, y) in self.coords.iter() {
            if *x == 0 {
                perimeter += 1;
            } else if grid[x-1][*y] != self.plant_type {
                perimeter += 1;
            }
            if *x == grid.len() - 1 {
                perimeter += 1;
            } else if grid[x+1][*y] != self.plant_type {
                perimeter += 1;
            }
            if *y == 0 {
                perimeter += 1;
            } else if grid[*x][y-1] != self.plant_type {
                perimeter += 1;
            }
            if *y == grid[0].len() - 1 {
                perimeter += 1;
            } else if grid[*x][y+1] != self.plant_type {
                perimeter += 1;
            }
        }
        perimeter
    }

    fn calculate_bulk_price(&self, grid: &Vec<Vec<char>>) -> usize {
        self.calculate_sides(grid) * self.coords.len()
    }

    fn calculate_sides(&self, grid: &Vec<Vec<char>>) -> usize {
        let mut sides = 0;
        let grid_max_x = grid.len() - 1;
        let grid_max_y = grid[0].len() - 1;
        for (x, y) in self.coords.iter() {
            // Calculate outside corners
            if (*x == 0 ||  grid[x-1][*y] != self.plant_type) && (*y == 0 ||  grid[*x][y - 1] != self.plant_type)  {
                sides +=  1;
            }
            if (*x == 0 ||  grid[x-1][*y] != self.plant_type) && (*y == grid_max_y ||  grid[*x][y + 1] != self.plant_type)  {
                sides +=  1;
            }

            if (*x == grid_max_x || grid[x+1][*y] != self.plant_type) && (*y == 0 ||  grid[*x][y - 1] != self.plant_type) {
                sides += 1;
            }

            if (*x == grid_max_x || grid[x+1][*y] != self.plant_type) && (*y == grid_max_y  ||  grid[*x][y + 1] != self.plant_type) {
                sides += 1;
            }

            // calculate inside corners
            if *x > 0 && *y > 0 && grid[x-1][*y] == self.plant_type && grid[*x][y-1] == self.plant_type && grid[x-1][y-1] != self.plant_type {
                sides += 1;
            }

            if *x < grid_max_x && *y < grid_max_y && grid[x+1][*y] == self.plant_type && grid[*x][y+1] == self.plant_type && grid[x+1][y+1] != self.plant_type {
                sides += 1;
            }
            if *x > 0 && *y < grid_max_y && grid[x-1][*y] == self.plant_type && grid[*x][y+1] == self.plant_type && grid[x-1][y+1] != self.plant_type {
                sides += 1;
            }
            if *x < grid_max_x && *y > 0 && grid[x+1][*y] == self.plant_type && grid[*x][y-1] == self.plant_type && grid[x+1][y-1] != self.plant_type {
                sides += 1;
            }
        }
        sides
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = build_2d_vec(reader)?;
        let regions = build_regions(&grid);

        let mut answer = 0;
        for region in regions {
            let region_answer = region.calculate_price(&grid);
            answer += region_answer;
        }

        Ok(answer)
    }

    assert_eq!(140, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(772, part1(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(1930, part1(BufReader::new(TEST3.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = build_2d_vec(reader)?;
        let regions = build_regions(&grid);

        let mut answer = 0;
        for region in regions {
            let region_answer = region.calculate_bulk_price(&grid);
            answer += region_answer;
        }

        Ok(answer)
    }

    assert_eq!(80, part2(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(236, part2(BufReader::new(TEST4.as_bytes()))?);
    assert_eq!(368, part2(BufReader::new(TEST5.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn build_regions(grid: &Vec<Vec<char>>) -> Vec<Region> {

    let mut visited = Vec::new();
    let mut regions = Vec::new();

    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            if visited.contains(&(x, y)) {
                continue;
            }
            let plant_type = grid[x][y];
            let coords = find_joining(plant_type, (x, y), &grid, &mut visited, Vec::new());
            let region = Region{coords, plant_type};
            regions.push(region);
        }
    }
    regions
}


fn find_joining(plant_type: char, (x, y): (usize, usize), grid: &Vec<Vec<char>>, visited: &mut Vec<(usize, usize)>, mut matching: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if visited.contains(&(x, y)) {
        return matching;
    }
    if grid[x][y] != plant_type {
        return matching;
    }
    matching.push((x, y));
    visited.push((x, y));

    if x > 0 {
        matching = find_joining(plant_type, (x-1, y), grid, visited, matching);
    }
    if y > 0 {
        matching = find_joining(plant_type, (x, y-1), grid, visited, matching);
    }

    if x < grid.len() - 1 {
        matching = find_joining(plant_type, (x+1, y), grid, visited, matching);
    }
    if y < grid[0].len() - 1 {
        matching = find_joining(plant_type, (x, y+1), grid, visited, matching);
    }

    matching
}