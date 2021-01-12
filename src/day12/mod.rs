use std::collections::VecDeque;

use json::JsonValue;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let json = json::parse(input).unwrap();
    let a_sum = sum(&json, false);
    println!("{}", a_sum);
    let a_sum = sum(&json, true);
    println!("{}", a_sum);
    Ok(())
}
fn sum(input: &JsonValue, ignore_red: bool) -> f32 {
    let mut sum = 0.;
    let mut queue: VecDeque<&JsonValue> = VecDeque::new();
    queue.push_back(input);
    loop {
        match queue.pop_front() {
            None => return sum,
            Some(JsonValue::Null) => {},
            Some(JsonValue::Short(_)) => {},
            Some(JsonValue::String(_)) => {},
            Some(JsonValue::Number(n)) => sum = sum + f32::from(*n),
            Some(JsonValue::Boolean(_)) => {},
            Some(JsonValue::Object(o)) => {
                if ignore_red {
                    let has_red = itertools::any(o.iter(), |(_, v)| v == "red");
                    if !has_red {
                        o.iter().for_each(|(_,v)| queue.push_back(v))
                    }
                } else {
                    o.iter().for_each(|(_,v)| queue.push_back(v))
                }
            },
            Some(JsonValue::Array(values)) => {
                values.iter().for_each(|v| queue.push_back(v))
            }
        }
    }
}