struct Equation {
    num1: i64,
    num2: EqItem,
}

enum EqItem {
    EQUATION(Box<Equation>),
    NUM(i64),
}

impl Equation {
    fn equals(self: &Self, n: i64, part2: bool) -> bool {
        for num in self.evaluate(part2) {
            if num == n { 
                //println!("{num}");
                return true;
            }
        }
        false
    }

    fn evaluate(self: &Self, part2: bool) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::new();
        match &self.num2 {
            EqItem::EQUATION(eq) => {
                for num in eq.evaluate(part2) {
                    result.push(self.num1 + num);
                    result.push(self.num1 * num);
                    if part2 { 
                        let n = num * 10i64.pow(self.num1.ilog10() + 1) + self.num1;
                        //println!("{num} || {} = {n}", self.num1);
                        result.push(n);
                    }
                }
                result
            },
            EqItem::NUM(num) => {
                result.push(self.num1 + num);
                result.push(self.num1 * num);
                if part2 { 
                    let n = num * 10i64.pow(self.num1.ilog10() + 1) + self.num1;
                    //println!("{num} || {} = {n}", self.num1);
                    result.push(n);
                }
                result
            },
        }
    }
}


fn parse(contents: &str) -> Vec<(i64, Equation)> {
    let mut eqs = Vec::new();
    for line in contents.split("\n") {
        if line == "" { continue }
        let first_split: Vec<_> = line.split(":").map(|s| s.trim()).collect();
        let result = first_split
            .get(0)
            .expect("result to exist")
            .parse::<i64>()
            .expect("it to be a number");
        let formulas: Vec<_> = first_split
            .get(1)
            .expect("Some numbers to exist")
            .split(" ")
            .map(|n| n.parse::<i64>().expect("it to be a number"))
            .collect();
        let eq = create_equations(formulas);
        eqs.push((result, eq))
    }
    eqs
}

fn create_equations(mut formulas: Vec<i64>) -> Equation {
    if let Some(num) = formulas.pop() {
        if formulas.len() == 1 {
            Equation {
                num1: num,
                num2: EqItem::NUM(formulas.pop().expect("We just checked for 1 item left"))
            }
        } else {
            Equation {
                num1: num,
                num2: EqItem::EQUATION(Box::new(create_equations(formulas)))
            }
        }
    } else {
        panic!("NO NUMBER???");
    }
}

fn part_one(contents: &str) -> i64 {
    let mut result = 0;
    let equations = parse(contents);
    for (res, eq) in equations.iter() {
        if eq.equals(*res, false) {
            result += res;   
        }
    }
    result
}

fn part_two(contents: &str) -> i64 { 
    let mut result = 0;
    let equations = parse(contents);
    for (res, eq) in equations.iter() {
        if eq.equals(*res, true) {
            result += res;   
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
        let file_path = "examples/day07.txt";
        let contents = fs::read_to_string(file_path)
            .expect("To read the file");
        
        let p1 = part_one(&contents);
        assert_eq!(p1, 3749);
    }

    #[test]
    fn example_p2() {
        let file_path = "examples/day07.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_two(&contents);
        assert_eq!(p1, 11387);
    }

    #[test]
    fn input() {
        let file_path = "puzzle_inputs/day07.txt";
        let contents = fs::read_to_string(file_path)
                .expect("To read the file");

        let p1 = part_one(&contents);
        assert_eq!(p1, 850435817339);
        let p2 = part_two(&contents);
        assert_eq!(p2, 104824810233437);
    }
}

