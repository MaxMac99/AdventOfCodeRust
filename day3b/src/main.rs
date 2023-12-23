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

async fn solution(lines: LinesStream<BufReader<File>>) -> Result<i64, Box<dyn Error>> {
    general_solution(lines, handle_line).await
}

fn handle_line(line: &String, previous: &Option<String>, next: &Option<String>) -> i64 {
    let re = Regex::new(r"\*").unwrap();
    re.captures_iter(line)
        .filter_map(|cap| cap.get(0))
        .filter_map(|val| find_surroundings(&val, line, previous, next))
        .sum()
}

fn find_surroundings(found: &Match, line: &String, previous: &Option<String>, next: &Option<String>) -> Option<i64> {
    let pos = found.start();
    let re = Regex::new(r"\d+").unwrap();
    let mut matches: Vec<i64> = re.captures_iter(line)
        .filter_map(|cap| cap.get(0))
        .filter(|val| val.end() == pos || val.start() == pos + 1)
        .map(|val| val.as_str().parse::<i64>().expect("Could not parse"))
        .collect();
    if matches.len() == 2 {
        return Some(matches[0] * matches[1]);
    }
    find_and_append_to_matches(previous, &pos, &re, &mut matches);
    find_and_append_to_matches(next, &pos, &re, &mut matches);

    if matches.len() == 2 {
        return Some(matches[0] * matches[1]);
    }
    None
}

fn find_and_append_to_matches(previous: &Option<String>, pos: &usize, re: &Regex, matches: &mut Vec<i64>) {
    if let Some(prev) = previous {
        matches.append(&mut re.captures_iter(prev)
            .filter_map(|cap| cap.get(0))
            .filter(|val| val.start() <= pos + 1 && pos <= &val.end())
            .map(|val| val.as_str().parse::<i64>().expect("Could not parse"))
            .collect::<Vec<i64>>()
            .clone())
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
        assert_eq!(result, 80253814);
        Ok(())
    }
}
