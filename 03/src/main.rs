
fn get_input() -> Vec<String> {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    strin.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn part1(input: &Vec<String>) {
    let width = input[0].len();
    let mut gammas = vec![0; width];
    for line in input {
        for (i, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'0' => gammas[i] -= 1,
                b'1' => gammas[i] += 1,
                _ => (),
            }
        }
    }

    let gamma_str = gammas.into_iter().fold(
        String::new(),
        |mut a: String, x| {
            match x {
                x if x > 0 => a.push('1'),
                _ => a.push('0'),
            }
            a
        }
    );

    let gamma = isize::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon = gamma ^ 0b111111111111;

    println!("gamma: {}, epsilon: {}, mul: {}", gamma, epsilon, gamma*epsilon);
}

fn part2(input: &Vec<String>) {
    let width = input[0].len();

    fn find_gamma(input: &Vec<String>, i: usize) -> i32 {
        let mut gamma = 0;
        for line in input.iter() {
            match line.chars().nth(i) {
                Some('0') => gamma -= 1,
                Some('1') => gamma += 1,
                _ => (),
            }
        }
        gamma
    }

    let mut sieve_ox = input.clone();
    let mut sieve_co2 = input.clone();
    for i in 0..width {
        let gamma_ox = find_gamma(&sieve_ox, i);
        let gamma_co2 = find_gamma(&sieve_co2, i);
        let f = |x: &String, gamma: i32| {
            match x.chars().nth(i) {
                Some('0') => if gamma >= 0 {false} else {true},
                _ => if gamma >= 0 {true} else {false},
            }
        };
        if sieve_ox.len() > 1 {
            sieve_ox = sieve_ox.into_iter().filter(|x| f(x, gamma_ox)).collect();
        }
        if sieve_co2.len() > 1 {
            sieve_co2 = sieve_co2.into_iter().filter(|x| !f(x, gamma_co2)).collect();
        }
    }

    let ox = isize::from_str_radix(sieve_ox[0].as_str(), 2).unwrap();
    let co2 = isize::from_str_radix(sieve_co2[0].as_str(), 2).unwrap();

    println!("ox: {}, co2: {}, mul: {}", ox, co2, ox*co2);
}

fn main() {
    let input = get_input();
    part1(&input);
    part2(&input);
}
