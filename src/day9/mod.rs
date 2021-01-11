use std::{collections::{HashMap, HashSet}, str::FromStr};

use itertools::Itertools;
use regex::Regex;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let distances = input.parse::<Distances>()?;
    println!("The minimum distance is {:?}", distances.hamilton_distance());
    println!("The maximum distance is {:?}", distances.max_hamilton_distance());
    Ok(())
}
struct Distances {
    cities: HashSet<String>,
    distance: HashMap<(String, String), u32>,
}

impl Distances {
    fn max_hamilton_distance(&self) -> Option<u32> {
        self.cities.iter().permutations(self.cities.len())
                          .map(|p| self.path_distance(p))
                          .max()
    }
    fn hamilton_distance(&self) -> Option<u32> {
        self.cities.iter().permutations(self.cities.len())
                          .map(|p| self.path_distance(p))
                          .min()
    }

    fn path_distance(&self, cities: Vec<&String>) -> u32 {
        cities.iter().zip(cities.iter().skip(1))
                     .map(|(&a, &b)| self.distance[&(a.clone(), b.clone())])
                     .sum()
    }
}
impl FromStr for Distances {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut distances: HashMap<(String, String), u32> = HashMap::new();
        let mut cities: HashSet<String> = HashSet::new();
        let line_regex = Regex::new(r"(?P<city1>\w+) to (?P<city2>\w+) = (?P<distance>\d+)").expect("my regexes are right");
        for line in s.lines() {
            if let Some(caps) = line_regex.captures(line) {
                let city1 = &caps["city1"];
                let city2 = &caps["city2"];
                let distance = caps["distance"].parse::<u32>().expect("regex ensured this");
                distances.insert((city1.to_string(), city2.to_string()), distance);
                distances.insert((city2.to_string(), city1.to_string()), distance);
                cities.insert(city1.to_string());
                cities.insert(city2.to_string());
            } else {
                println!("Unable to parse {}", line);
            }
        }
        Ok(Distances{
            cities,
            distance: distances,
        })
    }
}