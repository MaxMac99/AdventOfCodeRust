use std::error::Error;
use tokio::fs::File;
use std::future::Future;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

pub async fn execute_solution<F, Fut>(filename: &'static str, solution: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(LinesStream<BufReader<File>>) -> Fut,
        Fut: Future<Output=Result<i64, Box<dyn Error>>>
{
    let lines = read_lines(filename).await?;

    let result = solution(lines).await?;

    println!("Result: {}", result);
    Ok(())
}

pub async fn read_lines(filename: &'static str) -> Result<LinesStream<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    Ok(LinesStream::new(reader.lines()))
}
