struct Solution {
    input: String,
    position: usize,
}

impl Solution {
    fn parse_string<'a>(self: &mut Self, pattern: &'static str) -> bool {
        if self.position >= self.input.len() { return false }

        let string = &self.input[self.position..];
        
        let mut pos = 0;
        for char in string.chars() {
            if let Some(c) = pattern.chars().nth(pos) {
                if char == c { 
                    pos += 1;
                    if pos >= pattern.len() {
                        self.position += pos;
                        return true;
                    }
                } else {
                    break;
                }
            } 
        }
        false
    }

    fn parse_number(self: &mut Self, max_len: u32) -> Option<u32> {
        if self.position >= self.input.len() { return None }

        let string = &self.input[self.position..];

        let mut res = 0;
        let mut pos = 0;
        for char in string.chars() {
            if char.is_numeric() {
                if pos >= max_len { break; }
                let digit = char.to_digit(10).expect("This to be a number");
                res *= 10;
                res += digit;
                pos += 1;
            } else if pos > 0 {
                self.position += pos as usize;
                return Some(res)
            } else {
                break; 
            }
        }
        None
    }
}

fn part_one(contents: &str) -> u32 {
    let mut solution = Solution {
        input: String::from(contents),
        position: 0,
    };
    let mut result = 0;
    loop {
        if solution.position >= solution.input.len() { break; }

        if solution.parse_string("mul(") {
            if let Some(num1) = solution.parse_number(3) {
                if solution.parse_string(",") {
                    if let Some(num2) = solution.parse_number(3) {
                        if solution.parse_string(")") {
                            result += num1 * num2;
                        }
                    }
                }
            }
        } else {
            solution.position += 1;
        }
    }
    result
}

fn part_two(contents: &str) -> u32 {
    let mut solution = Solution {
        input: String::from(contents),
        position: 0,
    };
    let mut result = 0;
    let mut enabled = true;
    loop {
        if solution.position >= solution.input.len() { break; }

        if solution.parse_string("do()") {
            enabled = true;
        } else if solution.parse_string("don't()") {
            enabled = false;
        } else if enabled && solution.parse_string("mul(") {
            if let Some(num1) = solution.parse_number(3) {
                if solution.parse_string(",") {
                    if let Some(num2) = solution.parse_number(3) {
                        if solution.parse_string(")") {
                            result += num1 * num2;
                        }
                    }
                }
            }
        } else {
            solution.position += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn my_test() {
        let p1 = part_one("mul(234,359)");
        assert_eq!(p1, 234 * 359);
    }

    #[test]
    fn example_p1() {
        let file_path = "examples/day03_p1.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_one(&contents);
        assert_eq!(p1, 161);
    }

    #[test]
    fn example_p2() {
        let file_path = "examples/day03_p2.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_two(&contents);
        assert_eq!(p1, 48);
    }

    #[test]
    fn input() {
        let file_path = "puzzle_inputs/day03.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");
        println!("{contents}");

        let p1 = part_one(&contents);
        assert_eq!(p1, 164730528);
        let p2 = part_two(&contents);
        assert_eq!(p2, 70478672);
    }
}

