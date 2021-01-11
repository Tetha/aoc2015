use itertools::join;

use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let mut input = vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2];
    for _ in 0..40 {
        input = look_and_say(&input);
    }
    println!("{}", input.len());
    for _ in 0..10 {
        input = look_and_say(&input);
    }
    println!("{}", input.len());
    Ok(())
}
fn look_and_say(sequence: &Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let mut pos = 0;
    while pos < sequence.len() {
        let start = sequence[pos];
        let mut count = 0;
        while pos < sequence.len() && sequence[pos] == start {
            count += 1;
            pos += 1;
        }
        result.push(count);
        result.push(start);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let input = vec![1];
        assert_eq!(vec![1, 1], look_and_say(&input));
    }

    #[test]
    fn test_example_two() {
        let input = vec![1, 1];
        assert_eq!(vec![2, 1], look_and_say(&input));
    }

    #[test]
    fn test_example_three() {
        let input = vec![2, 1];
        assert_eq!(vec![1, 2, 1, 1], look_and_say(&input));
    }

    #[test]
    fn test_example_four() {
        let input = vec![1, 2, 1, 1];
        assert_eq!(vec![1, 1, 1, 2, 2, 1], look_and_say(&input));
    }

    #[test]
    fn test_example_five() {
        let input = vec![1, 1, 1, 2, 2, 1];
        assert_eq!(vec![3, 1, 2, 2, 1, 1], look_and_say(&input));
    }
}