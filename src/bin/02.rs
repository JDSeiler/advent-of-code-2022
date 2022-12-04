
/*
For first strategy :
65 66 67 == A B C
88 89 90 == X Y Z

A | X => Rock     (1pt)
B | Y => Paper    (2pt)
C | Z => Scissors (3pt)

Lose (+0pts)
Draw (+3pts)
Win  (+6pts)
*/
fn points_for_first_strategy(opponent: u8, you: u8) -> u32 {
    match (opponent, you) {
        (65, 88) => 4, // 1+3
        (65, 89) => 8, // 2+6
        (65, 90) => 3, // 3+0

        (66, 88) => 1, // 1+0
        (66, 89) => 5, // 2+3
        (66, 90) => 9, // 3+6

        (67, 88) => 7, // 1+6
        (67, 89) => 2, // 2+0
        (67, 90) => 6, // 3+3
        _ => unreachable!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_points = input
        .lines()
        .map(|l| {
            let bytes = l.as_bytes();
            points_for_first_strategy(bytes[0], bytes[2])
        })
        .sum();

    Some(total_points)
}

/*
For second strategy :
65 66 67 == A B C
88 89 90 == X Y Z

A => Rock     (1pt)
B => Paper    (2pt)
C => Scissors (3pt)

X => Lose (+0pts)
Y => Draw (+3pts)
Z => Win  (+6pts)
*/
fn points_for_second_strategy(opponent: u8, you: u8) -> u32 {
    match (opponent, you) {
        (65, 88) => 3, // Rock, must lose, play Scissors
        (65, 89) => 4, // Rock, must draw, play Rock
        (65, 90) => 8, // Rock, must win, play Paper

        (66, 88) => 1, // Paper, must lose, play Rock
        (66, 89) => 5, // Paper, must draw, play Paper
        (66, 90) => 9, // Paper, must win, play Scissors

        (67, 88) => 2, // Scissors, must lose, play Paper
        (67, 89) => 6, // Scissors, must draw, play Scissors
        (67, 90) => 7, // Scissors, must win, play Rock
        _ => unreachable!()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_points = input
        .lines()
        .map(|l| {
            let bytes = l.as_bytes();
            points_for_second_strategy(bytes[0], bytes[2])
        })
        .sum();

    Some(total_points)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
