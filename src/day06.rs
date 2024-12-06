use std::collections::HashSet;

fn parse(contents: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut result = Vec::new();
    let mut guard_x = 0;
    let mut guard_y = 0;
    for (i, line) in contents.split("\n").enumerate() {
        if line.len() == 0 { continue }
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '^' | '>' | 'V' | '<' => { 
                    guard_x = i;
                    guard_y = j;
                    row.push('.');
                },
                c => {
                    row.push(c);
                }
            }
        }
        result.push(row);
    }
    (result, (guard_x, guard_y))
}

fn guard_routine(map: &Vec<Vec<char>>, mut guard_pos: (usize, usize), mut dir: (i32, i32)) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    println!("Initial guard position: {guard_pos:?}");
    loop {
        visited.insert((guard_pos.0, guard_pos.1));
        if     guard_pos.0 == 0 && dir.0 == -1
            || guard_pos.0 == map.len()-1 && dir.0 == 1
            || guard_pos.1 == 0 && dir.1 == -1
            || guard_pos.1 == map.get(0).expect("first row to exist").len()-1 {
                //println!("Out of bounds! position: {guard_pos:?}");
                break;
        }
        let next_pos = (guard_pos.0 as i32 + dir.0, guard_pos.1 as i32 + dir.1);
        if let Some(row) = map.get(next_pos.0 as usize) {
            if let Some(c) = row.get(next_pos.1 as usize) {
                match c {
                    '#' => {
                        dir = match dir {
                            // UP
                            (-1, 0) => { (0, 1) },
                            // RIGHT
                            (0, 1) => { (1, 0)},
                            // DOWN
                            (1, 0) => { (0, -1) },
                            // LEFT
                            (0, -1) => { (-1, 0) },
                            _ => { panic!("Invalid direction") }
                        }
                    },
                    '.' => { guard_pos = (next_pos.0 as usize, next_pos.1 as usize) },
                    _ => { panic!("Invalid character in map") }
                }
            }
        }
    }
    visited
}

fn guard_check_loop(map: &Vec<Vec<char>>, mut guard_pos: (usize, usize), mut dir: (i32, i32)) -> bool {
    let mut visited: HashSet<((usize, usize), (i32, i32))> = HashSet::new();
    //println!("Initial guard position: {guard_pos:?}");
    loop {
        let pos_dir_pair = ((guard_pos.0, guard_pos.1), (dir.0, dir.1));
        //println!("{pos_dir_pair:?}");
        if visited.contains(&pos_dir_pair) { return true; }
        visited.insert(pos_dir_pair);
        if     guard_pos.0 == 0 && dir.0 == -1
            || guard_pos.0 == map.len()-1 && dir.0 == 1
            || guard_pos.1 == 0 && dir.1 == -1
            || guard_pos.1 == map.get(0).expect("first row to exist").len()-1 {
                break;
        }
        let next_pos = (guard_pos.0 as i32 + dir.0, guard_pos.1 as i32 + dir.1);
        if let Some(row) = map.get(next_pos.0 as usize) {
            if let Some(c) = row.get(next_pos.1 as usize) {
                match c {
                    '#' => {
                        dir = match dir {
                            // UP
                            (-1, 0) => { (0, 1) },
                            // RIGHT
                            (0, 1) => { (1, 0)},
                            // DOWN
                            (1, 0) => { (0, -1) },
                            // LEFT
                            (0, -1) => { (-1, 0) },
                            _ => { panic!("Invalid direction") }
                        }
                    },
                    '.' => { guard_pos = (next_pos.0 as usize, next_pos.1 as usize) },
                    _ => { panic!("Invalid character in map") }
                }
            }
        }
    }
    false 
}

fn part_one(contents: &str) -> i32 {
    let (map, guard_pos) = parse(contents);
    let dir: (i32, i32) = (-1, 0);
    guard_routine(&map, guard_pos, dir).len() as i32
}

// Not very efficient but it works
fn part_two(contents: &str) -> i32 {
    let mut result = 0;
    let (map, guard_pos) = parse(contents);
    let dir: (i32, i32) = (-1, 0);
    let possible_positions = guard_routine(&map, guard_pos, dir);
    println!("Options: {}", possible_positions.len());
    for pos in possible_positions {
        let mut new_map = map.clone();
        if let Some(row) = new_map.get_mut(pos.0) {
            let _ = std::mem::replace(&mut row[pos.1], '#');
        }
        if guard_check_loop(&new_map, guard_pos, dir) {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn example_p1() {
        let file_path = "examples/day06.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");
        
        let p1 = part_one(&contents);
        assert_eq!(p1, 41);
    }

    #[test]
    fn example_p2() {
        let file_path = "examples/day06.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_two(&contents);
        assert_eq!(p1, 6);
    }

    #[test]
    fn input() {
        let file_path = "puzzle_inputs/day06.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_one(&contents);
        assert_eq!(p1, 5153);
        let p2 = part_two(&contents);
        assert_eq!(p2, 1711);
    }
}

