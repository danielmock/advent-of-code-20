use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use std::cmp;
fn main() {
    let input = include_str!("../input");

    let input = input.lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let result1 = input.windows(26).find(|x| !part1(x)).unwrap()[25];
    println!("{}", result1);

    let result2 = part2(&input, result1);
    println!("{}", result2);
}

fn part1(v: &[usize]) -> bool {
    let x = v[25];
    let v = &v[0..25];
    v.iter().tuple_combinations().any(|(a,b)| a + b == x && a != b)
}

fn part2(input: &[usize], key: usize) -> usize {
    (0..input.len())
    .map(|i| {
        input[i..].iter()
        .fold_while( (0, usize::MAX, 0), |(sum, min, max), &elem| {
            let sum = sum + elem;
            let min = cmp::min(min, elem);
            let max = cmp::max(max, elem);
            if sum >= key {
                Done((sum, min, max))
            } else {
                Continue((sum, min, max))
            }
        }).into_inner()
    }).find_map(|(sum, min, max)| {
        if sum == key {
            Some(min + max)
        } else {
            None
        }
    }).unwrap()
}
