use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let input = input.trim().replace("\r\n", "\n");
    let parts: Vec<&str> = input.split("\n\n").collect();
    let (rules_str, sequences_str) = (parts[0], parts[1]);
    
    // Parse rules
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in rules_str.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        if let Some((from_str, to_str)) = line.split_once('|') {
            if let (Ok(from), Ok(to)) = (from_str.trim().parse(), to_str.trim().parse()) {
                rules.entry(from).or_default().push(to);
            }
        }
    }
    
    // Parse sequences
    let sequences: Vec<Vec<u32>> = sequences_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let nums: Result<Vec<u32>, _> = line
                .trim()
                .split(',')
                .map(|n| n.trim().parse())
                .collect();
            nums.ok()
        })
        .collect();
    
    (rules, sequences)
}

fn is_valid_sequence(sequence: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    // Build adjacency list for numbers in this sequence
    let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
    
    // Initialize adjacency list for all numbers in sequence
    for &num in sequence {
        adj_list.insert(num, Vec::new());
    }
    
    // Build graph only for numbers that appear in our sequence
    for (&from, to_list) in rules {
        if sequence.contains(&from) {
            for &to in to_list {
                if sequence.contains(&to) {
                    adj_list.entry(from).or_default().push(to);
                }
            }
        }
    }
    
    // Create position map
    let positions: HashMap<u32, usize> = sequence
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();
    
    // Check if sequence respects all dependencies
    for (&from, to_list) in &adj_list {
        for &to in to_list {
            if positions[&from] > positions[&to] {
                return false;
            }
        }
    }
    
    true
}

fn topological_sort(nums: &[u32], rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, i32> = HashMap::new();
    let mut result = Vec::new();
    
    // Initialize
    for &num in nums {
        adj_list.insert(num, Vec::new());
        in_degree.insert(num, 0);
    }
    
    // Build graph
    for (&from, to_list) in rules {
        if nums.contains(&from) {
            for &to in to_list {
                if nums.contains(&to) {
                    adj_list.entry(from).or_default().push(to);
                    *in_degree.entry(to).or_default() += 1;
                }
            }
        }
    }
    
    // Start with nodes that have no dependencies
    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&num, _)| num)
        .collect();
    
    // Process queue
    while let Some(num) = queue.pop_front() {
        result.push(num);
        
        if let Some(neighbors) = adj_list.get(&num) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }
    
    result
}

fn get_middle_number(sequence: &[u32]) -> u32 {
    sequence[sequence.len() / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, sequences) = parse_input(input);
    
    let sum = sequences
        .iter()
        .filter(|seq| is_valid_sequence(seq, &rules))
        .map(|seq| get_middle_number(seq))
        .sum();
    
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, sequences) = parse_input(input);
    
    let sum = sequences
        .iter()
        .filter(|seq| !is_valid_sequence(seq, &rules))
        .map(|seq| {
            let sorted = topological_sort(seq, &rules);
            get_middle_number(&sorted)
        })
        .sum();
    
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::Day;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", Day::new(5).unwrap()));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", Day::new(5).unwrap()));
        assert_eq!(result, Some(123));
    }
}
