use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use std::collections::HashSet;

const N: usize = 2020;
fn main() {
    let mut data: HashSet<usize> = HashSet::new();

    let filename = "input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let number = line.unwrap().parse::<usize>().unwrap();
        data.insert(number);
    }

    for el1 in data.iter() {
        for el2 in data.iter() {
            for el3 in data.iter() {
                if el1 + el2 + el3 == N {
                    println!("{}", el1 * el2 * el3)
                }
            }
        }
    }
}

fn main2() {
    let mut data: [bool; N+1] = [false; N+1];

    let filename = "input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let number = line.unwrap().parse::<usize>().unwrap();
        data[number] = true;
    }

    for v in data.iter().enumerate().combinations(3) {
        if v.iter().map(|&(_,b)| b).all(&|b| b == &true) 
        && v.iter().map(|&(x,_)| x).sum::<usize>() == N {
            println!("{}", v.iter().map(|&(x,_)| x).product::<usize>());
            return;
        }
    }
}


fn main1() {
    let mut data: [bool; N+1] = [false; N+1];

    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let number = line.unwrap().parse::<usize>().unwrap();
        data[number] = true;
        if data[N - number] {
            println!("{}", number * (N - number));
        }
    }
}
