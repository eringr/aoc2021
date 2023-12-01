
#[derive(Clone)]
struct FloorMap {
    grid: Vec<Vec<u8>>,
}

impl FloorMap {
    fn new() -> FloorMap {
        FloorMap {
            grid: vec![],
        }
    }
    fn add_line(self: &mut FloorMap, l: &Vec<u8>) {
        self.grid.push(l.clone());
    }
    fn find_mins(self: &FloorMap) -> usize {
        let mut danger = 0;
        fn find_on_edge(edge: &Vec<u8>, neighbor: &Vec<u8>) -> usize {
            let mut ret:usize = 0;
            for i in 1..edge.len()-1 {
                // let i = tester as u8;
                if edge[i-1] > edge[i] &&
                    edge[i+1] > edge[i] &&
                    neighbor[i] > edge[i]
                {
                    ret += edge[i] as usize + 1;
                }
            }
            ret
        }
        let get_column = |c: usize| -> Vec<u8> {
            self.grid.iter().fold(Vec::<u8>::new(), |mut a, i| {a.push(i[c]); a})
        };
        for (a, b, c) in [
            (self.grid[0][0], self.grid[0][1], self.grid[1][0]),
            (self.grid[0][99], self.grid[0][98], self.grid[1][99]),
            (self.grid[99][0], self.grid[99][1], self.grid[98][0]),
            (self.grid[99][99], self.grid[99][98], self.grid[98][99]),
        ] {
            if a < b && a < c {
                danger += a as usize + 1
            }
        }
        for (a, b) in [
            (&self.grid[0], &self.grid[1]),
            (&get_column(0), &get_column(1)),
            (&get_column(99), &get_column(98)),
            (&self.grid[99], &self.grid[98]),
        ] {
            danger += find_on_edge(a, b);
        }
        for l in 1..self.grid.len()-1 {
            // let i = tester as u8;
            let (a, b, c) = (&self.grid[l], &self.grid[l-1], &self.grid[l+1]);
            for i in 1..99 {
                if a[i] < a[i-1] &&
                    a[i] < a[i+1] &&
                    a[i] < b[i] &&
                    a[i] < c[i]
                {
                    danger += a[i] as usize + 1;
                }
            }
        }
        danger
    }
}

impl std::fmt::Debug for FloorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid[..] {
            write!(f, "{:?}\n", line).unwrap();
        }
        Ok(())
    }
}

type InputType = FloorMap;

fn get_input() -> InputType {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    let mut ret = FloorMap::new();
    for line in strin.lines() {
        let mut adder = Vec::new();
        for c in line.chars() {
            adder.push(c as u8 - 0x30);
        }
        ret.add_line(&mut adder);
    }
    // println!("{:?}", ret);
    ret
}

fn part1(input: &InputType) {
    println!("Danger pt1: {}", input.find_mins());
}

fn part2(input: &InputType) {
    println!("Unimplemented pt2: {}", 0);
}

fn main() {
    let distances = get_input();
    part1(&distances);
    part2(&distances);
}
