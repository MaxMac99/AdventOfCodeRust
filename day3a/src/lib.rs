use std::error::Error;

use tokio::fs::File;
use tokio::io::BufReader;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::LinesStream;

pub async fn general_solution<F>(mut lines: LinesStream<BufReader<File>>, handle_line: F) -> Result<u64, Box<dyn Error>>
    where
        F: Fn(&String, &Option<String>, &Option<String>) -> u64
{
    let mut previous_line: Option<String>;
    let mut current_line: Option<String> = None;
    let mut next_line: Option<String> = None;

    let mut sum = 0;
    while let Some(line) = lines.next().await {
        if let Ok(line) = line {
            previous_line = current_line.take();
            current_line = next_line.take();
            next_line = Some(line);

            if let Some(current) = &current_line {
                sum += handle_line(current, &previous_line, &next_line);
            }
        }
    }
    sum += handle_line(&next_line.unwrap(), &current_line.take(), &None);
    Ok(sum)
}