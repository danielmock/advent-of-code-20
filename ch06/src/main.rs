use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    let groups = parse_first(input);
    groups.iter().map(|group| group.len()).sum()
}

lazy_static! {
    static ref BLANK_LINE_REGEX: Regex = Regex::new(r"\n\s*\n").unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"[\s\n]*").unwrap();
}

fn parse_first(input: &str) -> Vec<HashSet<char>> {
    BLANK_LINE_REGEX
        .split(input)
        .map(|line| WHITESPACE_REGEX.replace_all(line, ""))
        .filter(|line| !line.is_empty())
        .map(|group| group.chars().collect::<HashSet<_>>())
        .collect()
}

fn solve_second(input: &str) -> usize {
    let groups = parse_second(input);
    groups.iter().map(|group| group.len()).sum()
}

fn parse_second(input: &str) -> Vec<HashSet<char>> {
    BLANK_LINE_REGEX
        .split(input)
        .map(|group_lines| {
            let mut group = group_lines
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().collect::<HashSet<_>>());
            let first = group.next().unwrap();
            group.fold(first, |acc, next| {
                acc.intersection(&next).cloned().collect()
            })
        })
        .collect()
}


// #![feature(iterator_fold_self)]
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    // let result1 = first_solution(input);
    // println!("{}", result1);

    // let result2 = second_solution(input);
    // println!("{}", result2);   
}

fn first_solution(input: &str) -> usize {
    input.split("\n\n")
        .map(|s| s.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<HashSet<_>>()
        )
        .map(|h| h.len())
        .sum()
}

// fn first_solution(input: &str) -> u32 {
//     input.split("\n\n")
//         .map(|s| s.split("\n")
//             .map(|line| line.trim().as_bytes()
//                 .iter()
//                 .map(|c| (c - b'a') as usize)
//                 .map(|i| 1 << i) 
//                 .fold(0u32, |a,b| a | b))
//             .fold_first(|a,b| a | b)
//             .unwrap()
//             .count_ones()
//         )
//         .sum()
// }


fn second_solution(input: &str) -> u32 {
    input.split("\n\n")
        .map(|s| s.split("\n")
            .map(|line| line.trim().as_bytes()
                .iter()
                .map(|c| (c - b'a') as usize)
                .map(|i| 1 << i) 
                .fold(0u32, |a,b| a | b))
            .fold_first(|a,b| a & b)
            .unwrap()
            .count_ones()
        )
        .sum()
}
