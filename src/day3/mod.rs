use std::collections::HashSet;

use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let program = include_str!("input");
    let houses = find_visited_houses(program);
    println!("He visited <{}>", houses.len());
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let program = include_str!("input");
    let houses = simulate_robo_santa(program);
    println!("He visited <{}> with robo", houses.len());
    Ok(())
}

fn find_visited_houses(program: &str) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    result.insert((x, y));
    for c in program.chars() {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => panic!("unhandled char {}", c),
        }
        result.insert((x, y));
    }
    result
}

fn simulate_robo_santa(program: &str) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;
    result.insert((0, 0));
    let chars = program.chars().collect::<Vec<char>>();
    for c in chars.chunks(2) {
        match c[0] {
            '>' => santa_x += 1,
            '<' => santa_x -= 1,
            '^' => santa_y -= 1,
            'v' => santa_y += 1,
            _ => panic!("unhandled char {}", c[0]),
        }
        result.insert((santa_x, santa_y));
        match c[1] {
            '>' => robo_x += 1,
            '<' => robo_x -= 1,
            '^' => robo_y -= 1,
            'v' => robo_y += 1,
            _ => panic!("unhandled char {}", c[1]),
        }
        result.insert((robo_x, robo_y));
    }
    result
}