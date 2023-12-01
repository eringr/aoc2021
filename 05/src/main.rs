
use regex::Regex;

#[derive(Clone)]
struct VentMap {
    grid: Vec<Vec<i64>>,
}

impl VentMap {
    fn new() -> VentMap {
        VentMap {
            grid: vec![vec![0; 1000]; 1000],
        }
    }
    fn count_crossings(self: &VentMap) -> usize {
        let mut total = 0;
        for line in &self.grid[..] {
            for pos in &line[..] {
                if *pos > 1 {
                    total += 1;
                }
            }
        }
        total
    }
}

fn get_input() -> Vec<(usize, usize, usize, usize)> {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    let re = Regex::new(r"([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").unwrap();
    let mut ret = Vec::new();
    for line in strin.lines() {
        let caps = re.captures(line).unwrap();
        let parser = |x: Option<regex::Match>|
            x.unwrap().as_str().parse::<usize>().unwrap();
        ret.push((
            parser(caps.get(1)),
            parser(caps.get(2)),
            parser(caps.get(3)),
            parser(caps.get(4)),
        ));
    }
    ret
}

fn mark_orthogonal(
    map: &mut VentMap,
    (x1, y1, x2, y2): (usize, usize, usize, usize)
) {
    if x1 == x2 {
        let (y_begin, y_end) = if y1 > y2 {(y2, y1)} else {(y1, y2)};
        for i in y_begin..=y_end {
            map.grid[i][x1] += 1;
        }
    } else if y1 == y2 {
        let (x_begin, x_end) = if x1 > x2 {(x2, x1)} else {(x1, x2)};
        for i in x_begin..=x_end {
            map.grid[y1][i] += 1;
        }
    }
}

fn part1(input: &Vec<(usize, usize, usize, usize)>) {
    let mut m = VentMap::new();
    for (x1, y1, x2, y2) in input.into_iter() {
        mark_orthogonal(&mut m, (*x1, *y1, *x2, *y2));
    }
    println!("Part1 crossings: {}", m.count_crossings());
}

fn part2(input: &Vec<(usize, usize, usize, usize)>) {
    let mut m = VentMap::new();
    for (x1, y1, x2, y2) in input.into_iter() {
        mark_orthogonal(&mut m, (*x1, *y1, *x2, *y2));
        if *x1 != *x2 && *y1 != *y2 {
            let (y_begin, y_end) = if *y1 > *y2 {(*y2, *y1)} else {(*y1, *y2)};
            let x_begin = if *y1 > *y2 {*x2} else {*x1};
            let x_step = if (*y1 > *y2) ^ (*x1 > *x2) {-1i64} else {1i64};
            let mut x = x_begin as i64;
            for y in y_begin..=y_end {
                m.grid[y][x as usize] += 1;
                x += x_step;
            }
        }
    }
    println!("Part2 crossings: {}", m.count_crossings());
}

fn main() {
    let coords = get_input();
    part1(&coords);
    part2(&coords);
}
