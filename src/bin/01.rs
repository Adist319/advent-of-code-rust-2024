advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Parse input into two vectors of numbers
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    
    for line in input.lines() {
        if let Some((left, right)) = line.split_whitespace().collect::<Vec<_>>().split_first() {
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<u32>(), right.last()?.parse::<u32>()) {
                left_list.push(left_num);
                right_list.push(right_num);
            }
        }
    }
    // Sort both lists
    left_list.sort_unstable();
    right_list.sort_unstable();

    // Calculate total distance
    let total_distance: u32 = left_list.iter()
        .zip(right_list.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse input into two vectors of numbers
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    
    for line in input.lines() {
        if let Some((left, right)) = line.split_whitespace().collect::<Vec<_>>().split_first() {
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<u32>(), right.last()?.parse::<u32>()) {
                left_list.push(left_num);
                right_list.push(right_num);
            }
        }
    }

    // Calculate similarity score
    let similarity_score: u32 = left_list.iter()
        .map(|&num| {
            // Count occurrences of this number in right list
            let count = right_list.iter().filter(|&&x| x == num).count() as u32;
            // Multiply number by its count
            num * count
        })
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part_one(test_input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let test_input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part_two(test_input), Some(31));
    }

    #[test]
    fn test_part_two_file() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
