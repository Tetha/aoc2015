use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let code_chars = input.lines()
         .map(|l| count_code_characters(l))
         .sum::<usize>();
    let memory_chars = input.lines()
         .map(|l| count_memory_characters(l))
         .sum::<usize>();
    let escaped_chars = input.lines()
         .map(|l| count_escaped_characters(l))
         .sum::<usize>();
    println!("Difference is {}", code_chars - memory_chars);
    println!("2. part Difference is {}", escaped_chars - code_chars);
    Ok(())
}

fn count_escaped_characters(input: &str) -> usize {
    2 + input.chars()
             .map(|c| match c {
                '\\' => 2,
                '"' => 2,
                _ => 1,
             })
             .sum::<usize>()
}

fn count_code_characters(input: &str) -> usize {
    input.len()
}

fn count_memory_characters(input: &str) -> usize {
    let mut char_iter = input.chars();
    char_iter.next(); // skip first <">
    let mut memory_chars = 0;
    loop {
        match char_iter.next() {
            Some(c) => {
                match c {
                    '"' => {
                        return memory_chars;
                    },
                    '\\' => {
                        memory_chars += 1;
                        match char_iter.next() {
                            None => panic!("trailing \\"),
                            Some(c) => match c {
                                '\\' => {},
                                '"' => {},
                                'x' => {
                                    char_iter.next().expect("expect 1 digit after x");
                                    char_iter.next().expect("digit 2 after x");
                                },
                                _ => panic!("unknown escape sequence {}", c),
                            }
                        }
                    },
                    _ => {
                        memory_chars += 1;
                    },
                }
            },
            None => return memory_chars,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let input = "\"\"";
        assert_eq!(2, count_code_characters(input));
        assert_eq!(0, count_memory_characters(input));
        assert_eq!(6, count_escaped_characters(input));
    }

    #[test]
    fn test_example_two() {
        let input = "\"abc\"";
        assert_eq!(5, count_code_characters(input));
        assert_eq!(3, count_memory_characters(input));
        assert_eq!(9, count_escaped_characters(input));
    }

    #[test]
    fn test_example_three() {
        let input = "\"aaa\\\"aaa\"";
        assert_eq!(10, count_code_characters(input));
        assert_eq!(7, count_memory_characters(input));
        assert_eq!(16, count_escaped_characters(input));
    }

    #[test]
    fn test_example_four() {
        let input = "\"\\x27\"";
        assert_eq!(6, count_code_characters(input));
        assert_eq!(1, count_memory_characters(input));
        assert_eq!(11, count_escaped_characters(input));
    }

    #[test]
    fn test_double_backslash() {
        let input = "\"\\\\x27\"";
        assert_eq!(7, count_code_characters(input));
        assert_eq!(4, count_memory_characters(input));
    }
}