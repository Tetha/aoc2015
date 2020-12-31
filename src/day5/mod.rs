use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");

    let result = input.lines()
         .filter(|&p| valid(p))
         .count();

    println!("{} passwords are nice", result);
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");

    let result = input.lines()
         .filter(|&p| revised_valid(p))
         .count();

    println!("{} passwords are nice", result);
    Ok(())
}
fn valid(password: &str) -> bool {
    !contains_naughty(password)
     && has_three_vowels(password)
     && has_double_letter(password)
}

fn revised_valid(password: &str) -> bool {
    check_for_pair(password) && has_repeating_letter_with_piece(password)
}

fn has_three_vowels(password: &str) -> bool {
    password.chars()
            .filter(|c| vec!['a', 'e', 'i', 'o', 'u'].contains(c))
            .count() >= 3
}

fn has_double_letter(password: &str) -> bool {
    password.chars().zip(password.chars().skip(1))
                    .any(|(c1, c2)| c1 == c2)
}

fn contains_naughty(password: &str) -> bool {
    password.contains("ab")
        || password.contains("cd")
        || password.contains("pq")
        || password.contains("xy")
}

fn check_for_pair(password: &str) -> bool {
    for (start, pair) in password.chars().zip(password.chars().skip(1)).enumerate() {
        if password[(start+2)..].contains(&format!("{}{}", pair.0, pair.1)) {
            return true;
        }
        /*
        let char_vec = password.chars().collect::<Vec<char>>();
        // check even occurences
        let even_pairs = char_vec.chunks_exact(2).filter(|c| c[0] == pair.0 && c[1] == pair.1).count();

        let odd_char_vec = password.chars().skip(1).collect::<Vec<char>>();
        let odd_pairs = odd_char_vec.chunks_exact(2).filter(|c| c[0] == pair.0 && c[1] == pair.1).count();

        if password == "xxyxx" {
            println!("{:?}", char_vec.chunks_exact(2).map(|c| (c[0], c[1])).collect::<Vec<(char, char)>>());
        }
        if 2 <= even_pairs || 2 <= odd_pairs {
            return true;
        }*/
    }
    return false;
}
fn has_repeating_letter_with_piece(password: &str) -> bool {
    password.chars().zip(password.chars().skip(1))
                    .zip(password.chars().skip(2))
                    .any(|((c1, _), c2)| c1 == c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_distinct_pairs() {
        assert_eq!(true, check_for_pair("xyxy"));
        assert_eq!(true, check_for_pair("aabcdefgaa"));
        assert_eq!(false, check_for_pair("aaa"));
        assert_eq!(true, check_for_pair("xxyxx"));
    }

    #[test]
    fn test_repeating_letter_with_separator() {
        assert_eq!(true, has_repeating_letter_with_piece("xyx"));
        assert_eq!(true, has_repeating_letter_with_piece("abcdefeghi"));
        assert_eq!(true, has_repeating_letter_with_piece("aaa"));
        assert_eq!(false, has_repeating_letter_with_piece("abcdefga"));
        assert_eq!(true, has_repeating_letter_with_piece("xxyxx"));
    }

    #[test]
    fn test_revised_valid() {
        assert_eq!(true, revised_valid("qjhvhtzxzqqjkmpb"));
        assert_eq!(true, revised_valid("xxyxx"));
        assert_eq!(false, revised_valid("uurcxstgmygtbstg"));
        assert_eq!(false, revised_valid("ieodomkazucvgmuy"));
    }
}