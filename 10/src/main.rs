
type InputType = Vec<String>;

fn get_input() -> InputType {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    let mut ret = Vec::new();
    for line in strin.lines() {
        ret.push(line.to_string())
    }
    // println!("{:?}", ret);
    ret
}

enum ScoreOrStack {
    Score(usize),
    Stack(Vec<char>),
}

fn process_line(line: &String) -> ScoreOrStack {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ']'|')'|'}'|'>' => {
                if stack.pop().unwrap() == c {
                    continue;
                }
                return match c {
                    ')' => ScoreOrStack::Score(3),
                    ']' => ScoreOrStack::Score(57),
                    '}' => ScoreOrStack::Score(1197),
                    '>' => ScoreOrStack::Score(25137),
                    _ => panic!(""),
                };
            },
            _ => panic!("{}", c),
        }
    }
    ScoreOrStack::Stack(stack)
}

fn part1(input: &InputType) {
    println!("Score pt1: {}", input.iter().filter_map(
        |l| if let ScoreOrStack::Score(x) = process_line(l) {Some(x)} else {None}
    ).sum::<usize>());
}

fn part2(input: &InputType) {
    let mut scores = Vec::new();
    for line in input {
        if let ScoreOrStack::Stack(stack) = process_line(line) {
            scores.push(stack.iter().rev().fold(0usize, |a, i| 5*a + match i {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!(""),
            }));
        }
    }
    scores.sort();
    println!("Score pt2: {}", scores[scores.len()/2]);
}

fn main() {
    let inputs = get_input();
    part1(&inputs);
    part2(&inputs);
}
