use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub trait Module: Debug {
    fn update_state(&mut self, input: bool, sender: String) -> Option<bool>;

    fn get_destinations(&self) -> &Vec<String>;

    fn register_input(&mut self, input: String);
}

pub struct Broadcaster {
    destinations: Vec<String>,
}

pub struct FlipFlop {
    input: bool,
    on: bool,
    destinations: Vec<String>,
}

pub struct Conjunction {
    states: HashMap<String, bool>,
    destinations: Vec<String>,
}

impl Broadcaster {
    pub fn from(destinations: Vec<&str>) -> Self {
        Self {
            destinations: destinations.iter().map(|item| String::from(*item)).collect(),
        }
    }
}

impl FlipFlop {
    pub fn from(destinations: Vec<&str>) -> Self {
        Self {
            input: false,
            on: false,
            destinations: destinations.iter().map(|item| String::from(*item)).collect(),
        }
    }
}

impl Conjunction {
    pub fn from(destinations: Vec<&str>) -> Self {
        Self {
            states: Default::default(),
            destinations: destinations.iter().map(|item| String::from(*item)).collect(),
        }
    }
}

impl Debug for Broadcaster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Broadcaster -> {:?}", self.destinations)
    }
}

impl Module for Broadcaster {
    fn update_state(&mut self, _input: bool, _sender: String) -> Option<bool> {
        Some(false)
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn register_input(&mut self, _input: String) {}
}

impl Debug for FlipFlop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlipFlop input: {}, on: {} -> {:?}", self.input, self.on, self.destinations)
    }
}

impl Module for FlipFlop {
    fn update_state(&mut self, input: bool, _sender: String) -> Option<bool> {
        if !input {
            self.on = !self.on;
        }
        self.input = input;
        if !input {
            Some(self.on)
        } else {
            None
        }
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn register_input(&mut self, _input: String) {}
}

impl Debug for Conjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conjunction states: {:?}", self.states)
    }
}

impl Module for Conjunction {
    fn update_state(&mut self, input: bool, sender: String) -> Option<bool> {
        *self.states.get_mut(&sender).unwrap() = input;
        Some(!self.states.iter().all(|(_, state)| *state))
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn register_input(&mut self, input: String) {
        self.states.insert(input, false);
    }
}
