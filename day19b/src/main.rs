#![feature(slice_take)]

use std::collections::HashMap;
use std::error::Error;

use crate::parser::parse_workspace;
use crate::types::{Destination, Entry, Range, Workflow};

mod parser;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    shared::execute_solution("day19b/input.txt", solution).await
}

async fn solution(file: String) -> Result<u64, Box<dyn Error>> {
    let parts: Vec<String> = file.split("\n\n").map(String::from).collect();
    let workflows: HashMap<&str, Workflow> = parts.get(0)
        .expect("Could not find workspace part")
        .split("\n")
        .map(|line| parse_workspace(line).expect("Could not parse workspace").1)
        .collect();

    let destination = Destination::Ref("in".to_string());
    let branches = resolve_branches(&destination, &workflows);
    let branches = branches.iter()
        .map(|(entry, _)| entry)
        .collect();
    Ok(calculate_permutation(branches))
}

fn resolve_branches<'a>(destination: &'a Destination, workflows: &'a HashMap<&str, Workflow>) -> Vec<(Entry, &'a Destination)> {
    let mut found_all = false;
    let start_entry = Entry {
        x: Range { start: 1, end: 4000 },
        m: Range { start: 1, end: 4000 },
        a: Range { start: 1, end: 4000 },
        s: Range { start: 1, end: 4000 },
    };
    let mut branches: Vec<(Entry, &Destination)> = vec![(start_entry, destination)];

    while !found_all {
        found_all = true;
        branches = branches.iter()
            .flat_map(|(rules, destination)| match destination {
                Destination::Accepted => vec![(rules.clone(), *destination)],
                Destination::Rejected => Vec::new(),
                Destination::Ref(name) => {
                    found_all = false;
                    workflows.get(name.as_str())
                        .expect("Could not resolve name")
                        .create_branches(rules)
                }
            })
            .collect();
    }

    branches
}

fn calculate_permutation(branches: Vec<&Entry>) -> u64 {
    branches.iter()
        .map(|entry| entry.permutation())
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solution() -> Result<(), Box<dyn Error>> {
        let lines = std::fs::read_to_string("example.txt")?;
        let result = solution(lines).await?;
        assert_eq!(result, 167409079868000);
        Ok(())
    }
}
