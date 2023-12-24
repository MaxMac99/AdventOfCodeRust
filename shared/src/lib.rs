use std::error::Error;
use std::future::Future;
use std::time::Instant;

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

pub async fn execute_solution_stream<F, Fut>(filename: &'static str, solution: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(LinesStream<BufReader<File>>) -> Fut,
        Fut: Future<Output=Result<u64, Box<dyn Error>>>
{
    let lines = read_lines(filename).await?;

    let result = solution(lines).await?;

    println!("Result: {}", result);
    Ok(())
}

pub async fn execute_solution<F, Fut>(filename: &'static str, solution: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(String) -> Fut,
        Fut: Future<Output=Result<u64, Box<dyn Error>>>
{
    let lines = std::fs::read_to_string(filename)?;

    let start = Instant::now();
    let result = solution(lines).await?;
    let duration = start.elapsed();

    println!("Result: {} in {:?}", result, duration);
    Ok(())
}

pub async fn read_lines(filename: &'static str) -> Result<LinesStream<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    Ok(LinesStream::new(reader.lines()))
}
