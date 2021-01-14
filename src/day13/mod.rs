use std::{collections::{HashMap, HashSet}, str::FromStr};

use itertools::Itertools;
use regex::Regex;

use crate::AdventError;


pub fn test() -> Result<(), AdventError> {
    let input = include_str!("test_input");
    let distances = input.parse::<Distances>()?;
    println!("The minimum distance is {:?}", distances.hamilton_distance());
    println!("The maximum distance is {:?}", distances.max_hamilton_distance());
    Ok(())
}

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut distances = input.parse::<Distances>()?;
    println!("The minimum distance is {:?}", distances.hamilton_distance());
    println!("The maximum distance is {:?}", distances.max_hamilton_distance());

    for person in &distances.persons {
        distances.satisfaction.insert(("me".to_string(), person.clone()), 0);
        distances.satisfaction.insert((person.clone(), "me".to_string()), 0);
    }
    distances.persons.insert("me".to_string());
    println!("The minimum distance is {:?}", distances.hamilton_distance());
    println!("The maximum distance is {:?}", distances.max_hamilton_distance());
    Ok(())
}
struct Distances {
    persons: HashSet<String>,
    satisfaction: HashMap<(String, String), i32>,
}

impl Distances {
    fn max_hamilton_distance(&self) -> Option<i32> {
        self.persons.iter().permutations(self.persons.len())
                          .map(|p| self.path_distance(p))
                          .max()
    }
    fn hamilton_distance(&self) -> Option<i32> {
        self.persons.iter().permutations(self.persons.len())
                          .map(|p| self.path_distance(p))
                          .min()
    }

    fn path_distance(&self, cities: Vec<&String>) -> i32 {
        let start = (*cities.first().unwrap()).clone();
        let last = (*cities.last().unwrap()).clone();
        let last_weight = self.satisfaction[&(start.clone(), last.clone())] + self.satisfaction[&(last, start)];

        let mut total_satisfaction = 0;
        for (p1, p2) in cities.iter().zip(cities.iter().skip(1)) {
            let p1 = p1.to_string();
            let p2 = p2.to_string();
            total_satisfaction += self.satisfaction[&(p1.clone(), p2.clone())] + self.satisfaction[&(p2.clone(), p1.clone())];
        }
        let res = total_satisfaction + last_weight;
        //println!("{:?} -> {}", cities, res);
        res
    }
}
impl FromStr for Distances {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut distances: HashMap<(String, String), i32> = HashMap::new();
        let mut cities: HashSet<String> = HashSet::new();
        let line_regex = Regex::new(r"(?P<first>\w+) would (?P<dir>gain|lose) (?P<weight>\d+) happiness units by sitting next to (?P<other>\w+).").unwrap();
        for line in s.lines() {
            if let Some(caps) = line_regex.captures(line) {
                let first = &caps["first"];
                let dir = &caps["dir"];
                let weight = caps["weight"].parse::<i32>().expect("regex ensured this");
                let other = &caps["other"];
                distances.insert((first.to_string(), other.to_string()), match dir {
                    "gain" => weight,
                    "lose" => -weight,
                    _ => panic!("unexpected value {}", dir),
                });
                cities.insert(first.to_string());
                cities.insert(other.to_string());
            } else {
                println!("Unable to parse {}", line);
            }
        }
        Ok(Distances{
            persons: cities,
            satisfaction: distances,
        })
    }
}