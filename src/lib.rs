use anyhow::*;
use std::io::BufRead;
use itertools::Itertools;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

pub fn build_2d_vec<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>> {
    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line?;
        data.push(line.chars().to_owned().collect_vec());
    }
    Ok(data)
}

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
