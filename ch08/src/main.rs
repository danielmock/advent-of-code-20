use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let len = input.lines().count();

    let mut line_number = 0i32;
    let mut acc = 0i32;
    let mut visited_lines = HashSet::new();
    
    let lines2change = loop {
        if !visited_lines.insert(line_number) {
            break visited_lines.clone();
        }
        let mut line = input.lines().nth(line_number as usize).unwrap().split(" ");
        let instruction = line.next().unwrap();
        let number = line.next().unwrap().parse::<i32>().unwrap();
        match instruction {
            "jmp" => line_number = line_number + number,
            "acc" => {acc = acc + number;  line_number = line_number + 1;}
            _ => line_number = line_number + 1
        }
    };

    for i in lines2change {
        let mut line_number = 0i32;
        let mut acc = 0i32;
        let mut visited_lines = HashSet::new();
        let result = loop {
            if line_number == len as i32 {
                break Some(acc);
            }
            if !visited_lines.insert(line_number) {
                break None;
            }
            let mut line = input.lines().nth(line_number as usize).unwrap().split(" ");
            let instruction = line.next().unwrap();
            let number = line.next().unwrap().parse::<i32>().unwrap();
            if instruction == "jmp" && i != line_number || instruction == "nop" && i == line_number {
                line_number = line_number + number
            } else if instruction == "acc" {
                acc = acc + number;  
                line_number = line_number + 1;
            } else {
                line_number = line_number + 1;
            }
        };
        
        if let Some(x) = result {
            println!("{}", x);
            return;
        }
    }
}
