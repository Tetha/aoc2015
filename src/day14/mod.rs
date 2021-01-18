use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let reindeer = vec![
        Reindeer::new(27, 5, 132),
        Reindeer::new(22, 2, 41),
        Reindeer::new(11, 5, 48),
        Reindeer::new(28, 5, 134),
        Reindeer::new(4, 16, 55),
        Reindeer::new(14, 3, 38),
        Reindeer::new(3, 21, 40),
        Reindeer::new(18, 6, 103),
        Reindeer::new(18, 5, 84),
    ];

    let max_distance = reindeer.iter().map(|r| r.fly(2503)).max();
    println!("furthest distance: {:?}", max_distance);
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let reindeer = vec![
        Reindeer::new(27, 5, 132),
        Reindeer::new(22, 2, 41),
        Reindeer::new(11, 5, 48),
        Reindeer::new(28, 5, 134),
        Reindeer::new(4, 16, 55),
        Reindeer::new(14, 3, 38),
        Reindeer::new(3, 21, 40),
        Reindeer::new(18, 6, 103),
        Reindeer::new(18, 5, 84),
    ];
    /*
    let reindeer = vec![
        Reindeer::new(14, 10, 127),
        Reindeer::new(16, 11, 162),
    ];*/

    let mut points = vec![0; reindeer.len()];

    for time in 1..=2503 {
        let distances: Vec<u32> = reindeer.iter().map(|r| r.fly(time)).collect();
        let winning_distance = distances.iter().enumerate().max_by(|(_, d1), (_, d2)| d1.cmp(d2)).unwrap().1;
        for (i, dist) in distances.iter().enumerate() {
            if dist == winning_distance {
                points[i] = points[i] + 1;
            }
        }
        println!("t={}: {:?} -> {:?}", time, distances, points);
    }

    let total_winner = points.iter().enumerate().max_by(|(_, p1), (_, p2)| p1.cmp(p2)).unwrap().1;
    println!("Winner points: {}", total_winner);
    Ok(())
}
struct Reindeer {
    speed: u32,
    endurance: u32,
    rest: u32
}

impl Reindeer {
    fn new(speed: u32, endurance: u32, rest: u32) -> Reindeer {
        Reindeer{speed, endurance, rest}
    }
    fn fly(&self, time: u32) -> u32 {
        let mut remaining_time = time;
        let mut distance: u32 = 0;

        while remaining_time > 0 {
            if remaining_time >= self.endurance {
                remaining_time -= self.endurance;
                distance += self.speed * self.endurance;
            } else {
                distance += remaining_time * self.speed;
                break;
            }

            if remaining_time >= self.rest {
                remaining_time -= self.rest;
            } else {
                break;
            }
        }

        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comet_example() {
        let subject = Reindeer::new(14, 10, 127);
        let distance = subject.fly(1000);
        assert_eq!(1120, distance);
    }

    #[test]
    fn test_dancer_example() {
        let subject = Reindeer::new(16, 11, 162);
        let distance = subject.fly(1000);
        assert_eq!(1056, distance);
    }
}