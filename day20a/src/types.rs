use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use tokio::sync::Mutex;

pub trait Module: Debug {
    fn id(&self) -> u64;

    fn update_state(&mut self, input: bool, sender: u64) -> Option<bool>;

    fn count(&self) -> u64;

    fn reset(&mut self);

    fn add_destination(&mut self, destination: Arc<Mutex<dyn Module>>);

    fn get_destinations(&mut self) -> &Vec<Arc<Mutex<dyn Module>>>;
}

pub struct Broadcaster {
    count: u64,
    destinations: Vec<Arc<Mutex<dyn Module>>>,
}

pub struct FlipFlop {
    id: u64,
    count: u64,
    input: bool,
    on: bool,
    destinations: Vec<Arc<Mutex<dyn Module>>>,
}

pub struct Conjunction {
    id: u64,
    count: u64,
    states: HashMap<u64, bool>,
    destinations: Vec<Arc<Mutex<dyn Module>>>,
}

impl Broadcaster {
    pub fn new() -> Self {
        Self {
            count: 0,
            destinations: vec![],
        }
    }
}

impl FlipFlop {
    pub fn from(id: u64) -> Self {
        Self {
            id,
            count: 0,
            input: false,
            on: false,
            destinations: vec![],
        }
    }
}

impl Conjunction {
    pub fn from(id: u64) -> Self {
        Self {
            id,
            count: 0,
            states: Default::default(),
            destinations: vec![],
        }
    }
}

impl Debug for Broadcaster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Broadcaster #0 -> ")?;
        for dest in &self.destinations {
            write!(f, "{}, ", dest.blocking_lock().id())?;
        }
        Ok(())
    }
}

impl Module for Broadcaster {
    fn id(&self) -> u64 {
        0
    }

    fn update_state(&mut self, _input: bool, _sender: u64) -> Option<bool> {
        self.count += 1;
        Some(false)
    }

    fn count(&self) -> u64 {
        self.count
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn add_destination(&mut self, destination: Arc<Mutex<dyn Module>>) {
        self.destinations.push(destination);
    }

    fn get_destinations(&mut self) -> &Vec<Arc<Mutex<dyn Module>>> {
        &self.destinations
    }
}

impl Debug for FlipFlop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlipFlop #{} count: {}, input: {}, on: {} -> ", self.id, self.count, self.input, self.on)?;
        for dest in &self.destinations {
            write!(f, "{}, ", dest.blocking_lock().id())?;
        }
        Ok(())
    }
}

impl Module for FlipFlop {
    fn id(&self) -> u64 {
        self.id
    }

    fn update_state(&mut self, input: bool, _sender: u64) -> Option<bool> {
        self.count += 1;
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

    fn count(&self) -> u64 {
        self.count
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn add_destination(&mut self, destination: Arc<Mutex<dyn Module>>) {
        self.destinations.push(destination);
    }

    fn get_destinations(&mut self) -> &Vec<Arc<Mutex<dyn Module>>> {
        &self.destinations
    }
}

impl Debug for Conjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conjunction #{} count: {}, states: ", self.id, self.count)?;
        for (key, value) in &self.states {
            write!(f, "{}: {}, ", key, value)?;
        }
        Ok(())
    }
}

impl Module for Conjunction {
    fn id(&self) -> u64 {
        self.id
    }

    fn update_state(&mut self, input: bool, sender: u64) -> Option<bool> {
        self.count += 1;
        self.states.entry(sender).and_modify(|val| *val = input);
        Some(self.states.iter().all(|(_, state)| *state))
    }

    fn count(&self) -> u64 {
        self.count
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn add_destination(&mut self, destination: Arc<Mutex<dyn Module>>) {
        self.destinations.push(destination.clone());
        self.states.insert(destination.blocking_lock().id(), false);
    }

    fn get_destinations(&mut self) -> &Vec<Arc<Mutex<dyn Module>>> {
        &self.destinations
    }
}
