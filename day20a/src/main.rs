#![feature(slice_take)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use crate::parser::parse_machine;
use crate::types::Module;

mod parser;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    shared::execute_solution("day20a/input.txt", solution).await
}

async fn solution(content: String) -> Result<u64, Box<dyn Error>> {
    run(&content)
}

fn run(content: &str) -> Result<u64, Box<dyn Error>> {
    let modules = parse_machine(content)?;

    let mut sum: (u64, u64) = (0, 0);
    for _ in 0..1000 {
        let result = forward_signals(&vec![String::from("broadcaster")], &modules, false, "broadcaster");
        sum.0 += result.0;
        sum.1 += result.1;
    }

    Ok(sum.0 * sum.1)
}

fn forward_signals(send_to: &Vec<String>, modules: &HashMap<String, Rc<RefCell<dyn Module>>>, input: bool, sender: &str) -> (u64, u64) {
    let mut count = (send_to.len() as u64 * input as u64, send_to.len() as u64 * !input as u64);
    send_to.iter()
        .filter_map(|module_name| modules.get(module_name).map(|module| (module_name, module)))
        .filter_map(|(module_name, module)| module.borrow_mut().update_state(input, String::from(sender)).map(|output| (module_name, module, output)))
        .collect::<Vec<_>>()
        .iter()
        .for_each(|(module_name, module, output)| {
            let destinations = module.borrow().get_destinations().clone();
            let result = forward_signals(&destinations, modules, *output, module_name);
            count.0 += result.0;
            count.1 += result.1;
        });
    count
}


#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::solution;

    #[tokio::test]
    async fn test_example1() -> Result<(), Box<dyn Error>> {
        let lines = std::fs::read_to_string("example1.txt")?;
        let result = solution(lines).await?;
        assert_eq!(result, 32000000);
        Ok(())
    }

    #[tokio::test]
    async fn test_example2() -> Result<(), Box<dyn Error>> {
        let lines = std::fs::read_to_string("example2.txt")?;
        let result = solution(lines).await?;
        assert_eq!(result, 11687500);
        Ok(())
    }
}
