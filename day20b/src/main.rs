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
    shared::execute_solution("day20b/input.txt", solution).await
}

async fn solution(content: String) -> Result<u64, Box<dyn Error>> {
    run(&content)
}

fn run(content: &str) -> Result<u64, Box<dyn Error>> {
    let modules = parse_machine(content)?;

    let mut count = 0;
    let mut min = 0;
    loop {
        if forward_signals(count, &vec![String::from("broadcaster")], &modules, false, "broadcaster") {
            break;
        }
        count += 1;
        println!("({})", count);
    }
    println!("({}) {:?}", count, modules.get("tg"));

    Ok(count)
}

fn forward_signals(count: u64, send_to: &Vec<String>, modules: &HashMap<String, Rc<RefCell<dyn Module>>>, input: bool, sender: &str) -> bool {
    let mut rx_triggered = send_to.contains(&String::from("rx")) && !input;
    send_to.iter()
        .filter_map(|module_name| modules.get(module_name).map(|module| (module_name, module)))
        .filter_map(|(module_name, module)| module.borrow_mut().update_state(input, String::from(sender)).map(|output| (module_name, module, output)))
        .collect::<Vec<_>>()
        .iter()
        .for_each(|(module_name, module, output)| {
            let destinations = module.borrow().get_destinations().clone();
            if forward_signals(count, &destinations, modules, *output, module_name) {
                rx_triggered = true;
            }
        });
    if rx_triggered {
        println!("({}) Found rx: {:?}", count, modules.get("tg"));
    }
    rx_triggered
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
        assert_eq!(result, 0);
        Ok(())
    }
}
