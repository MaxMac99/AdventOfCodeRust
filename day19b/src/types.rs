use std::ops::{Index, IndexMut};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq)]
pub struct Workflow {
    pub rules: Vec<Rule>,
    pub default: Destination,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rule {
    pub category: Category,
    pub condition: Condition,
    pub value: u16,
    pub destination: Destination,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Range {
    pub start: u16,
    pub end: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
    pub x: Range,
    pub m: Range,
    pub a: Range,
    pub s: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Destination {
    Accepted,
    Rejected,
    Ref(String),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Condition {
    Larger,
    Lower,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumIter)]
pub enum Category {
    X,
    M,
    A,
    S,
}

impl Workflow {
    pub fn create_branches(&self, entry: &Entry) -> Vec<(Entry, &Destination)> {
        let mut last_false = Some(entry.clone());
        let mut results: Vec<(Entry, &Destination)> = self.rules.iter()
            .filter_map(|rule| {
                if let Some(last) = &last_false {
                    let (true_entry, false_entry) = rule.split(last);
                    last_false = false_entry;
                    true_entry.map(|entry| (entry, &rule.destination))
                } else {
                    None
                }
            })
            .collect();
        if let Some(default) = last_false {
            results.push((default, &self.default));
        }
        results
    }
}

impl Entry {
    pub fn permutation(&self) -> u64 {
        Category::iter()
            .map(|category| self[category].permutation())
            .fold(1, |agg, val| agg * val)
    }
}

impl Index<Category> for Entry {
    type Output = Range;

    fn index(&self, i: Category) -> &Self::Output {
        match i {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

impl IndexMut<Category> for Entry {
    fn index_mut(&mut self, i: Category) -> &mut Self::Output {
        match i {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }
}

impl Rule {
    pub fn split(&self, entry: &Entry) -> (Option<Entry>, Option<Entry>) {
        let (l, r) = entry[self.category].split(self.value, self.condition);
        (l.map(|range| {
            let mut clone = entry.clone();
            clone[self.category] = range;
            clone
        }), r.map(|range| {
            let mut clone = entry.clone();
            clone[self.category] = range;
            clone
        }))
    }
}

impl Range {
    /// Returns range inside (left) and range outside (right)
    fn split(&self, value: u16, condition: Condition) -> (Option<Self>, Option<Self>) {
        match condition {
            Condition::Larger => if self.start > value {
                (Some(*self), None)
            } else if self.end <= value {
                (None, Some(*self))
            } else {
                (Some(Range::from(value + 1, self.end)), Some(Range::from(self.start, value)))
            }
            Condition::Lower => if self.end < value {
                (Some(*self), None)
            } else if self.start >= value {
                (None, Some(*self))
            } else {
                (Some(Range::from(self.start, value - 1)), Some(Range::from(value, self.end)))
            }
        }
    }

    fn permutation(&self) -> u64 {
        (self.end - self.start + 1) as u64
    }

    fn from(start: u16, end: u16) -> Self {
        Range { start, end }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split1() {
        let range = Range { start: 1, end: 10 };
        let condition = Condition::Lower;
        let value = 5;

        assert_eq!(range.split(value, condition), (Some(Range::from(1, 4)), Some(Range::from(5, 10))));
    }

    #[test]
    fn test_split2() {
        let range = Range { start: 1, end: 10 };
        let condition = Condition::Larger;
        let value = 5;

        assert_eq!(range.split(value, condition), (Some(Range::from(6, 10)), Some(Range::from(1, 5))));
    }

    #[test]
    fn test_split3() {
        let range = Range { start: 1, end: 10 };
        let condition = Condition::Larger;
        let value = 11;

        assert_eq!(range.split(value, condition), (None, Some(Range::from(1, 10))));
    }

    #[test]
    fn test_split4() {
        let range = Range { start: 1, end: 10 };
        let condition = Condition::Lower;
        let value = 11;

        assert_eq!(range.split(value, condition), (Some(Range::from(1, 10)), None));
    }

    #[test]
    fn test_split5() {
        let range = Range { start: 11, end: 20 };
        let condition = Condition::Larger;
        let value = 8;

        assert_eq!(range.split(value, condition), (Some(Range::from(11, 20)), None));
    }

    #[test]
    fn test_split6() {
        let range = Range { start: 11, end: 20 };
        let condition = Condition::Lower;
        let value = 8;

        assert_eq!(range.split(value, condition), (None, Some(Range::from(11, 20))));
    }

    #[test]
    fn test_entry_permutation() {
        let entry = Entry {
            x: Range { start: 1, end: 1 },
            m: Range { start: 1, end: 5 },
            a: Range { start: 1, end: 1 },
            s: Range { start: 1, end: 10 },
        };

        assert_eq!(entry.permutation(), 50);
    }

    #[test]
    fn test_range_permutation() {
        let range = Range { start: 1, end: 4 };

        assert_eq!(range.permutation(), 4);
    }

    #[test]
    fn test_range_permutation2() {
        let range = Range { start: 3990, end: 4000 };

        assert_eq!(range.permutation(), 11);
    }
}
