use regex::Regex;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let instruction_decoder = InstructionDecoder::new();
    let mut grid = LightGrid::new(1000, 1000);
    let instructions = instruction_decoder.decode_input(input);
    for instruction in instructions {
        //println!("executing {:?}", instruction);
        grid.execute(instruction);
        //println!("{} lights are on", grid.count_lights());
    }
    println!("{} lights are on", grid.count_lights());
    println!("brightness: {}", grid.total_brightness());
    Ok(())
}
struct InstructionDecoder {
    matcher: Regex,
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

#[derive(Debug)]
enum Command {
    Toggle,
    TurnOn,
    TurnOff,
}

struct LightGrid {
    lights: Vec<u32>,
    width: usize,
}

impl LightGrid {
    fn new(width: usize, height: usize) -> LightGrid {
        LightGrid{
            lights: vec![0; width * height],
            width,
        }
    }

    fn count_lights(&self) -> usize {
        self.lights.iter().filter(|&l| *l > 0 ).count()
    }

    fn total_brightness(&self) -> u32 {
        self.lights.iter().copied().sum()
    }

    fn execute(&mut self, instruction: Instruction) {
        for y in instruction.start_y..=instruction.end_y {
            for x in instruction.start_x..=instruction.end_x {
                match instruction.command {
                    Command::Toggle => self.toggle(x, y),
                    Command::TurnOn => self.turn_on(x, y),
                    Command::TurnOff => self.turn_off(x, y),
                }
            }
        }
    }
    fn toggle(&mut self, x: usize, y: usize) {
        self.lights[x + y * self.width] += 2;
    }

    fn turn_off(&mut self, x: usize, y: usize) {
        if self.lights[x + y * self.width] >= 1 {
            self.lights[x + y * self.width] -= 1;
        }
    }

    fn turn_on(&mut self, x: usize, y: usize) {
        self.lights[x + y * self.width] += 1;
    }
}

impl InstructionDecoder {
    fn new() -> InstructionDecoder {
        return InstructionDecoder{
            matcher: Regex::new(r"(?P<command>turn off|turn on|toggle) (?P<start_x>\d+),(?P<start_y>\d+) through (?P<end_x>\d+),(?P<end_y>\d+)").unwrap(),
        }
    }

    fn decode_input(&self, input: &str) -> Vec<Instruction> {
        input.lines()
             .map(|l| self.decode_line(l))
             .collect::<Vec<Instruction>>()
    }

    fn decode_line(&self, line: &str) -> Instruction {
        let caps = self.matcher.captures(line).unwrap();
        let command = match &caps["command"] {
            "turn off" => Command::TurnOff,
            "turn on" => Command::TurnOn,
            "toggle" => Command::Toggle,
            _ => panic!("cannot handle {}", &caps["command"])
        };
        Instruction{
            command,
            start_x: caps["start_x"].parse::<usize>().unwrap(),
            start_y: caps["start_y"].parse::<usize>().unwrap(),
            end_x: caps["end_x"].parse::<usize>().unwrap(),
            end_y: caps["end_y"].parse::<usize>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let mut grid = LightGrid::new(1000, 1000);
        let instruction = Instruction{
            command: Command::TurnOn,
            start_x: 0,
            start_y: 0,
            end_x: 999,
            end_y: 999,
        };
        grid.execute(instruction);
        assert_eq!(1_000_000, grid.count_lights());
    }
}