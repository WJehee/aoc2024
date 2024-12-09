fn part_one(contents: &str) -> i64 {
    let mut result = 0;
   
    0
}

//fn part_two(contents: &str) -> i64 {}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn example_p1() {
        let file_path = "examples/day00.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");
        
        let p1 = part_one(&contents);
        assert_eq!(p1, -1);
    }

    //#[test]
    //fn example_p2() {
    //    let file_path = "examples/day00.txt";
    //    let contents = fs::read_to_string(file_path)
    //            .expect("To read the file");
    //
    //    let p1 = part_two(&contents);
    //    assert_eq!(p1, -1);
    //}

    #[test]
    fn input() {
        let file_path = "puzzle_inputs/day00.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_one(&contents);
        assert_eq!(p1, -1);
        //let p2 = part_two(&contents);
        //assert_eq!(p2, -1);
    }
}
