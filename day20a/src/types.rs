use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Module {
    fn update_state(&mut self, input: bool, sender: usize);

    fn send_signal(&self);

    fn count(&self) -> usize;

    fn reset(&mut self);

    fn add_destination(&mut self, destination: Rc<RefCell<dyn Module>>);
}

pub struct Broadcaster {
    count: usize,
    destinations: Vec<Rc<RefCell<dyn Module>>>,
}

pub struct FlipFlop {
    id: usize,
    count: usize,
    input: bool,
    on: bool,
    destinations: Vec<Rc<RefCell<dyn Module>>>,
}

pub struct Conjunction {
    id: usize,
    count: usize,
    states: HashMap<usize, bool>,
    destinations: Vec<Rc<RefCell<dyn Module>>>,
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
    pub fn from(id: usize) -> Self {
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
    pub fn from(id: usize) -> Self {
        Self {
            id,
            count: 0,
            states: Default::default(),
            destinations: vec![],
        }
    }
}

impl Module for Broadcaster {
    fn update_state(&mut self, _input: bool, _sender: usize) {
        self.count += 1;
    }

    fn send_signal(&self) {
        for destination in self.destinations.iter() {
            destination.borrow_mut().update_state(false, 0);
        }
        for destination in self.destinations.iter() {
            destination.borrow().send_signal();
        }
    }

    fn count(&self) -> usize {
        self.count
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn add_destination(&mut self, destination: Rc<RefCell<dyn Module>>) {
        self.destinations.push(destination);
    }
}

impl Module for FlipFlop {
    fn update_state(&mut self, input: bool, _sender: usize) {
        self.count += 1;
        if !input {
            self.on = !self.on;
        }
        self.input = input;
    }

    fn send_signal(&self) {
        if !self.input {
            for destination in self.destinations.iter() {
                destination.borrow_mut().update_state(self.on, self.id)
            }
            for destination in self.destinations.iter() {
                destination.borrow().send_signal();
            }
        }
    }

    fn count(&self) -> usize {
        self.count
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn add_destination(&mut self, destination: Rc<RefCell<dyn Module>>) {
        self.destinations.push(destination);
    }
}

impl Module for Conjunction {
    fn update_state(&mut self, input: bool, sender: usize) {
        self.count += 1;
        self.states.entry(sender).and_modify(|val| *val = input);
    }

    fn send_signal(&self) {
        for destination in self.destinations.iter() {
            destination.borrow_mut().update_state(self.states.iter().all(|(_, state)| *state), self.id)
        }
        for destination in self.destinations.iter() {
            destination.borrow().send_signal();
        }
    }

    fn count(&self) -> usize {
        self.count
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn add_destination(&mut self, destination: Rc<RefCell<dyn Module>>) {
        self.destinations.push(destination);
    }
}
