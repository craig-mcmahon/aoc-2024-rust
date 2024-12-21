use anyhow::*;
use std::io::BufRead;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

pub fn build_2d_vec<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>> {
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

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
