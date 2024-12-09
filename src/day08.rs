use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn parse(contents: &str) -> ((i64, i64), HashMap<char, Vec<(i64, i64)>>) {
    let mut tower_locations: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut height = 0;
    let mut width = 0;
    let contents = contents.split("\n");
    for (i, line) in contents.enumerate() {
        if line == "" { continue }
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() || c.is_alphabetic() {
                if let Some(freq) = tower_locations.get_mut(&c) {
                    freq.push((i as i64, j as i64));
                } else {
                    tower_locations.insert(c, vec![(i as i64, j as i64)]);
                }
            }
            width = j;
        }
        height = i;
    }
    ((height as i64, width as i64), tower_locations)
}

fn outside_bounds((loc_x, loc_y): (i64, i64), height: i64, width: i64) -> bool { 
    return loc_x < 0 || loc_y < 0 || loc_x > height || loc_y > width 
}

fn get_anodes(loc1: (i64, i64), loc2: (i64, i64), distances: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    let anode1 = (loc1.0 + distances.0, loc1.1 + distances.1);
    let anode2 = (loc2.0 - distances.0, loc2.1 - distances.1);

    (anode1, anode2)
}

fn part_one(contents: &str) -> i64 {
    let ((height, width), tower_locations) = parse(contents);
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

    for (freq, locations) in tower_locations.iter() {
        println!("{freq:}");
        for (loc1, loc2) in locations.iter().tuple_combinations() {
            let distances = ((loc1.0 - loc2.0), (loc1.1 - loc2.1));
            let (anode1, anode2) = get_anodes(*loc1, *loc2, distances);
            if !outside_bounds(anode1, height, width) { antinodes.insert(anode1); }
            if !outside_bounds(anode2, height, width) { antinodes.insert(anode2); }
        }
    }
    println!("\nAnti nodes:");
    for anode in antinodes.iter() {
        println!("{anode:?}");
    }
    antinodes.len() as i64
}

fn part_two(contents: &str) -> i64 {
    let ((height, width), tower_locations) = parse(contents);
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

    for (freq, locations) in tower_locations.iter() {
        println!("{freq:}");
        for (loc1, loc2) in locations.iter().tuple_combinations() {
            let distances = ((loc1.0 - loc2.0), (loc1.1 - loc2.1));
            let (mut anode1, mut anode2) = (*loc1, *loc2);
            loop {
                let oob1 = outside_bounds(anode1, height, width);
                let oob2 = outside_bounds(anode2, height, width);
                if !oob1{ antinodes.insert(anode1); }
                if !oob2{ antinodes.insert(anode2); }
                if oob1 && oob2 { break; }
                (anode1, anode2) = get_anodes(anode1, anode2, distances);
            }
        }
    }
    for i in 0..height+1 {
        for j in 0..width+1 {
            if antinodes.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    //println!("\nAnti nodes:");
    //for anode in antinodes.iter() {
    //    println!("{anode:?}");
    //}
    antinodes.len() as i64
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn example_p1() {
        let file_path = "examples/day08.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");

        let p1 = part_one(&contents);
        assert_eq!(p1, 14);
    }

    #[test]
    fn example_p2() {
        let file_path = "examples/day08.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");

        let p1 = part_two(&contents);
        assert_eq!(p1, 34);
    }

    #[test]
    fn input() {
        let file_path = "puzzle_inputs/day08.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");

        let p1 = part_one(&contents);
        assert_eq!(p1, 371);
        let p2 = part_two(&contents);
        assert_eq!(p2, 1229);
    }
}
