#[macro_use]
extern crate lalrpop_util;

use std::fs::File;
use std::io::{BufRead, BufReader};
use core::cmp::min;

pub struct Entry {
    start: i32,
    end: i32,
    letter: char,
    content: String,
}

lalrpop_mod!(pub grammar);


fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    println!("{}", reader.lines()
    .map(|l|grammar::LineParser::new().parse(&l.unwrap())
    .unwrap())
    .map(check2)
    .filter(|&x| x==true).count())
    // let expr = grammar::LineParser::new().parse("1-3 a: abcde");
}

fn check((start, end, letter, content): (usize,usize,char,String)) -> bool {
    let count = content.matches(letter).count();
    count >= start && count <= end
}

fn check2((start, end, letter, content): (usize,usize,char,String)) -> bool {
    (content.chars().nth(start-1).unwrap() == letter) ^ (content.chars().nth(end-1).unwrap()== letter)
}

#[test]
fn test1(){
    assert!(check2(grammar::LineParser::new().parse("1-3 a: abcde").unwrap()));
}