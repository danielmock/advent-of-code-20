use std::ops::Add;
use std::ops::AddAssign;
use Cardinal::*;
use Instruction::*;

struct State {
    dir: Cardinal,
    east: isize,
    north: isize
}

enum Instruction {
    Rel(Relative, usize),
    Card(Cardinal, usize),
    Forward(usize)
}

#[derive(Copy, Clone)]
enum Cardinal {
    North = 0,
    East = 1,
    South = 2,
    West = 3
}

#[derive(Copy, Clone)]
enum Relative {
    Left = 3,
    Right = 1
}

fn main() {
    let input = include_str!("../input");

    let input = input.lines().map(|line| {
        let dir = line.chars().nth(0).unwrap();
        let num = line[1..].parse::<usize>().unwrap();

        match dir {
            'N' => Card(North, num),
            'E' => Card(East, num),
            'S' => Card(South, num),
            'W' => Card(West, num),
            'L' => Rel(Relative::Left, num),
            'R' => Rel(Relative::Right, num),
            'F' => Forward(num),
            _ => unreachable!(),
        }
    }).collect::<Vec<Instruction>>();

    print!("{}", part1(input));
}

fn part1(input: Vec<Instruction>) -> isize {
    let mut state = State::new();

    for instr in input.iter() {
        match instr {
            Rel(r, u) => for _ in 0..(u/90) {state.turn(*r)},
            Card(c, u) => {let temp = state.dir; state.dir = *c; state.forward(*u); state.dir = temp;},
            Forward(u) => state.forward(*u),
        }
    }

    state.east.abs() + state.north.abs()
}


impl Cardinal {
    fn from(n: i8) -> Self {
        let n = n % 4;
        match n {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => unreachable!()
        }
    }
}

impl Add<Relative> for Cardinal {
    type Output = Self;

    fn add(self, other: Relative) -> Self {
        Cardinal::from(self as i8 + other as i8)
    }
}
impl AddAssign<Relative> for Cardinal {
    fn add_assign(&mut self, other: Relative) {
        *self = self.add(other);
    }
}

impl State {
    fn new() -> State {
        State {
            dir: East,
            east: 0,
            north: 0
        }
    }

    fn forward(&mut self, n: usize) {
        let n = n as isize;
        match self.dir {
            North => self.north += n,
            East => self.east += n,
            South => self.north -= n,
            West => self.east -= n
        }
    }

    fn turn(&mut self, r: Relative) {
        self.dir += r;
    }
}
