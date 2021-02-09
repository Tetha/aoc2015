use itertools::Itertools;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let buckets = vec![
    47,
    46,
    44,
    44,
    43,
    41,
    38,
    36,
    34,
    31,
    27,
    21,
    17,
    17,
    10,
    9,
    6,
    4,
    4,
    3];
    let target = 150;
    let (options, min_buckets) = solve(target, &buckets);
    println!("There are {} options", options);
    let valid_combination = buckets.iter()
                                         .copied()
                                         .combinations(min_buckets)
                                         .filter(|c| c.iter().copied().sum::<u32>() == target)
                                         .count();
    println!("The minimum buckets {} can be combined {} many times", min_buckets, valid_combination);
    Ok(())
}
fn solve(target: u32, sorted_buckets: &Vec<u32>) -> (u32, usize) {
    let mut options = vec![0; (target+1) as usize];
    let mut buckets_needed = vec![sorted_buckets.len()+1; (target + 1) as usize];
    options[target as usize] = 1;
    buckets_needed[target as usize] = 0;

    for &val in sorted_buckets.iter() {
        for i in val..=target {
            options[(i - val) as usize] += options[i as usize];
            if buckets_needed[i as usize] + 1 < buckets_needed[(i - val) as usize] {
                buckets_needed[(i - val) as usize] = buckets_needed[i as usize] + 1;
            }
        }
    }
    (options[0], buckets_needed[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let target = 25;
        let buckets = vec![20, 15, 10, 5, 5];
        let (combinations, buckets) = solve(target, &buckets);
        assert_eq!(4, combinations);
        assert_eq!(2, buckets);
    }
}