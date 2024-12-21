use std::cmp::Ordering;
use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let (ordering_rules, updates) = parse_input(reader)?;

        'updates: for pages_to_reproduce in updates {
            let mut printed = Vec::new();
            for page in pages_to_reproduce {
                let printed_page = page.clone();
                printed.push(printed_page);
                if !ordering_rules.contains_key(&page) {
                    continue;
                }
                let previous_pages = ordering_rules.get(&page).unwrap();
                for previous_page in previous_pages {
                    if printed.contains(&previous_page) {
                        continue 'updates;
                    }
                }
            }
            answer += printed[printed.len() / 2].parse::<usize>()?;
        }

        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let (ordering_rules, updates) = parse_input(reader)?;
        let mut incorrect_ordering = Vec::new();
        'updates: for pages_to_reproduce in updates {
            let mut printed = Vec::new();
            let pages = pages_to_reproduce.clone();
            for page in pages_to_reproduce {
                let printed_page = page.clone();
                printed.push(printed_page);
                if !ordering_rules.contains_key(&page) {
                    continue;
                }
                let previous_pages = ordering_rules.get(&page).unwrap();
                for previous_page in previous_pages {
                    if printed.contains(&previous_page) {
                        incorrect_ordering.push(pages.clone());
                        continue 'updates;
                    }
                }
            }
        }
        for pages_to_reproduce in incorrect_ordering {
            let mut pages = pages_to_reproduce.clone();
            pages.sort_by(|a, b| {
                if ordering_rules.contains_key(a) && ordering_rules.get(a).unwrap().contains(b) {
                    return Ordering::Less;
                }

                if ordering_rules.contains_key(b) && ordering_rules.get(b).unwrap().contains(a) {
                    return Ordering::Greater;
                }
                Ordering::Equal
            });
            answer += pages[pages.len() / 2].parse::<usize>()?;
        }

        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> Result<(HashMap<String, Vec<String>>, Vec<Vec<String>>)> {
    let mut ordering_rules = HashMap::new();
    let mut updates = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains('|') {
            let (p1, p2) = line.split('|').collect_tuple().unwrap();
            if !ordering_rules.contains_key(p1) {
                ordering_rules.insert(p1.to_string(), Vec::new());
            }
            ordering_rules.get_mut(p1).unwrap().push(p2.to_string());
            continue;
        }
        if line.contains(',') {
            let mut pages = Vec::new();
            for page in line.split(',') {
                pages.push(page.to_string());
            }
            updates.push(pages);
        }
    }
    Ok((ordering_rules, updates))
}