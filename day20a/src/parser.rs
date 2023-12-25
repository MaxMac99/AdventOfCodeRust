use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::peek;
use nom::error::context;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{terminated, tuple};

use crate::types::{Broadcaster, Conjunction, FlipFlop, Module};

pub fn parse_machine(content: &str) -> Result<HashMap<String, Rc<RefCell<dyn Module>>>, Box<dyn Error>> {
    let modules: HashMap<String, Rc<RefCell<dyn Module>>> = content.split("\n")
        .map(|line| parse_item(line).expect("Could not parse line"))
        .map(|(name, item)| (String::from(name), create_module(item)))
        .collect();

    register_source("broadcaster", "", &modules);

    Ok(modules)
}

fn register_source(name: &str, source: &str, modules: &HashMap<String, Rc<RefCell<dyn Module>>>) {
    if let Some(module) = modules.get(name) {
        if let Ok(mut module_ref) = module.try_borrow_mut() {
            module_ref.register_input(String::from(source));
            drop(module_ref);
            for dest in module.borrow().get_destinations().clone() {
                register_source(&dest, name, modules);
            }
        }
    }
}

fn create_module(item: ParsedItem) -> Rc<RefCell<dyn Module>> {
    match item.parsed_type {
        ParsedType::Broadcaster => Rc::new(RefCell::new(Broadcaster::from(item.destinations))),
        ParsedType::FlipFlop => Rc::new(RefCell::new(FlipFlop::from(item.destinations))),
        ParsedType::Conjunction => Rc::new(RefCell::new(Conjunction::from(item.destinations))),
    }
}

fn parse_item(input: &str) -> IResult<&str, ParsedItem> {
    context("line", tuple((parse_type, terminated(alphanumeric1, tag(" -> ")), separated_list0(tag(", "), alphanumeric1))))(input)
        .map(|(_, res)| {
            let (parsed_type, name, destinations) = res;
            (name, ParsedItem {
                parsed_type,
                destinations,
            })
        })
}

fn parse_type(input: &str) -> IResult<&str, ParsedType> {
    context("type", alt((peek(tag("broadcaster")), tag("&"), tag("%"))))(input)
        .map(|(rest, val)| (rest, val.into()))
}

#[derive(Debug, PartialEq)]
struct ParsedItem<'a> {
    parsed_type: ParsedType,
    destinations: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
enum ParsedType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

impl From<&str> for ParsedType {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "broadcaster" => ParsedType::Broadcaster,
            "%" => ParsedType::FlipFlop,
            "&" => ParsedType::Conjunction,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use super::*;

    #[test]
    fn test_parse_type() {
        assert_eq!(
            parse_type("%test -> a, b"),
            Ok(("test -> a, b", ParsedType::FlipFlop))
        );
        assert_eq!(
            parse_type("&test -> a, b"),
            Ok(("test -> a, b", ParsedType::Conjunction))
        );
        assert_eq!(
            parse_type("broadcaster -> a, b"),
            Ok(("broadcaster -> a, b", ParsedType::Broadcaster))
        );
        assert_eq!(
            parse_type("test -> a, b"),
            Err(nom::Err::Error(nom::error::Error {
                input: "test -> a, b",
                code: ErrorKind::Tag,
            }))
        );
    }

    #[test]
    fn test_parse_item() {
        assert_eq!(
            parse_item("broadcaster -> a, b, c"),
            Ok(("broadcaster", ParsedItem {
                parsed_type: ParsedType::Broadcaster,
                destinations: vec!["a", "b", "c"],
            }))
        );
        assert_eq!(
            parse_item("%a -> bc"),
            Ok(("a", ParsedItem {
                parsed_type: ParsedType::FlipFlop,
                destinations: vec!["bc"],
            }))
        );
        assert_eq!(
            parse_item("&inv -> a, bc"),
            Ok(("inv", ParsedItem {
                parsed_type: ParsedType::Conjunction,
                destinations: vec!["a", "bc"],
            }))
        );
    }
}