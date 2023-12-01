
type InputType = OctopusMap;

fn get_input() -> InputType {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    let mut ret = OctopusMap::new();
    for line in strin.lines() {
        let mut adder = Vec::new();
        for c in line.chars() {
            adder.push(c as usize - 0x30);
        }
        ret.add_line(&mut adder);
    }
    ret
}

#[derive(Clone)]
struct OctopusMap {
    grid: Vec<Vec<usize>>,
}

impl std::fmt::Debug for OctopusMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid[..] {
            write!(f, "{:?}\n", line).unwrap();
        }
        Ok(())
    }
}

impl OctopusMap {
    fn new() -> OctopusMap {
        OctopusMap {
            grid: vec![],
        }
    }
    fn add_line(self: &mut OctopusMap, l: &Vec<usize>) {
        self.grid.push(l.clone());
    }
    fn increment_cell(self: &mut OctopusMap, x: usize, y: usize) {
        self.grid[y][x] += 1;
        if self.grid[y][x] == 10 {
            self.increment_adjacents(x, y);
        }
    }
    fn increment_adjacents(self: &mut OctopusMap, x: usize, y: usize) {
        if x > 0 {
            self.increment_cell(x-1, y);
            if y > 0 {
                self.increment_cell(x-1, y-1);
            }
            if y < 9 {
                self.increment_cell(x-1, y+1);
            }
        }
        if y > 0 {
            self.increment_cell(x, y-1);
        }
        if y < 9 {
            self.increment_cell(x, y+1);
        }
        if x < 9 {
            self.increment_cell(x+1, y);
            if y > 0 {
                self.increment_cell(x+1, y-1);
            }
            if y < 9 {
                self.increment_cell(x+1, y+1);
            }
        }
    }
    fn step(self: &mut OctopusMap) -> usize {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                self.grid[y][x] += 1;
                if self.grid[y][x] == 10 {
                    self.increment_adjacents(x, y);
                }
            }
        }
        let mut ret = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.grid[y][x] > 9 {
                    self.grid[y][x] = 0;
                    ret += 1;
                }
            }
        }
        ret
    }
}

fn part1(input: &InputType) {
    let mut map = input.clone();
    let mut acc = 0;
    for _ in 0..100 {
        acc += map.step();
    }
    println!("pt1 acc: {}", acc);
}

fn part2(input: &InputType) {
    let mut map = input.clone();
    for i in 1..10000000 {
        // println!("{:?}", map);
        if map.step() == 100 {
            println!("pt2 step: {}", i);
            break;
        }
        // std::thread::sleep(std::time::Duration::from_millis(150));
        // print!("{}[2J", 27 as char);
    }
}

fn main() {
    let inputs = get_input();
    part1(&inputs);
    part2(&inputs);
}
