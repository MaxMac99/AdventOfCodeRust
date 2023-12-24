use nom::branch::alt;
use nom::bytes::complete::{is_not, take_until, take_while};
use nom::bytes::streaming::tag;
use nom::character::complete::{char, one_of};
use nom::character::streaming;
use nom::error::{Error, ErrorKind, ParseError};
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::delimited;

use crate::types::{Category, Condition, Destination, Rule, Workflow};

pub fn parse_workspace(input: &str) -> IResult<&str, (&str, Workflow)> {
    let (input, name) = take_until("{")(input)?;
    let (_, input) = delimited(char('{'), is_not("}"), char('}'))(input)?;
    let (input, default) = split_last(",", input)?;
    let (_, default) = parse_destination(default)?;
    let (_, rules) = separated_list0(char(','), parse_rule)(input)?;

    Ok((name, (name, Workflow {
        rules,
        default,
    })))
}

fn split_last<'a>(delimiter: &str, input: &'a str) -> IResult<&'a str, &'a str> {
    match input.rfind(delimiter) {
        None => Ok((input, "")),
        Some(idx) => Ok((&input[..idx], &input[idx + delimiter.len()..])),
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, category) = parse_category(input)?;
    let (input, condition) = parse_condition(input)?;
    let (input, value) = streaming::u16(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, destination) = parse_destination(input)?;

    Ok((input, Rule {
        category,
        condition,
        value,
        destination,
    }))
}

fn parse_category(input: &str) -> IResult<&str, Category> {
    let (input, category) = one_of("xmas")(input)?;
    match category {
        'x' => Ok((input, Category::X)),
        'm' => Ok((input, Category::M)),
        'a' => Ok((input, Category::A)),
        's' => Ok((input, Category::S)),
        _ => Err(nom::Err::Error(Error::from_error_kind("", ErrorKind::OneOf)))
    }
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, condition) = one_of("<>")(input)?;
    match condition {
        '>' => Ok((input, Condition::Larger)),
        '<' => Ok((input, Condition::Lower)),
        _ => Err(nom::Err::Error(Error::from_error_kind("", ErrorKind::OneOf)))
    }
}

fn parse_destination(input: &str) -> IResult<&str, Destination> {
    let (input, destination) = alt((tag("A"), tag("R"), take_while(char::is_alphabetic)))(input)?;
    match destination {
        "A" => Ok((input, Destination::Accepted)),
        "R" => Ok((input, Destination::Rejected)),
        _ => Ok((input, Destination::Ref(destination.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace() {
        let input = "gv{a>1626:A,x<2292:ex,a<1391:R,R}";

        assert_eq!(parse_workspace(input), Ok(("gv", ("gv", Workflow {
            rules: vec![
                Rule {
                    category: Category::A,
                    condition: Condition::Larger,
                    value: 1626,
                    destination: Destination::Accepted,
                },
                Rule {
                    category: Category::X,
                    condition: Condition::Lower,
                    value: 2292,
                    destination: Destination::Ref("ex".to_string()),
                },
                Rule {
                    category: Category::A,
                    condition: Condition::Lower,
                    value: 1391,
                    destination: Destination::Rejected,
                },
            ],
            default: Destination::Rejected,
        }))));
    }

    #[test]
    fn test_rule() {
        let input = "a>1626:A";

        assert_eq!(parse_rule(input), Ok(("", Rule {
            category: Category::A,
            condition: Condition::Larger,
            value: 1626,
            destination: Destination::Accepted,
        })));
    }
}

// fn evaluate_condition_permutations(conditions: Vec<&Condition>) -> usize {
//     let conditions_grouped: HashMap<PartVariable, Vec<&Condition>> = conditions.iter()
//         .fold(HashMap::new(), |mut agg, item| {
//             agg.entry(item.variable)
//                 .and_modify(|entry| entry.push(item))
//                 .or_insert(Vec::from(item));
//             agg
//         });
//     let permutations_list = conditions_grouped.iter()
//         .map(|(_, mut conditions)| conditions.sort_by(|cond_a, cond_b| cond_a.value.cmp(*cond_b.value))).collect()
//         .map(|conditions: Vec<&Condition>| {
//             let mut permutations = 0;
//             let mut start_value = 1;
//             conditions.iter()
//                 .for_each(|condition| {
//                     match condition.operator {
//                         Operator::LessThan => {
//                             permutations += condition.value - start_value;
//                             start_value = condition.value;
//                         },
//                         Operator::GreaterThan => {
//                             start_value = condition.value;
//                         }
//                     }
//                 });
//             let last_condition = conditions.last()
//                 .expect("no condition found");
//             match last_condition.operator {
//                 Operator::LessThan => {}
//                 Operator::GreaterThan => {
//                     permutations += 4000 - last_condition.value;
//                 }
//             }
//             permutations
//         });
//     permutations_list.iter().fold(0, |agg, item| item * agg)
// }