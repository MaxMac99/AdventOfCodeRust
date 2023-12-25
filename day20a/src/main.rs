#![feature(slice_take)]

use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_recursion::async_recursion;
use futures::future::{BoxFuture, join_all};
use futures::FutureExt;
use tokio::sync::Mutex;

use crate::parser::parse_machine;
use crate::types::Module;

mod parser;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    shared::execute_solution("day20a/input.txt", solution).await
}

async fn solution(content: String) -> Result<u64, Box<dyn Error>> {
    let modules = parse_machine(content)?;

    let broadcaster = modules.get("broadcaster").expect("Could not find broadcaster");

    for _ in 0..1000 {
        forward_signals(Box::pin(vec![broadcaster.clone()]), false, 0).await;
        println!(" ");
        for (name, module) in &modules {
            println!("{} = {:?}", name, module);
        }
    }

    let mut sum = 0;
    for (_, module) in modules {
        sum += module.blocking_lock().count();
    }

    Ok(sum as u64)
}

#[async_recursion]
async fn forward_signals(modules: Pin<Box<Vec<Arc<Mutex<dyn Module>>>>>, input: bool, sender: u64) {
    let futures = modules.iter()
        .filter_map(|module| update_state(module.clone(), input, sender))
        .collect::<Vec<_>>();

    join_all(futures).await;
}

fn update_state(module: Arc<Mutex<dyn Module>>, input: bool, sender: u64) -> Option<BoxFuture<'static, ()>> {
    if let Some(next_input) = module.clone().blocking_lock_owned().update_state(input, sender) {
        let destinations = Box::pin(module.clone().blocking_lock().get_destinations().clone());
        let sender = module.blocking_lock().id();
        Some(forward_signals(destinations, next_input, sender).boxed())
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::solution;

    #[tokio::test]
    async fn test_example1() -> Result<(), Box<dyn Error>> {
        let lines = std::fs::read_to_string("example1.txt")?;
        let result = solution(lines).await?;
        assert_eq!(result, 167409079868000);
        Ok(())
    }
}
