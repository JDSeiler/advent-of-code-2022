fn parse_range(range: &str) -> (u32, u32) {
    let (start, end) = range.split_once('-').unwrap();
    (
        start.parse::<u32>().unwrap(),
        end.parse::<u32>().unwrap()
    )
}

struct CleaningPair {
    top_range: (u32, u32),
    bottom_range: (u32, u32)
}

fn is_redundant(c: CleaningPair) -> u32 {
    // Is the top range inside the bottom range?
    if c.bottom_range.0 <= c.top_range.0 && c.top_range.1 <= c.bottom_range.1 {
        return 1;
    }

    // Is the bottom range inside the top range?
    if c.top_range.0 <= c.bottom_range.0 && c.bottom_range.1 <= c.top_range.1 {
        return 1;
    }

    // We return a number when a boolean would fit better so that we can easily
    // count up all the cases where is_redundant == true
    return 0;
}

fn is_overlapping(c: CleaningPair) -> u32 {
    // Is the top range to the left of the bottom range?
    // or, is it to the right of the bottom range?
    if c.top_range.1 < c.bottom_range.0 || c.bottom_range.1 < c.top_range.0 {
        // Then they can't possibly overlap
        return 0;
    }

    // Same check, but see if bottom range is on the left/right
    if c.bottom_range.1 < c.top_range.0 || c.top_range.1 < c.bottom_range.0 {
        return 0;
    }

    // The two conditions are above are all of the cases where they ranges AREN'T
    // overlapping. So if they're not NOT overlapping... well... then they are...
    return 1;
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = CleaningPair> + 'a {
    input
        .lines()
        .map(|line| {
            let (first_range, second_range) = line.split_once(',').unwrap();
            CleaningPair {
                top_range: parse_range(first_range),
                bottom_range: parse_range(second_range)
            }
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let number_redundant_pairs: u32 = parse(input)
        .map(is_redundant)
        .sum();

    Some(number_redundant_pairs)
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_overlapping_pars: u32 = parse(input)
        .map(is_overlapping)
        .sum();

    Some(number_overlapping_pars)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
