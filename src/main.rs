use std::env;
use std::num::ParseIntError;

mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> Result<(), AdventError> {
    let args: Vec<String> = env::args().collect();

    match &args[1] as &str {
        "day2_part1" => day2::part1(),
        "day3_part1" => day3::part1(),
        "day3_part2" => day3::part2(),
        "day4_test" => day4::test(),
        "day4_part1" => day4::part1(),
        "day5_part1" => day5::part1(),
        "day5_part2" => day5::part2(),
        _ => {
            println!("Unknown argument {}", args[0]);
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct AdventError {
    pub cause: String,
}

impl From<ParseIntError> for AdventError {
    fn from(src: ParseIntError) -> Self {
        AdventError{cause: src.to_string()}
    }
}
