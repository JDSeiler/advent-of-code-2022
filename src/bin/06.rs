fn all_different(packet: &[char]) -> bool {
    let mut seen = vec![false; 26];
    for &c in packet {
        let c_idx = (c as usize) - 97;
        if seen[c_idx] {
            return false;
        }
        seen[c_idx] = true;
    }
    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<char> = input.chars().collect();
    for (idx,window) in chars.windows(4).enumerate() {
        if all_different(window) {
            return Some((idx + 4) as u32);
        }
    }
    unreachable!();
}

pub fn part_two(input: &str) -> Option<u32> {
    /*
    N => the length of the string
    W => the length of the window
    Each window is processed in O(W)
    We process N - W + 1 windows
    So the complexity becomes O(NW - W^2 + W)
    
    Perhaps the worst case is W = N/2?
    (N^2)/2 - (N^2)/4 + N/2
    2(N^2)/4 - (N^2)/4 + N/2
    (N^2)/4 + N/2

    Clearly this algorithm is O(N^2) then.

    If we put in more effort and carefully track some state, we can do it in O(N).
    */
    let chars: Vec<char> = input.chars().collect();
    for (idx,window) in chars.windows(14).enumerate() {
        if all_different(window) {
            return Some((idx + 14) as u32);
        }
    }
    unreachable!();
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
