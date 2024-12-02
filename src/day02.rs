use std::fs;

fn part_one() -> i32 {
    let file_path = "puzzle_inputs/day02.txt";
    let contents = fs::read_to_string(file_path)
        .expect("To read the file");

    let reports = contents.split("\n");
    let mut safe_reports = 0;
    for report in reports {
        let mut prev_level = None;
        for level in report.split(" ") {
            let level: i32 = level.parse().expect("Level to be a number");
            if let Some(prev) = prev_level {
                prev_level = Some(level);
            }
        }
        safe_reports += 1;
    }
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(part_one(), 0);
    }
}



