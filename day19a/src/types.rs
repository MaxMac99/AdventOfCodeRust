#[derive(Debug, PartialEq)]
pub struct Workflow {
    pub rules: Vec<Rule>,
    pub default: Destination,
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub category: Category,
    pub condition: Condition,
    pub value: u64,
    pub destination: Destination,
}

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64,
}

#[derive(Debug, PartialEq)]
pub enum Destination {
    Accepted,
    Rejected,
    Ref(String),
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Larger,
    Lower,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Category {
    X,
    M,
    A,
    S,
}

impl Workflow {
    pub fn evaluate(&self, entry: &Entry) -> &Destination {
        self.rules.iter()
            .map(|rule| rule.evaluate(entry))
            .find(Option::is_some)
            .map(Option::unwrap)
            .unwrap_or(&self.default)
    }
}

impl Entry {
    pub fn get_value(&self, category: &Category) -> u64 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    pub fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl Rule {
    pub fn evaluate(&self, entry: &Entry) -> Option<&Destination> {
        if match self.condition {
            Condition::Larger => entry.get_value(&self.category) > self.value,
            Condition::Lower => entry.get_value(&self.category) < self.value
        } {
            return Some(&self.destination);
        }
        None
    }
}

impl Destination {
    pub fn is_done(&self) -> bool {
        match self {
            Destination::Accepted => true,
            Destination::Rejected => true,
            Destination::Ref(_) => false,
        }
    }
}
