
type InputType = Vec<usize>;

fn get_input() -> InputType {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    let mut ret = Vec::new();
    let line = strin.lines().nth(0).unwrap();
    for s in line.split(",") {
        ret.push(s.parse::<usize>().unwrap());
    }
    ret
}

fn part1(input: &InputType) {
    let mut fish = input.clone();
    for _ in 0..80 {
        let mut extender = Vec::<usize>::new();
        for f in fish.iter_mut() {
            if *f > 0 {
                 *f -= 1;
            } else {
                *f = 6;
                extender.push(8);
            }
        }
        fish.extend(extender);
    }
    println!("Number of lanternfish pt1: {}", fish.len());
}

fn part2(input: &InputType) {
    let mut ages: [usize; 9] = [0; 9];
    for a in input {
        ages[*a] += 1;
    }
    for _ in 0..256 {
        let mut new_ages: [usize; 9] = [0; 9];
        for age in 0..9 {
            if age == 0 {
                new_ages[6] += ages[0];
                new_ages[8] += ages[0];
            } else {
                new_ages[age - 1] += ages[age];
            }
        }
        ages = new_ages.clone();
        // println!("{:?}", ages);
    }
    println!("Number of lanternfish pt2: {}", ages.iter().sum::<usize>());
}

fn main() {
    let fish = get_input();
    part1(&fish);
    part2(&fish);
}
