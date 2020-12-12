fn main() {
    let input = include_str!("../input");

    // let height = input.lines().count();
    // let width = input.lines().first().unwrap().chars().count();

    let system = input.lines().map(|line|
        line.chars().map(|seat| {
            parse_seat(seat)
        }).collect::<Vec<Seat>>()
    ).collect::<Vec<_>>();


}

pub enum Seat {
    Floor, 
    Free,
    Occupied
}

fn part1() {
    loop {

        if old == new {
            break;
        }
    }
}

fn parse_seat(c: char) -> Seat {
    match c {
        'L' => Seat::Free,
        '.' => Seat::Floor,
        '#' => Seat::Occupied
    }
}

fn neighbors(system: &[&[Seat]], i: usize, j: usize) -> Vec<Option<Seat>> {
    let vec = Vec::new();
    for (di, dj) in [(-1,-1), (-1,0), (-1,1),  (0,-1), (0,1), (1, -1), (1, 0), (1, -1)].iter() {
        vec.push(
            system.get(i + di).map(|v| v.get(j + dj)));
    }
    vec
}

pub struct Automaton {
    height: usize,
    width: usize,
    field: Vec<Vec<Seat>>
}

impl Automaton {
    fn neighbors(&self, i: usize, j: usize) -> [Option<usize>; 8] {
        let mut result = [None; 8];

        if i == 0

        result
    }
}