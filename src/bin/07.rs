/*
Embarrassingly, I could not figure out how to solve this one *in Rust*.
What I wanted to do was recreate the file-system tree, but difficulties
arose with memory ownership because I tried to keep a parent pointer, so that
I could emulate "cd .." operations.

Now, there are perfectly straightforward ways to solve this problem in Rust,
as I would find out later. I didn't spend enough time considering alternate ways
of looking at the problem and ended up frustrated. Once I was frustrated I was not
thinking very productively. After well over an hour of trying I bailed on
the problem and solved both parts quickly and without much trouble in Node JS.
You can see my solution in `07.js`

I'm pretty embarrassed and frustrated I didn't get this one in Rust, but sometimes
you don't perform as well as you want to, that's just life.
*/

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
