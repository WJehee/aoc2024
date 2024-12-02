const MAX_DIFF: i32 = 3;

fn part_one(contents: &str) -> i32 {
    let reports = contents.split("\n").filter(|r| *r != "");
    let mut safe_reports = 0;
    for report in reports {
        let mut prev_level = None;
        let mut increasing = None;
        let mut safe = true;
        for level in report.split_whitespace() {
            let level: i32 = level.trim().parse().expect("Level to be a number");
            if let Some(prev) = prev_level {
                let diff: i32 = prev - level;
                if diff == 0 || diff.abs() > MAX_DIFF {
                    safe = false;
                    break;
                }
                if let Some(inc) = increasing {
                    if (inc && diff < 0) || (!inc && diff > 0) {
                        safe = false;
                        break;
                    } 
                } else {
                    if prev < level { increasing = Some(false) } else if prev > level { increasing = Some(true) } else { 
                        safe = false;
                        break;
                    }
                }
            } 
            prev_level = Some(level);
        }
        if safe { safe_reports += 1; }
    }
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example() {
        let file_path = "examples/day02.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");
        assert_eq!(part_one(&contents), 2);
    }

    #[test]
    fn input() {
        let file_path = "puzzle_inputs/day02.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");
        assert_eq!(part_one(&contents), 442);
    }
}

