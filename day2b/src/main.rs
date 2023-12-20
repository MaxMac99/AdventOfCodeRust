use std::error::Error;

use tokio::fs::File;
use tokio::io::BufReader;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::LinesStream;

use day2a::{parse_line, Set};
use shared::execute_solution;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    execute_solution("day2a/input.txt", solution).await
}

async fn solution(lines: LinesStream<BufReader<File>>) -> Result<i64, Box<dyn Error>> {
    Ok(lines
        .map(|line| line
            .unwrap_or_else(|err| panic!("Could not find line: {}", err)))
        .map(|line| calc_line(&line))
        .fold(0, |acc, x| acc + x).await)
}

fn calc_line(line: &String) -> i64 {
    let game = parse_line(line);

    let min_set = Set {
        red: game.sets.iter().map(|set| set.red).max().unwrap(),
        green: game.sets.iter().map(|set| set.green).max().unwrap(),
        blue: game.sets.iter().map(|set| set.blue).max().unwrap(),
    };
    min_set.red * min_set.green * min_set.blue
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
        assert_eq!(result, 72227);
        Ok(())
    }
}
