const MAX_DIFF: i32 = 3;

fn part_one(contents: &str) -> i32 {
    let reports = contents.split("\n").filter(|r| *r != "");
    let mut safe_reports = 0;
    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|level| level.trim().parse::<i32>().expect("Level to be a number"))
            .collect();
        if is_safe(&levels) { safe_reports += 1; }
    }
    safe_reports
}

fn part_two(contents: &str) -> i32 {
    let reports = contents.split("\n").filter(|r| *r != "");
    let mut safe_reports = 0;
    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|level| level.trim().parse::<i32>().expect("Level to be a number"))
            .collect();
        if is_safe(&levels) { safe_reports += 1; } else {
            for i in 0..levels.len() {
                let mut alt = levels.clone();
                alt.remove(i);

                if is_safe(&alt) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    safe_reports
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut prev_level = None;
    let mut increasing = None;
    for level in levels {
        if let Some(prev) = prev_level {
            let diff: i32 = prev - level;
            if diff == 0 || diff.abs() > MAX_DIFF {
                return false;
            }
            if let Some(inc) = increasing {
                if (inc && diff < 0) || (!inc && diff > 0) {
                    return false
                } 
            } else {
                if prev < level { increasing = Some(false) } else if prev > level { increasing = Some(true) } 
            }
        } 
        prev_level = Some(level);
    }
    true
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
        assert_eq!(part_two(&contents), 4);
    }

    #[test]
    fn input() {
        let file_path = "puzzle_inputs/day02.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");
        assert_eq!(part_one(&contents), 442);
        assert_eq!(part_two(&contents), 493);
    }
}

