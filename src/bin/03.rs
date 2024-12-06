advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        // Look for "mul(" pattern
        if i + 3 < chars.len() && 
           chars[i..i+4].iter().collect::<String>() == "mul(" {
            i += 4; // Skip past "mul("
            
            // Parse first number
            let mut num1_str = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                num1_str.push(chars[i]);
                i += 1;
            }

            // Check for comma
            if i < chars.len() && chars[i] == ',' {
                i += 1;
                
                // Parse second number
                let mut num2_str = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() {
                    num2_str.push(chars[i]);
                    i += 1;
                }

                // Check for closing parenthesis
                if i < chars.len() && chars[i] == ')' && 
                   !num1_str.is_empty() && !num2_str.is_empty() {
                    if let (Ok(num1), Ok(num2)) = (num1_str.parse::<u32>(), num2_str.parse::<u32>()) {
                        sum += num1 * num2;
                    }
                }
            }
        }
        i += 1;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut mul_enabled = true;

    while i < chars.len() {
        // Check for do() instruction
        if i + 2 < chars.len() && 
           chars[i..i+3].iter().collect::<String>() == "do(" &&
           i + 3 < chars.len() && chars[i+3] == ')' {
            mul_enabled = true;
            i += 4;
            continue;
        }

        // Check for don't() instruction
        if i + 5 < chars.len() && 
           chars[i..i+6].iter().collect::<String>() == "don't(" &&
           i + 6 < chars.len() && chars[i+6] == ')' {
            mul_enabled = false;
            i += 7;
            continue;
        }

        // Look for "mul(" pattern
        if i + 3 < chars.len() && 
           chars[i..i+4].iter().collect::<String>() == "mul(" {
            i += 4; // Skip past "mul("
            
            // Parse first number
            let mut num1_str = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                num1_str.push(chars[i]);
                i += 1;
            }

            // Check for comma
            if i < chars.len() && chars[i] == ',' {
                i += 1;
                
                // Parse second number
                let mut num2_str = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() {
                    num2_str.push(chars[i]);
                    i += 1;
                }

                // Check for closing parenthesis
                if i < chars.len() && chars[i] == ')' && 
                   !num1_str.is_empty() && !num2_str.is_empty() {
                    if let (Ok(num1), Ok(num2)) = (num1_str.parse::<u32>(), num2_str.parse::<u32>()) {
                        if mul_enabled {
                            sum += num1 * num2;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
