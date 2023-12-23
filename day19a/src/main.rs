#![feature(slice_take)]

use std::collections::HashMap;
use std::error::Error;

use nom::InputTake;
use tokio_stream::StreamExt;

use crate::parser::{parse_entry, parse_workspace};
use crate::types::{Destination, Entry, Workflow};

mod parser;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    shared::execute_solution("day19a/input.txt", solution).await
}

async fn solution(file: String) -> Result<u64, Box<dyn Error>> {
    let parts: Vec<String> = file.split("\n\n").map(String::from).collect();
    let workflows: HashMap<&str, Workflow> = parts.get(0)
        .expect("Could not find workspace part")
        .split("\n")
        .map(|line| parse_workspace(line).expect("Could not parse workspace"))
        .collect();

    let entries: Vec<Entry> = parts.get(1)
        .expect("Could not find entries")
        .split("\n")
        .map(|line| parse_entry(line).expect("Could not parse entry").1)
        .collect();

    Ok(entries.iter()
        .map(|entry| (resolve_destination(entry, &Destination::Ref("in".to_string()), &workflows), entry))
        .filter(|(result, entry)| *result)
        .map(|(result, entry)| entry.sum())
        .sum())
}

fn resolve_destination(entry: &Entry, destination: &Destination, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut destination = destination;
    while let Destination::Ref(name) = destination {
        destination = workflows.get(name.as_str()).expect("Could not find reference").evaluate(entry);
    }
    match destination {
        Destination::Accepted => true,
        Destination::Rejected => false,
        Destination::Ref(_) => panic!("Something went terribly wrong")
    }
}

