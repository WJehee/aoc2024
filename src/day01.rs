use std::{collections::HashMap, fs, iter::zip};

fn part_one(contents: &str) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let lines = contents.split("\n");
    for line in lines {
        if let Some((l, r)) = line.split_once(" ") {
            let l: i32 = l.trim().parse().expect("this to be a number");
            let r: i32 = r.trim().parse().expect("this to be a number");
            left.push(l);
            right.push(r);
        }
    }
    left.sort();
    right.sort();
    let mut sum = 0;
    for (l, r) in zip(left, right) {
        let diff = (l - r).abs();
        sum += diff;
    }
    sum
}

fn part_two(contents: &str) -> i32 {
    let mut left = Vec::new();
    let mut counts = HashMap::new();

    let lines = contents.split("\n");
    for line in lines {
        if let Some((l, r)) = line.split_once(" ") {
            let l: i32 = l.trim().parse().expect("this to be a number");
            let r: i32 = r.trim().parse().expect("this to be a number");
            left.push(l);
            if let Some(n) = counts.get(&r) {
                counts.insert(r, n+1);
            } else {
                counts.insert(r, 1);
            }
        }
    }
    let mut sum = 0;
    for num in left {
        let count = counts.get(&num).unwrap_or(&0);
        sum += num * count;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        let file_path = "puzzle_inputs/day01.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_one(&contents);
        assert_eq!(p1, 1319616);
        let p2 = part_two(&contents);
        assert_eq!(p2, 27267728);
    }
}

