use itertools::Itertools;

fn main() {
    let parameters: [(usize, usize); 5] = [(1,1), (1,3), (1,5), (1,7), (2,1)];
    // let parameters: [(usize, usize); 1] = [(1,3)];
    let input = include_str!("../input");
    println!("{}",parameters.iter().map(|x| compute(*x, input)).product::<usize>());
}

fn compute((down, right): (usize, usize), lines: &str) -> usize {
    let mut pos = 0;
    let mut trees = 0;
    for line in lines.lines().step_by(down) {
        match line.chars().nth(pos) {
            Some('#') => trees = trees +1,
            Some('.') => (),
            _ => println!("ohoh"),
        } 
        pos = (pos + right) % line.chars().count();
    }
    trees
}


