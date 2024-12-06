advent_of_code::solution!(4);

// The four possible diagonal MAS combinations
const PATTERNS: [((&str, i32, i32), (&str, i32, i32)); 4] = [
    (("MAS", -1, 1), ("MAS", 1, 1)),    // Both forward
    (("SAM", -1, 1), ("MAS", 1, 1)),    // Top-left backwards, bottom-left forwards
    (("MAS", -1, 1), ("SAM", 1, 1)),    // Top-left forwards, bottom-left backwards
    (("SAM", -1, 1), ("SAM", 1, 1)),    // Both backwards
];

fn check_diagonal(grid: &Vec<Vec<char>>, row: i32, col: i32, pattern: &str, dx: i32, dy: i32) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    for (i, target_char) in pattern.chars().enumerate() {
        let new_row = row + dx * i as i32;
        let new_col = col + dy * i as i32;
        
        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }
        
        if grid[new_row as usize][new_col as usize] != target_char {
            return false;
        }
    }
    true
}

fn check_x_mas(grid: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    // The center of the X must be 'A'
    if grid[row as usize][col as usize] != 'A' {
        return false;
    }

    for &((pattern1, dx1, dy1), (pattern2, dx2, dy2)) in PATTERNS.iter() {
        if check_diagonal(grid, row - dx1, col - dy1, pattern1, dx1, dy1) && 
           check_diagonal(grid, row - dx2, col - dy2, pattern2, dx2, dy2) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    if grid.is_empty() {
        return None;
    }
    
    let mut count = 0;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    for row in 0..rows {
        for col in 0..cols {
            if check_x_mas(&grid, row, col) {
                count += 1;
            }
        }
    }
    
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
