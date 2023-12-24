use std::cmp::{max, min};
use std::error::Error;

use regex::{Match, Regex};
use tokio::fs::File;
use tokio::io::BufReader;
use tokio_stream::wrappers::LinesStream;

use day3a::general_solution;
use shared::execute_solution_stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    execute_solution_stream("day3a/input.txt", solution).await
}

async fn solution(lines: LinesStream<BufReader<File>>) -> Result<u64, Box<dyn Error>> {
    general_solution(lines, handle_line).await
}

fn handle_line(line: &String, previous: &Option<String>, next: &Option<String>) -> u64 {
    let re = Regex::new(r"\d+").unwrap();
    re.captures_iter(line)
        .filter_map(|cap| cap.get(0))
        .filter(|val| has_surrounding(val, &String::from(line), previous, next))
        .map(|val| val.as_str().parse::<u64>().expect("Could not parse"))
        .sum()
}

fn has_surrounding(found: &Match, line: &String, previous: &Option<String>, next: &Option<String>) -> bool {
    let mut chars: Vec<char> = vec![];
    let start: u64 = found.start() as u64;
    let end: u64 = found.end() as u64;
    if start > 0 {
        chars.push(line.chars().nth((start - 1) as usize).unwrap())
    }
    if end < line.len() as u64 {
        chars.push(line.chars().nth(end as usize).unwrap())
    }
    append_to_chars_in_range(previous, &mut chars, start, end);
    append_to_chars_in_range(next, &mut chars, start, end);

    let res = chars.iter().any(|val| !val.is_digit(10) && val != &'.');
    res
}

fn append_to_chars_in_range(previous: &Option<String>, chars: &mut Vec<char>, start: u64, end: u64) {
    if let Some(prev) = previous {
        let lower_bound = max(start - 1, 0) as usize;
        let upper_bound = min(end as usize, prev.len());
        chars.append(&mut prev.chars().skip(lower_bound).take(upper_bound - lower_bound + 1).collect::<Vec<char>>().clone());
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use shared::read_lines;

    use crate::solution;

    #[tokio::test]
    async fn test_solution() -> Result<(), Box<dyn Error>> {
        let lines = read_lines("input.txt").await?;
        let result = solution(lines).await?;
        assert_eq!(result, 530495);
        Ok(())
    }
}
