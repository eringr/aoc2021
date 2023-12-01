
use std::collections::HashSet;

type InputType = Vec<(Vec<String>, Vec<String>)>;

fn get_input() -> InputType {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    let mut ret = Vec::new();
    for line in strin.lines() {
        let (mut a, mut b) = (Vec::<String>::new(), Vec::<String>::new());
        let mut splitter = line.split("|");
        a.extend(splitter.next().unwrap().split_whitespace().map(|x| x.to_string()));
        b.extend(splitter.next().unwrap().split_whitespace().map(|x| x.to_string()));
        ret.push((a, b));
    }
    ret
}

fn part1(input: &InputType) {
    let mut acc = 0;
    for (_, output) in input {
        acc += output.iter().filter(|x| [2, 3, 4, 7].contains(&x.len())).count();
    }
    println!("acc pt1: {}", acc);
}

fn part2(input: &InputType) {
    let mut output = 0;
    for line in input {
        let mut inputs = Vec::<HashSet<char>>::new();
        inputs.extend(line.0.iter().map(|x| x.chars().collect::<HashSet<char>>()));
        let d1 = inputs.iter().filter(|x| x.len() == 2).next().unwrap();
        let d4 = inputs.iter().filter(|x| x.len() == 4).next().unwrap();
        let d7 = inputs.iter().filter(|x| x.len() == 3).next().unwrap();
        let d8 = inputs.iter().filter(|x| x.len() == 7).next().unwrap();
        let d2 = inputs.iter().filter(|x| x.len() == 5 && (*x & &d4).len() == 2).next().unwrap();
        let d3 = inputs.iter().filter(|x| x.len() == 5 && (*x & &d1).len() == 2).next().unwrap();
        let d5 = inputs.iter().filter(|x| x.len() == 5 && *x != d2 && *x != d3).next().unwrap();
        let d9 = inputs.iter().filter(|x| x.len() == 6 && (*x & &d4).len() == 4).next().unwrap();
        let d6 = inputs.iter().filter(|x| x.len() == 6 && (*x & &d1).len() == 1).next().unwrap();
        let d0 = inputs.iter().filter(|x| x.len() == 6 && *x != d6 && *x != d9).next().unwrap();
        let mut code = String::new();
        for o in line.1.iter().map(|x| x.chars().collect::<HashSet<char>>()) {
            for (i, d) in [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9].iter().enumerate() {
                if o == **d {
                    code.push(char::from_digit(i as u32, 10).unwrap());
                }
            }
        }
        output += code.parse::<usize>().unwrap();
    }
    println!("acc pt2 {}", output);
}

fn main() {
    let distances = get_input();
    part1(&distances);
    part2(&distances);
}
