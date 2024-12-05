use std::{cmp::Ordering, collections::{HashMap, HashSet}};

fn part_one(rules: &HashMap<i32, Vec<i32>>, updates: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for update in updates {
        if let Some(middle) = check_order(&rules, &update) {
            result += middle;
        }
    }
    result
}

fn part_two(rules: &HashMap<i32, Vec<i32>>, mut updates: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for update in &mut updates {
        if check_order(&rules, &update).is_none() {
            update.sort_by(|a, b| {
                if let Some(must_be_after) = rules.get(b) {
                    if must_be_after.contains(a) {
                        return Ordering::Less
                    }
                } 
                Ordering::Equal
            });
            result += update.get(update.len()/2).expect("update to have a middle");
        }
    }
    result
}

fn parse(contents: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = Vec::new();
    let mut parsing_rules = true;
    for line in contents.split("\n") {
        if line == "" && parsing_rules {
            parsing_rules = false;
            println!("{rules:?}");
        } else if parsing_rules {
            let ordering: Vec<_> = line.split("|").collect();
            let left: i32 = ordering.get(0).expect("left to be there").parse().expect("to be a valid number");
            let right: i32 = ordering.get(1).expect("right to be there").parse().expect("to be a valid number");
            if let Some(existing_rules) = rules.get_mut(&left) {
                existing_rules.push(right);
            } else {
                rules.insert(left, vec![right]);
            }
        } else if line != "" {
            let nums: Vec<_> = line.split(",").map(|num| num.parse::<i32>().expect("it to be a number")).collect();
            updates.push(nums);

        }
    }
    (rules, updates)
}

fn check_order(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Option<i32> {
    println!("{update:?}");
    let mut need_to_be_before = HashSet::new();
    for num in update.iter().rev() {
        if need_to_be_before.contains(num) { return None }
        if let Some(after) = rules.get(num) {
            for rule in after {
                need_to_be_before.insert(rule);
            }
        }
    }
    Some(*update.get(update.len()/2).expect("update to have a middle"))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn example_p1() {
        let file_path = "examples/day05.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");
        
        let (rules, updates) = parse(&contents);
        let p1 = part_one(&rules, updates);
        assert_eq!(p1, 143);
    }

    #[test]
    fn example_p2() {
        let file_path = "examples/day05.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let (rules, updates) = parse(&contents);
        let p1 = part_two(&rules, updates);
        assert_eq!(p1, 123);
    }

    #[test]
    fn input() {
        let file_path = "puzzle_inputs/day05.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let (rules, updates) = parse(&contents);
        let p1 = part_one(&rules, updates);
        assert_eq!(p1, 4135);

        let (rules, updates) = parse(&contents);
        let p2 = part_two(&rules, updates);
        assert_eq!(p2, 5285);
    }
}

