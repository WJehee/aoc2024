use std::fs;

fn part_one(contents: &str) -> i32 {
    let mut grid = Vec::new();
    let mut x_locations = Vec::new();
    for (i, line) in contents.split("\n").enumerate() {
        let mut row = Vec::new();
        for (j, letter) in line.chars().enumerate() {
            match letter {
                'X' => x_locations.push((i as i32, j as i32)),
                _ => {},
            }
            row.push(letter);
        }
        if row.len() > 0 {
            grid.push(row);
        }
    }
    let mut result = 0;
    for x_loc in x_locations {
        println!("{:?}", x_loc);
        for m_loc in find_surrounding(&grid, (x_loc.0, x_loc.1), 'M') {
            let x_dir = x_loc.0 - m_loc.0;
            let y_dir = x_loc.1 - m_loc.1;
            println!("X: {:?}", (x_loc.0+1, x_loc.1+1));
            if find_in_direction(&grid, x_loc, x_dir, y_dir, "MAS") {
                result += 1;
            }
        }
    }
    result
}

fn part_two(contents: &str) -> i32 {
    let mut grid = Vec::new();
    let mut a_locations = Vec::new();
    for (i, line) in contents.split("\n").enumerate() {
        let mut row = Vec::new();
        for (j, letter) in line.chars().enumerate() {
            match letter {
                'A' => a_locations.push((i as i32, j as i32)),
                _ => {},
            }
            row.push(letter);
        }
        if row.len() > 0 {
            grid.push(row);
        }
    }
    let mut result = 0;
    for a_loc in a_locations {
        if check_for_x(&grid, a_loc) { result += 1 }
    }
    result
}

fn check_for_x(grid: &Vec<Vec<char>>, loc: (i32, i32)) -> bool {
    let mut opposite_counts = 0;
    let surrounding_ms = find_surrounding(&grid, loc, 'M');
    let surrounding_ss = find_surrounding(&grid, loc, 'S');
    if surrounding_ms.len() >= 2 && surrounding_ss.len() >= 2 {
        for m_loc in &surrounding_ms {
            for s_loc in &surrounding_ss {
                let x_diff = (m_loc.0 - s_loc.0).abs();
                let y_diff = (m_loc.1 - s_loc.1).abs();
                if x_diff == 2 && y_diff == 2 { opposite_counts += 1 }
                if opposite_counts == 2 { return true }
            }
        }
    }
    false
}

fn find_in_direction(grid: &Vec<Vec<char>>, loc: (i32, i32), x_dir: i32, y_dir: i32, chars_to_find: &str) -> bool {
    if let Some(c) = chars_to_find.chars().nth(0) {
        let new_loc = (loc.0-x_dir, loc.1-y_dir);
        if let Some(row) = grid.get(new_loc.0 as usize) {
            if let Some(letter) = row.get(new_loc.1 as usize) {
                if *letter == c { 
                    println!("{c}: {:?}", (new_loc.0+1, new_loc.1+1));
                    return find_in_direction(grid, new_loc, x_dir, y_dir, &chars_to_find[1..])
                } else {
                    println!("invalid char: {letter}, expected: {c}\n");
                }
            }
        }
    } else {
        println!("XMAS!\n");
        return true
    }
    false 
}

// I implemented the NYT games word search...
//fn find_xmas(grid: &Vec<Vec<char>>, x_loc: (i32, i32)) -> bool {
//    for m_loc in find_surrounding(&grid, (x_loc.0, x_loc.1), 'M') {
//        for a_loc in find_surrounding(&grid, (m_loc.0, m_loc.1), 'A') {
//            for _s_loc in find_surrounding(&grid, (a_loc.0, a_loc.1), 'S') {
//                return true;
//            }
//        }
//    }    
//    false
//}

fn find_surrounding(grid: &Vec<Vec<char>>, location: (i32, i32), char_to_find: char) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let row_lower = if location.0 == 0 { 0 } else {location.0-1};
    let col_lower = if location.1 == 0 { 0 } else {location.1-1};

    for i in row_lower..(location.0+2) {
        for j in col_lower..(location.1+2) {
            if i != location.0 || j != location.1 {
                if let Some(row) = grid.get(i as usize) {
                    if let Some(letter) = row.get(j as usize) {
                        if *letter == char_to_find {
                            result.push((i, j));
                        }
                    }
                }

            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_p1() {
        let file_path = "examples/day04.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");

        let p1 = part_one(&contents);
        assert_eq!(p1, 18);
    }

    #[test]
    fn example_p2() {
        let file_path = "examples/day04.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_two(&contents);
        assert_eq!(p1, 9);
    }

    #[test]
    fn input() {
        let file_path = "puzzle_inputs/day04.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_one(&contents);
        assert_eq!(p1, 2591);
        let p2 = part_two(&contents);
        assert_eq!(p2, 1880);
    }
}

