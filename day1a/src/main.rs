use std::collections::HashMap;
use std::error::Error;

use lazy_static::lazy_static;
use tokio::fs::File;
use tokio::io::BufReader;
use tokio_stream::wrappers::LinesStream;

use day1a::general_solution;
use shared::execute_solution_stream;

lazy_static! {
    static ref WORDS: HashMap<&'static str, i64> = vec![
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ].into_iter().collect();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    execute_solution_stream("day1a/input.txt", solution).await
}

async fn solution(lines: LinesStream<BufReader<File>>) -> Result<i64, Box<dyn Error>> {
    Ok(general_solution(lines, &WORDS).await)
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
        assert_eq!(result, 56465);
        Ok(())
    }
}
