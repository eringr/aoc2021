
fn get_input() -> Vec<(String, i32)> {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    let mut ret = Vec::new();
    for line in strin.lines() {
        let mut fields = line.split_whitespace();
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
