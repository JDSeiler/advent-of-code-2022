fn read_grid(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect()
}

/* 
Both my part 1 and part 2 solutions have horrible code duplication.
I recognize it, I don't think it's good, but I can't be bothered to 
figure out how to factor it out when the indexing logic is so fiddly.
It's just advent of code :)
*/

fn count_visible(grid: Vec<Vec<u32>>) -> u32 {
    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();
    let mut from_left = vec![vec![0; cols]; rows];
    let mut from_right = vec![vec![0; cols]; rows];
    let mut from_bottom = vec![vec![0; cols]; rows];
    let mut from_top = vec![vec![0; cols]; rows];


    let mut max = 0;
    for i in 0..rows {
        for j in 0..cols {
            from_left[i][j] = max;
            if grid[i][j] > max {
                max = grid[i][j]
            }
        }
        // For all of these, we reset max between rows or columns, because
        // each row/column is independent
        max = 0;
    }

    max = 0;
    for i in 0..rows {
        for j in (0..cols).rev() {
            from_right[i][j] = max;
            if grid[i][j] > max {
                max = grid[i][j]
            }
        }
        max = 0;
    }
    
    max = 0;
    for j in 0..cols {
        for i in (0..rows).rev() {
            from_bottom[i][j] = max;
            if grid[i][j] > max {
                max = grid[i][j]
            }
        }
        max = 0;
    }
    
    max = 0;
    for j in 0..cols {
        for i in 0..rows {
            from_top[i][j] = max;
            if grid[i][j] > max {
                max = grid[i][j]
            }
        }
        max = 0;
    }

    let mut visible_trees = 0;
    for i in 0..rows {
        for j in 0..cols {
            if i == 0 || i == rows-1 || j == 0 || j == cols-1 {
                visible_trees += 1;
            } else {
                let current_tree = grid[i][j];
                let maxes = [
                    from_left[i][j],
                    from_right[i][j],
                    from_bottom[i][j],
                    from_top[i][j],
                ];

                let visible_in_any_direction = maxes.iter().any(|&max| max < current_tree);
                if visible_in_any_direction {
                    visible_trees += 1;
                }
            }
        }
    }

    visible_trees
}

pub fn part_one(input: &str) -> Option<u32> {
    // This is a row major grid.
    let grid = read_grid(input);
    let visible_trees = count_visible(grid);

    Some(visible_trees)
}

fn highest_scenic_score(grid: Vec<Vec<u32>>) -> u32 {
    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();
    
    let mut best_score = 0;

    for i in 0..rows {
        for j in 0..cols {
            let mut vis_left: Option<usize> = None;
            let mut vis_right: Option<usize> = None;
            let mut vis_up: Option<usize> = None;
            let mut vis_down: Option<usize> = None;

            // Send a "ray" to the left
            for x in (0..j).rev() {
                if grid[i][x] >= grid[i][j] {
                    vis_left = Some(j - x);
                    break;
                }
            }
            // No trees are in our way and we're not on the left edge.
            if vis_left.is_none() && j != 0 {
                vis_left = Some(j);
            }

            // Send a "ray" to the right
            for x in (j+1)..cols {
                if grid[i][x] >= grid[i][j] {
                    vis_right = Some(x - j);
                    break;
                }
            }
            if vis_right.is_none() && j != cols-1 {
                vis_right = Some(cols-1 - j);
            }

            // Send a ray downwards
            for y in i+1..rows {
                if grid[y][j] >= grid[i][j] {
                    vis_down = Some(y - i);
                    break;
                }
            }
            if vis_down.is_none() && i != rows-1 {
                vis_down = Some(rows-1 - i);
            }

            for y in (0..i).rev() {
                if grid[y][j] >= grid[i][j] {
                    vis_up = Some(i - y);
                    break;
                }
            }
            if vis_up.is_none() && i != 0 {
                vis_up = Some(i);
            }

            let total_score = 
                vis_left.unwrap_or(0) *
                vis_right.unwrap_or(0) *
                vis_up.unwrap_or(0) *
                vis_down.unwrap_or(0);

            if total_score > best_score {
                best_score = total_score
            }
        }
    }

    best_score as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    /*
    Ok, I think I can solve this in a similar, approach, but I need to tweak things a bit.
    
    In each of my 4 grids (from_left, from_right, etc.) I store three pieces of information:
    1. The visibility score at this cell in the given direction.
    2. The tallest tree seen so far in this row/col.
    3. The index that value was found at.

    I must also update the max BEFORE I set the value in the grid.
    3 0 3 7 3
    (vis going left, tallest, idx)
    [
        (0, 3, 0),
        (1, 3, 0),
        (2, 3, 2),
        (3, 7, 3),
        (1, 7, 3)
    ]
    So long as we have the tallest tree so far and its location, we know that for the
    current cell we either:
    1. Are blocked by that tree, and we can use its index + our own to compute our visibility
    2. We are the new max height tree, and we go all the way to the edge. Our visibility is given by our idx
    */    
    let grid = read_grid(input);
    let top_score = highest_scenic_score(grid);

    Some(top_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
