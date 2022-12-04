pub fn part_one(input: &str) -> Option<u32> {
    let mut lookup: Vec<bool> = vec![false; 123];

    let total_priority = input
        .lines()
        .map(|rucksack| {
            // First we find the character that occurs in both halves of the string.
            // Rucksack length will always be an even number
            let divide_at = rucksack.len() / 2;
            let (f, s) = rucksack.split_at(divide_at);

            s.as_bytes().iter().for_each(|c| lookup[*c as usize] = true);

            let mut answer: Option<u8> = None;
            for &item in f.as_bytes().iter() {
                if lookup[item as usize] {
                    answer = Some(item);
                    break;
                }
            }
            lookup.clear();
            lookup.resize(123, false);

            answer.unwrap()
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
    // These three arrays function like HashMaps
    // "Does the character with byte-value X exist in the first rucksack"
    let mut first: Vec<bool> = vec![false; 123];
    let mut second: Vec<bool> = vec![false; 123];
    let mut third: Vec<bool> = vec![false; 123];

    let lines: Vec<&str> = input.lines().collect();
    let total_priority = lines
        .chunks(3)
        .map(|group| {
            group[0]
                .as_bytes()
                .iter()
                .for_each(|c| first[*c as usize] = true);
            group[1].as_bytes().iter().for_each(|c| {
                second[*c as usize] = true;
            });
            group[2].as_bytes().iter().for_each(|c| {
                third[*c as usize] = true;
            });

            // We store the answer outside to avoid ownership issues.
            // Clearing the vectors inside a loop is... rough.
            let mut answer: Option<u8> = None;
            let mut candidates: Vec<usize> = Vec::new();
            for (c, &in_first) in first.iter().enumerate() {
                if in_first && second[c] {
                    candidates.push(c);
                }
            }
            for candidate in candidates {
                if third[candidate] {
                    answer = Some(candidate as u8);
                    break;
                }
            }

            // Reset all the maps.
            first.clear();
            first.resize(123, false);
            second.clear();
            second.resize(123, false);
            third.clear();
            third.resize(123, false);

            answer.unwrap()
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
