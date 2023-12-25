#![feature(slice_take)]

use std::error::Error;

use crate::parser::parse_machine;

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
        broadcaster.borrow_mut().update_state(false, 0);
        broadcaster.borrow().send_signal();
    }

    let mut sum = 0;
    for (_, module) in modules {
        sum += module.borrow().count();
    }

    Ok(sum as u64)
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
