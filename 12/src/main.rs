
use std::collections::HashMap;
use itertools::Itertools;

type InputType = (usize, usize, Vec<Cave>);

#[derive(Clone, Debug)]
struct Cave {
    connected: Vec<usize>,
    reentrant: bool,
}

fn get_input() -> InputType {
    let strin = std::fs::read_to_string("input").expect("Input not found");
    let cave_map: HashMap<&str, usize> = strin.lines()
        .flat_map(|x| x.split("-"))
        .unique()
        .zip(0..9999)
        .collect();
    println!("{:?}", cave_map);
    let capitals: Vec<usize> = cave_map.iter()
        .filter_map(|(x, y)|
            if x.chars().next().unwrap().is_ascii_uppercase() {Some(*y)}
            else {None}
        ).collect();
    println!("{:?}", capitals);
    let mut ret: Vec<Cave> = (0..cave_map.len())
        .map(|x| Cave {connected: Vec::new(), reentrant: capitals.contains(&x)})
        .collect();
    for (l, r) in strin.lines()
        .flat_map(|x| x.split("-"))
        .tuples::<(_,_)>()
    {
        ret[cave_map[l]].connected.push(cave_map[r]);
        ret[cave_map[r]].connected.push(cave_map[l]);
    }
    (cave_map["start"], cave_map["end"], ret)
}

fn find_all_paths(
    caves_in: &InputType,
    i: usize,
    mut visited: Vec<bool>,
    can_visit_twice: bool,
) -> usize {
    let (start, end, caves) = caves_in;
    if i == *end {
        return 1;
    }
    visited[i] = !caves[i].reentrant;
    caves[i].connected
        .iter()
        .filter(|&&n| n != *start && (can_visit_twice || !visited[n]))
        .map(|&next_position| {
            find_all_paths(
                caves_in,
                next_position,
                visited.clone(),
                can_visit_twice && !visited[next_position],
            )
        })
        .sum()
}

fn part1(input: &InputType) {
    println!("{:?}", find_all_paths(input, input.0, vec![false;input.2.len()], false));
}

fn part2(input: &InputType) {
    println!("{:?}", find_all_paths(input, input.0, vec![false;input.2.len()], true));
}

fn main() {
    let inputs = get_input();
    println!("{:?}", inputs);
    part1(&inputs);
    part2(&inputs);
}
