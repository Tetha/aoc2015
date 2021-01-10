
use std::collections::{HashMap, HashSet};
use regex::Regex;
use crate::AdventError;
use std::str::FromStr;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    solve(input)
}
pub fn test() -> Result<(), AdventError> {
    let input = include_str!("test_input");
    solve(input)
}

#[derive(Debug, Clone)]
enum Gate {
    Input(u16, String),
    Copy(String, String),
    And(String, String, String),
    Or(String, String, String),
    Not(String, String),
    Rshift(String, u32, String),
    Lshift(String, u32, String),
}

fn solve(input: &str) -> Result<(), AdventError> {
    let gates = parse_input(input)?;
    println!("parsed");
    //println!("{:?}", gates);

    //for gate in gates.keys() {
    //    println!("{}: {}", gate, get_value(&gates, gate));
    //}
    let mut cache: HashMap<String, u16> = HashMap::new();
    let a_output = get_value(&gates, "a", &mut cache);
    println!("{}: {}", "a", a_output);

    let mut modified_gates = gates.clone();
    modified_gates.insert("b".to_string(), Gate::Input(a_output, "b".to_string()));
    cache.clear();
    let a_output = get_value(&modified_gates, "a", &mut cache);
    println!("{}: {}", "a", a_output);
    Ok(())
}

fn get_value(gates: &HashMap<String, Gate>, gate: &str, cache: &mut HashMap<String, u16>) -> u16 {
    if gate == "1" {
        return 1;
    }
    if cache.contains_key(gate) {
        return cache[gate];
    }
    if !gates.contains_key(gate) {
        panic!("undefined gate {} in {:?}", gate, gates);
    }
    let result = match &gates[gate] {
        Gate::Input(v, _) => *v,
        Gate::And(a, b, _) => get_value(gates, a, cache) & get_value(gates, b, cache),
        Gate::Or(a, b, _) => get_value(gates, a, cache) | get_value(gates, b, cache),
        Gate::Not(v, _) => !get_value(gates, v, cache),
        Gate::Rshift(i, s, _) => get_value(gates, i, cache) >> s,
        Gate::Lshift(i, s, _) => get_value(gates, i, cache) << s,
        Gate::Copy(a, _) => get_value(gates, a, cache),
    };
    cache.insert(gate.to_string(), result);
    return result;
}
fn parse_input(input: &str) -> Result<HashMap<String, Gate>, AdventError> {
    let mut result = HashMap::new();
    for line in input.lines() {
        let gate = line.parse::<Gate>()?;
        let output = get_output(&gate);
        result.insert(output.clone(), gate);
    }
    return Ok(result);
}

fn get_output(gate: &Gate) -> &String {
    match gate {
        Gate::Input(_, o) => o,
        Gate::And(_, _, o) => o,
        Gate::Or(_, _, o) => o,
        Gate::Not(_, o) => o,
        Gate::Rshift(_, _, o) => o,
        Gate::Lshift(_, _, o) => o,
        Gate::Copy(_, o) => o,
    }
}
impl FromStr for Gate {
    type Err = AdventError;
    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let input_regex = Regex::new(r"^(?P<value>\d+) -> (?P<gate>\w+)$").unwrap();
        let copy_regex = Regex::new(r"^(?P<in>\w+) -> (?P<gate>\w+)$").unwrap();
        let and_regex = Regex::new(r"(?P<right>\w+) AND (?P<left>\w+) -> (?P<out>\w+)").unwrap();
        let or_regex = Regex::new(r"(?P<right>\w+) OR (?P<left>\w+) -> (?P<out>\w+)").unwrap();
        let not_regex = Regex::new(r"NOT (?P<in>\w+) -> (?P<out>\w+)").unwrap();
        let rshift_regex = Regex::new(r"(?P<input>\w+) RSHIFT (?P<shift>\d+) -> (?P<out>\w+)").unwrap();
        let lshift_regex = Regex::new(r"(?P<input>\w+) LSHIFT (?P<shift>\d+) -> (?P<out>\w+)").unwrap();

        if let Some(caps) = input_regex.captures(input) {
            return Ok(Gate::Input(
                caps["value"].parse::<u16>()?,
                caps["gate"].to_string(),
            ));
        }

        if let Some(caps) = copy_regex.captures(input) {
            return Ok(Gate::Copy(
                caps["in"].to_string(),
                caps["gate"].to_string(),
            ));
        }

        if let Some(caps) = and_regex.captures(input) {
            return Ok(Gate::And(
                caps["left"].to_string(),
                caps["right"].to_string(),
                caps["out"].to_string(),
            ))
        }

        if let Some(caps) = or_regex.captures(input) {
            return Ok(Gate::Or(
                caps["left"].to_string(),
                caps["right"].to_string(),
                caps["out"].to_string(),
            ))
        }

        if let Some(caps) = not_regex.captures(input) {
            return Ok(Gate::Not(
                caps["in"].to_string(),
                caps["out"].to_string(),
            ))
        }

        if let Some(caps) = rshift_regex.captures(input) {
            return Ok(Gate::Rshift(
                caps["input"].to_string(),
                caps["shift"].parse::<u32>()?,
                caps["out"].to_string(),
            ))
        }

        if let Some(caps) = lshift_regex.captures(input) {
            return Ok(Gate::Lshift(
                caps["input"].to_string(),
                caps["shift"].parse::<u32>()?,
                caps["out"].to_string(),
            ))
        }
        panic!("Cannot handle line {}", input);
    }
}
