use itertools::all;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let mut password = password_to_numbers("hepxcrrq");
    skip_to_next_valid_password(&mut password);
    println!("{}", password_to_string(&password));
    skip_to_next_valid_password(&mut password);
    println!("{}", password_to_string(&password));

    Ok(())
}
type Password = Vec<u8>;

fn skip_to_next_valid_password(password: &mut Password) {
    increment(password);
    while !is_valid_password(password) {
        increment(password);
    }
}

fn increment(password: &mut Password) {
    for i in (0..password.len()).rev() {
        if password[i] == 25 {
            password[i] = 0;
        } else {
            password[i] = password[i] + 1;
            return;
        }
    }
}

fn is_valid_password(password: &Password) -> bool {
    check_requirement_one(password)
        && check_requirement_two(password)
        && check_requirement_three(password)
}
fn check_requirement_one(password: &Password) -> bool {
    for i in 0..password.len()-2 {
        if password[i] + 1 == password[i+1] && password[i+1] + 1 == password[i+2] {
            return true;
        }
    }
    return false
}

fn check_requirement_two(password: &Password) -> bool {
    all(password, |&c| c != letter_to_number('i') && c != letter_to_number('l') && c != letter_to_number('o'))
}

fn check_requirement_three(password: &Password) -> bool {
    for i in 0..password.len() - 1 {
        if password[i] == password[i+1] {
            for j in (i+2)..password.len() - 1 {
                if password[j] == password[j+1] {
                    return true
                }
            }
        }
    }
    return false
}

fn password_to_numbers(password: &str) -> Password {
    password.chars().map(letter_to_number).collect()
}

fn letter_to_number(letter: char) -> u8 {
    (letter as u8) - ('a' as u8)
}

fn number_to_letter(letter: u8) -> char {
    return (('a' as u8) + letter) as char
}

fn password_to_string(password: &Password) -> String {
    password.iter().copied().map(number_to_letter).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let password = password_to_numbers("hijklmmn");
        assert_eq!(true, check_requirement_one(&password));
        assert_eq!(false, check_requirement_two(&password));
    }

    #[test]
    fn test_example_two() {
        let password = password_to_numbers("abbceffg");
        assert_eq!(false, check_requirement_one(&password));
        assert_eq!(true, check_requirement_three(&password));
    }

    #[test]
    fn test_example_three() {
        let password = password_to_numbers("abbcegjk");
        assert_eq!(false, check_requirement_three(&password));
    }

    #[test]
    fn test_example_four() {
        let mut password = password_to_numbers("abcdefgh");
        skip_to_next_valid_password(&mut password);
        let next_password = password_to_string(&password);
        assert_eq!("abcdffaa", next_password);

    }
}