use std::{collections::HashMap, str::FromStr};

use regex::Regex;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut aunts = input.lines()
         .map(|l| l.parse::<PossibleAunt>())
         .collect::<Result<Vec<PossibleAunt>, AdventError>>()?;

    let mut evidence: HashMap<AuntProperty, u32> = HashMap::new();
    evidence.insert(AuntProperty::Children, 3);
    evidence.insert(AuntProperty::Cats, 7);
    evidence.insert(AuntProperty::Samoyeds, 2);
    evidence.insert(AuntProperty::Pomeranians, 3);
    evidence.insert(AuntProperty::Akitas, 0);
    evidence.insert(AuntProperty::Vizslas, 0);
    evidence.insert(AuntProperty::Goldfish, 5);
    evidence.insert(AuntProperty::Trees, 3);
    evidence.insert(AuntProperty::Cars, 2);
    evidence.insert(AuntProperty::Perfumes, 1);

    for (property, count) in evidence {
        aunts = exclude_aunts(&aunts, property, count);
    }
    println!("{:?}\n", aunts);
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut aunts = input.lines()
         .map(|l| l.parse::<PossibleAunt>())
         .collect::<Result<Vec<PossibleAunt>, AdventError>>()?;

    let mut evidence: HashMap<AuntProperty, u32> = HashMap::new();
    evidence.insert(AuntProperty::Children, 3);
    evidence.insert(AuntProperty::Cats, 7);
    evidence.insert(AuntProperty::Samoyeds, 2);
    evidence.insert(AuntProperty::Pomeranians, 3);
    evidence.insert(AuntProperty::Akitas, 0);
    evidence.insert(AuntProperty::Vizslas, 0);
    evidence.insert(AuntProperty::Goldfish, 5);
    evidence.insert(AuntProperty::Trees, 3);
    evidence.insert(AuntProperty::Cars, 2);
    evidence.insert(AuntProperty::Perfumes, 1);

    for (property, count) in evidence {
        aunts = exclude_aunts_p2(&aunts, property, count);
    }
    println!("{:?}\n", aunts);
    Ok(())
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AuntProperty {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes
}

#[derive(Debug, Clone, Copy)]
enum KnownAuntProperty {
    Unknown,
    KnownValue(u32)
}

#[derive(Debug, Clone)]
struct PossibleAunt {
    number: String,
    knowns: HashMap<AuntProperty, KnownAuntProperty>,
}

impl FromStr for PossibleAunt {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let aunt_regex = Regex::new(
            r"Sue (?P<num>\d+): (?P<prop1_type>\w+): (?P<prop1_count>\d+), (?P<prop2_type>\w+): (?P<prop2_count>\d+), (?P<prop3_type>\w+): (?P<prop3_count>\d+)"
        ).unwrap();
        match aunt_regex.captures(s) {
            None => Err(AdventError{cause: "parse error".to_string()}),
            Some(caps) => {
                let mut knowns: HashMap<AuntProperty, KnownAuntProperty> = HashMap::new();

                let prop1_count = caps["prop1_count"].parse::<u32>()?;
                let prop1_type = caps["prop1_type"].parse::<AuntProperty>()?;
                knowns.insert(prop1_type, KnownAuntProperty::KnownValue(prop1_count));

                let prop2_count = caps["prop2_count"].parse::<u32>()?;
                let prop2_type = caps["prop2_type"].parse::<AuntProperty>()?;
                knowns.insert(prop2_type, KnownAuntProperty::KnownValue(prop2_count));

                let prop3_count = caps["prop3_count"].parse::<u32>()?;
                let prop3_type = caps["prop3_type"].parse::<AuntProperty>()?;
                knowns.insert(prop3_type, KnownAuntProperty::KnownValue(prop3_count));

                Ok(PossibleAunt{
                    number: caps["num"].to_string(),
                    knowns,
                })
            }
        }
    }
}

impl FromStr for AuntProperty {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "children"  => Ok(AuntProperty::Children),
            "cats"      => Ok(AuntProperty::Cats),
            "samoyeds"  => Ok(AuntProperty::Samoyeds),
            "pomeranians" => Ok(AuntProperty::Pomeranians),
            "akitas"    => Ok(AuntProperty::Akitas),
            "vizslas"   => Ok(AuntProperty::Vizslas),
            "goldfish"  => Ok(AuntProperty::Goldfish),
            "trees"     => Ok(AuntProperty::Trees),
            "cars"      => Ok(AuntProperty::Cars),
            "perfumes"  => Ok(AuntProperty::Perfumes),
            _ => Err(AdventError{cause: format!("unknown property {}", s)}),
        }
    }
}
fn exclude_aunts(possibilities: &Vec<PossibleAunt>, property: AuntProperty, value: u32) -> Vec<PossibleAunt> {
    possibilities.iter()
                 .filter(|&a| check_property(a, property, value))
                 .cloned()
                 .collect::<Vec<PossibleAunt>>()
                 
}

fn check_property(aunt: &PossibleAunt, property: AuntProperty, value: u32) -> bool {
    match aunt.knowns.get(&property).unwrap_or(&KnownAuntProperty::Unknown) {
        KnownAuntProperty::Unknown => true,
        KnownAuntProperty::KnownValue(k) => *k == value,
    }
}

fn exclude_aunts_p2(possibilities: &Vec<PossibleAunt>, property: AuntProperty, value: u32) -> Vec<PossibleAunt> {
    possibilities.iter()
                 .filter(|&a| check_property_p2(a, property, value))
                 .cloned()
                 .collect::<Vec<PossibleAunt>>()
                 
}

fn check_property_p2(aunt: &PossibleAunt, property: AuntProperty, value: u32) -> bool {
    match aunt.knowns.get(&property).unwrap_or(&KnownAuntProperty::Unknown) {
        KnownAuntProperty::Unknown => true,
        KnownAuntProperty::KnownValue(k) => match property {
            AuntProperty::Cats | AuntProperty::Trees => *k > value,
            AuntProperty::Goldfish | AuntProperty::Pomeranians => *k < value,
            _ => *k == value,
        }
    }
}