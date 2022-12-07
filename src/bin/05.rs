#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize
}

fn parse_moves(moves: &str) -> Vec<Move> {
    moves
        .lines()
        .map(|line| {
            // move X from Y to Z
            // [move, X, from, Y, to, Z]
            // { X, Y, Z }
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            Move {
                amount: parts[1].parse::<usize>().unwrap(),
                from: parts[3].parse::<usize>().unwrap(),
                to: parts[5].parse::<usize>().unwrap()
            }
        }).collect()
}

type Stacks = Vec<Vec<char>>;

fn parse_stacks(diagram: &str) -> Stacks {
    // We happen to know there are 9 stacks in the input. We allocate an extra
    // so that we can use 1 indexing.
    // No sense in making it generic over any number of stacks just yet.
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 10];
    diagram.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(idx, char)| {
            if char.is_ascii_alphabetic() {
                /*
                Now for some horrid index math.
                index | stack
                1     | 1
                5     | 2
                9     | 3
                13    | 4
                17    | 5
                etc....

                Each stack adds 4 to the index. So the number of times 4 goes into
                your index is basically your stack number.
                */
                let stack_number = 1 + (idx / 4);
                stacks[stack_number].push(char);
            }
        })
    });
    // We add the characters top to bottom, so we have to reverse each stack.
    // I could maybe make this faster by using VecDeque, which removes
    // the need for reversal, since I can efficiently use the Deque as a stack
    // from the get-go.
    stacks.iter_mut().for_each(|stack| stack.reverse());
    stacks
}

fn format_answer(stacks: Stacks) -> String {
   let mut answer = String::from("");
   stacks.iter().for_each(|stack| {
       // crate is a reserved word
       if let Some(krate) = stack.last() {
            answer.push(*krate);
       }
   });
   answer
}

pub fn part_one(input: &str) -> Option<String> {
    let (diagram, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(diagram);
    let parsed_moves = parse_moves(moves);

    parsed_moves.iter().for_each(|m| {
        for _ in 0..m.amount {
            let krate = stacks[m.from].pop().unwrap();
            stacks[m.to].push(krate);
        }
    });
    let answer = format_answer(stacks);
    Some(answer)
}

pub fn part_two(input: &str) -> Option<String> {
    let (diagram, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(diagram);
    let parsed_moves = parse_moves(moves);

    parsed_moves.iter().for_each(|m| {
        let from_stack = stacks.get_mut(m.from).unwrap();
        let krates = from_stack.split_off(from_stack.len() - m.amount);

        for krate in krates {
            stacks[m.to].push(krate);
        }
    });
    let answer = format_answer(stacks);
    Some(answer)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
