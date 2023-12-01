
type InputType = Vec<i64>;

fn get_input() -> InputType {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    let mut ret = Vec::new();
    let line = strin.lines().nth(0).unwrap();
    for s in line.split(",") {
        ret.push(s.parse::<i64>().unwrap());
    }
    ret
}

fn calc_fuel(input: &InputType, f: &dyn Fn(i64, i64)->i64) -> i64 {
    let mut fuels_used = Vec::new();
    for t in *input.iter().min().unwrap()..*input.iter().max().unwrap() {
        fuels_used.push(input.iter().map(|c| f(*c, t)).sum::<i64>());
    }
    *fuels_used.iter().min().unwrap()
}

fn part1(input: &InputType) {
    let add_fn = |l:i64, r:i64| (l-r).abs();
    println!("Fuel used pt1: {}", calc_fuel(input, &add_fn));
}

fn part2(input: &InputType) {
    let add_fn = |l:i64, r:i64|(1..=(l-r).abs()).into_iter().sum::<i64>();
    println!("Fuel used pt2: {}", calc_fuel(input, &add_fn));
}

fn main() {
    let distances = get_input();
    part1(&distances);
    part2(&distances);
}
