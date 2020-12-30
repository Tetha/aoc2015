use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    println!("iwrupvqb -> {}", mine("iwrupvqb"));
    Ok(())
}

pub fn test() -> Result<(), AdventError> {
    println!("abcdef -> {}", mine("abcdef"));
    println!("pqrstuv -> {}", mine("pqrstuv"));
    Ok(())
}

fn mine(input: &str) -> u32 {
    let mut candidate = 1;
    loop {
        if candidate % 100000 == 0 {
            //println!(".");
        }
        let hash_input = format!("{}{}", input, candidate);
        candidate += 1;
        let hash = md5::compute(&hash_input);
        //if hash[0] == 0 && hash[1] == 0 && hash[2] < 15 {
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            //println!("{} {:?} {} {:?}", hash_input, hash, hash.len(), &hash[..]);
            return candidate - 1;
        }
    }
}