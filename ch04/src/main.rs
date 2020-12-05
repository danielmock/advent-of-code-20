
const RESULT1:&str = &"byrcidecleyrhclhgtiyrpid";
const RESULT2:&str = &"byrecleyrhclhgtiyrpid";
fn main() {
    let input = include_str!("../input");

    // println!("{}", input.split("\n\n").next().unwrap());

    println!("{}", input.split("\n\n")
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .filter(check)
        .count() 
    );
}

fn check(v: &Vec<&str>) -> bool{
    let mut v = v.iter()
        .map(|s|s.split(":").next().unwrap())
        .collect::<Vec<&str>>();
    v.sort();
    v.join("") == RESULT1 || v.join("") == RESULT2
}