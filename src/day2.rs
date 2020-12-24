use std::str::FromStr;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("day2_input");
    let packages = input.lines()
                              .map(|l| l.parse::<Package>())
                              .map(|r| r.unwrap())
                              .collect::<Vec<Package>>();
    println!("There are {} packages", packages.len());
    println!("{:?}", packages.iter().take(5).collect::<Vec<&Package>>());
    let area = packages.iter()
            .map(|p| p.surface_area())
            .sum::<u32>();

    println!("The elfes need {} square feet", area);

    let ribbon = packages.iter()
            .map(|p| p.ribbon_length())
            .sum::<u32>();
    println!("The elfes need {} of ribbon", ribbon);
    Ok(())
}

#[derive(Debug)]
struct Package {
    width: u32,
    height: u32,
    length: u32,
}

impl Package {
    fn surface_area(&self) -> u32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
            + (self.length * self.width).min(self.width * self.height).min(self.height * self.length)
    }

    fn ribbon_length(&self) -> u32 {
        let smallest_circumference = (2*self.length + 2*self.width)
                                     .min(2*self.width + 2*self.height)
                                     .min(2*self.length + 2*self.height);
        let bow_length = self.width * self.height * self.length;
        smallest_circumference + bow_length
    }
}
impl FromStr for Package {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = s.split("x").collect::<Vec<&str>>();
        if chunks.len() != 3 {
            return Err(AdventError{cause: format!("malformed line {}", s)});
        }

        return Ok(Package{
            length: chunks[0].parse::<u32>()?,
            width: chunks[1].parse::<u32>()?,
            height: chunks[2].parse::<u32>()?,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let subject = Package{ width: 2, height: 3, length: 4};
        assert_eq!(subject.surface_area(), 58);
        assert_eq!(subject.ribbon_length(), 34);
    }

    #[test]
    fn test_example_two() {
        let subject = Package{ width: 1, height: 1, length: 10};
        assert_eq!(subject.surface_area(), 43);
        assert_eq!(subject.ribbon_length(), 14);
    }
}