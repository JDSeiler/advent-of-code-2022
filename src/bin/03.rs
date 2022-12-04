use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let total_priority = input
        .lines()
        .map(|rucksack| {
            // First we find the character that occurs in both halves of the string.
            // Rucksack length will always be an even number
            let divide_at = rucksack.len() / 2;
            let (f, s) = rucksack.split_at(divide_at);
            // I tried this "make an ordered slice of bytes and binary search"
            // because I thought it'd be faster than a HashSet, but it's about the
            // same if not slower.
            let mut second = s.as_bytes().to_owned();
            second.sort_unstable();

            for item in f.as_bytes().iter() {
                if second.binary_search(item).is_ok() {
                    return *item;
                }
            }
            // We will always find a duplicate, if we omit this `unreachable`
            // then the compiler rightly asserts that this `map` will sometimes
            // return () instead of u8.
            unreachable!()
        })
        .map(|duplicate_item| {
            // a-z => should map to 1-26
            // A-Z => should map to 27-52

            // Because capital letters come before lowercase letters in ASCII
            // We need two separate branches / two separate offsets.
            match duplicate_item {
                // a-z
                97..=122 => duplicate_item as u32 - 96,
                // A-Z
                65..=90 => duplicate_item as u32 - 38,
                _ => unreachable!(),
            }
        })
        .sum();

    Some(total_priority)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Removing this allocation could speed up the solution quite a bit.
    // But `chunks` is not stabilized on normal iterators yet, only on slices.
    let lines: Vec<&str> = input.lines().collect();
    let total_priority = lines
        .chunks(3)
        .map(|group| {
            // Allocating and using all these HashSets could also be very slow.
            // But I'm not sure if there's actually a faster way, considering
            // that in part_one I replaced a HashSet with binary search and it wasn't
            // any faster.
            let first: HashSet<char> = HashSet::from_iter(group[0].chars());
            let second: HashSet<char> = HashSet::from_iter(group[1].chars());
            let third: HashSet<char> = HashSet::from_iter(group[2].chars());

            // If anything it makes the solution very readable.
            for candidate in first.intersection(&second) {
                if third.contains(candidate) {
                    return *candidate;
                }
            }
            // At least one candidate WILL occur in `third`
            unreachable!();
        })
        .map(|duplicate_item| {
            // A final optimization for both parts would be to ditch iterators
            // completely so that I can compute the sum as I'm calculating the
            // priorities. I am assuming the final `sum` call at the end of this
            // iterator chain is another pass over every element that could be
            // avoided.

            // a-z => should map to 1-26
            // A-Z => should map to 27-52

            // Because capital letters come before lowercase letters in ASCII
            // We need two separate branches / two separate offsets.
            match duplicate_item {
                'a'..='z' => duplicate_item as u32 - 96,
                'A'..='Z' => duplicate_item as u32 - 38,
                _ => unreachable!(),
            }
        })
        .sum();

    Some(total_priority)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
