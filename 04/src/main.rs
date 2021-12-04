
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Copy, Clone)]
struct Board {
    all_numbers: [i32; 25],
    marked_positions: [bool; 25],
    winning_n: Option<i32>,
}

impl Board {
    fn new(input: Vec<i32>) -> Board {
        let mut b = Board {
            all_numbers: [0; 25],
            marked_positions: [false; 25],
            winning_n: None,
        };
        for (i, v) in input.into_iter().enumerate() {
            b.all_numbers[i] = v;
        }
        b
    }

    fn mark(self: &mut Board, num: i32) {
        for i in 0..25 {
            if self.all_numbers[i] == num {
                self.marked_positions[i] = true;
            }
        }
    }

    fn test_bingo(self: &Board) -> bool {
        for i in 0..5 {
            if self.marked_positions[5*i..5*i+5].iter().all(|x| *x) {
                return true;
            }
        }
        for i in 0..5 {
            if self.marked_positions[i..].iter().step_by(5).all(|x| *x) {
                return true;
            }
        }
        self.marked_positions[..].iter().step_by(6).all(|x| *x) ||
        self.marked_positions[4..].iter().step_by(4).take(5).all(|x| *x)
    }

    fn unmarked_sum(self: &Board) -> i32 {
        let mut sum = 0;
        for i in 0..25 {
            if !self.marked_positions[i] {
                sum += self.all_numbers[i];
            }
        }
        sum
    }
}

fn get_input() -> (Vec<i32>, Vec<Board>) {
    let file = File::open("input").expect("Input not found");
    let reader = BufReader::new(file);
    let mut it = reader.lines();

    let line1 = it.next();
    let draws = line1.unwrap().unwrap().split(",")
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = Vec::<Board>::new();
    let in_buf = it.map(|x| x.unwrap()).collect::<Vec<String>>();
    for chunk in in_buf.chunks(6) {
        boards.push(Board::new(
            chunk.join(" ")
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        ));
    }
    (draws, boards)
}

fn part1(draws: &Vec<i32>, boards_in: &Vec<Board>) {
    let mut boards: Vec<Board> = boards_in.clone();
    'outer: for n in draws {
        for board in boards.iter_mut() {
            board.mark(*n);
        }
        for board in boards.iter() {
            if board.test_bingo() {
                println!("Part1 bingo: {}", board.unmarked_sum()*(*n));
                break 'outer;
            }
        }
    }
}

fn part2(draws: &Vec<i32>, boards_in: &Vec<Board>) {
    let mut boards: Vec<Board> = boards_in.clone();
    let mut last: Board = boards[0].clone();
    for n in draws {
        for board in boards.iter_mut() {
            board.mark(*n);
        }
        for board in boards.iter_mut() {
            if board.winning_n.is_none() && board.test_bingo() {
                board.winning_n = Some(*n);
                last = board.clone();
            }
        }
    }
    println!("Part2 bingo: {}", last.unmarked_sum()*last.winning_n.unwrap());
}

fn main() {
    let (draws, boards) = get_input();
    part1(&draws, &boards);
    part2(&draws, &boards);
}

// impl fmt::Debug for Board {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in 0..5 {
//             let line_slice = &self.all_numbers[5*line..5*line+5];
//             let stringified = line_slice.iter()
//                 .map(|x| format!("{:>2}", x.to_string()));
//             write!(f, "{}\n", stringified.collect::<Vec<String>>().join(" ")).unwrap();
//         }
//         Ok(())
//     }
// }
