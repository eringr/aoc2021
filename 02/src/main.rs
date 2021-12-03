
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_input() -> Vec<(String, i32)> {
    let file = File::open("input").expect("Input not found");
    let reader = BufReader::new(file);
    let mut ret = Vec::new();
    for line in reader.lines() {
        let line_unwrapped = line.unwrap();
        let mut fields = line_unwrapped.split_whitespace();
        ret.push((
            fields.next().unwrap().to_string(),
            fields.next().unwrap().parse::<i32>().unwrap())
        );
    }
    ret
}

fn part1(input: &Vec<(String, i32)>) {
    let it = input.iter();
    let mut depth = 0;
    let mut pos = 0;
    for (dir, amt) in it {
        match dir.as_str() {
            "forward" => pos += amt,
            "up" => depth += amt,
            "down" => depth -= amt,
            _ => (),
        }
    }
    println!("pos: {}, depth: {}, mul: {}", pos, depth, pos*depth);
}

fn part2(input: &Vec<(String, i32)>) {
    let it = input.iter();
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for (dir, amt) in it {
        match dir.as_str() {
            "forward" => {
                pos += amt;
                depth += aim*amt;
            },
            "up" => aim -= amt,
            "down" => aim += amt,
            _ => (),
        }
    }
    println!("pos: {}, depth: {}, mul: {}", pos, depth, pos*depth);
}

fn main() {
    let input = get_input();
    part1(&input);
    part2(&input);
}
