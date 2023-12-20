use std::collections::HashMap;

use tokio::fs::File;
use tokio::io::BufReader;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::LinesStream;

pub async fn general_solution(lines: LinesStream<BufReader<File>>, words: &HashMap<&'static str, i64>) -> i64 {
    lines
        .map(|line| line
            .unwrap_or_else(|err| panic!("Could not find line: {}", err)))
        .map(|line| create_number(&line, words)
            .unwrap_or_else(|err| panic!("Could not create number from {}: {}", line, err)))
        .fold(0, |acc, x| acc + x).await
}

fn create_number(line: &String, words: &HashMap<&'static str, i64>) -> Result<i64, String> {
    let first = words.iter()
        .map(|(word, val)| (line.find(word), val))
        .filter_map(|(word, val)| word.map(|word| (word, val)))
        .min_by_key(|&(a, _): &(usize, &i64)| a)
        .expect("Could not find any digit")
        .1;
    let last = words.iter()
        .map(|(word, val)| (line.rfind(word), val))
        .filter_map(|(word, val)| word.map(|word| (word, val)))
        .max_by_key(|&(a, _): &(usize, &i64)| a)
        .expect("Could not find any digit")
        .1;
    let val = format!("{}{}", first, last);
    Ok(val.parse().unwrap())
}
