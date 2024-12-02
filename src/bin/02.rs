advent_of_code::solution!(2);

fn is_safe_sequence_with_skip(numbers: &[i32], skip_idx: Option<usize>) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let mut prev = None;
    let mut increasing = None;

    // Two-pointer approach to compare adjacent numbers, skipping the index if specified
    let mut i = 0;
    while i < numbers.len() {
        // Skip the number we want to remove
        if Some(i) == skip_idx {
            i += 1;
            continue;
        }

        let current = numbers[i];
        
        // If we have a previous number, check the difference
        if let Some(previous) = prev {
            let diff: i32 = current - previous;
            
            // Check if difference is valid (between 1 and 3)
            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }
            
            // Check if direction is consistent
            match increasing {
                None => {
                    increasing = Some(diff > 0);
                }
                Some(is_increasing) => {
                    if (diff > 0) != is_increasing {
                        return false;
                    }
                }
            }
        }
        
        prev = Some(current);
        i += 1;
    }
    
    true
}

fn is_safe_with_dampener(numbers: &[i32]) -> bool {
    // First check if it's already safe without removing anything
    if is_safe_sequence_with_skip(numbers, None) {
        return true;
    }

    // Try removing each number one at a time
    for skip_idx in 0..numbers.len() {
        if is_safe_sequence_with_skip(numbers, Some(skip_idx)) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count = 0;

    // Process each non-empty line
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        // Convert the line into numbers
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if this sequence is safe
        if is_safe_sequence_with_skip(&numbers, None) {
            safe_count += 1;
        }
    }

    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count = 0;

    // Process each non-empty line
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        // Convert the line into numbers
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if this sequence is safe with the dampener
        if is_safe_with_dampener(&numbers) {
            safe_count += 1;
        }
    }

    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
