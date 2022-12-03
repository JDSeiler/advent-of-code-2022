fn get_total_calories(input: &str) -> Vec<u32> {
    let mut accum = 0;
    let mut all_totals: Vec<u32> = Vec::with_capacity(1000);
    input.split('\n').for_each(|row| {
        if row.is_empty() {
            all_totals.push(accum);
            accum = 0;
        } else {
            accum += row.parse::<u32>().unwrap();
        }
    });
    all_totals
}

pub fn part_one(input: &str) -> Option<u32> {
    let each_elf_calories = get_total_calories(input);
    let most_calories = each_elf_calories.iter().max().unwrap();

    Some(*most_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut each_elf_calories = get_total_calories(input);
    each_elf_calories.sort_by(|a, b| b.cmp(a));
    Some(
        each_elf_calories[0] +
        each_elf_calories[1] +
        each_elf_calories[2]
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
