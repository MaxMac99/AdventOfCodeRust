use std::error::Error;

use tokio::fs::File;
use tokio::io::BufReader;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::LinesStream;

use day2a::{parse_line, Set};
use shared::execute_solution_stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    execute_solution_stream("day2a/input.txt", solution).await
}

async fn solution(lines: LinesStream<BufReader<File>>) -> Result<u64, Box<dyn Error>> {
    Ok(lines
        .map(|line| line
            .unwrap_or_else(|err| panic!("Could not find line: {}", err)))
        .map(|line| check_line(&line))
        .fold(0, |acc, x| acc + x).await)
}

fn check_line(line: &String) -> u64 {
    let game = parse_line(line);

    let given_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    if game.sets.iter()
        .all(|set| set.red <= given_set.red &&
            set.green <= given_set.green &&
            set.blue <= given_set.blue) {
        game.id
    } else {
        0
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
        assert_eq!(result, 2716);
        Ok(())
    }
}
