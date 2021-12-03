
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_input() -> Vec<i32> {
    let file = File::open("input").expect("Input not found");
    let reader = BufReader::new(file);
    let mut ret = Vec::new();
    for line in reader.lines() {
        ret.push(line.unwrap().parse::<i32>().unwrap());
    }
    ret
}

fn part1(input: &Vec<i32>) {
    let mut it = input.iter();
    let mut previous = it.next().unwrap();
    let mut total = 0;
    for i in it {
        if i > previous {
            total += 1;
        }
        previous = i;
    }
    println!("part1 number of increases: {}", total);
}

fn part2(input: &Vec<i32>) {
    let mut it = input.iter();
    let mut previous = *it.next().unwrap();
    let mut increases = 0;
    loop {
        let mut window_iter = it.clone();
        let mut window_total = 0;
        for i in window_iter.take(3) {
            window_total += i;
        }
        if window_total > previous {
            increases += 1;
        }
        previous = window_total;
        if let None = it.next() {
            break;
        }
    }
    println!("part2 number of increases: {}", increases);
}

fn main() {
    let input = get_input();
    part1(&input);
    part2(&input);
}
