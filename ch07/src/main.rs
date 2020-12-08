fn main() {
    
    let input = include_str!("../input");
}

fn parse_line(line: &str) {
    let iter = line.split(" ");

    let bag = iter.take(2).collect::<Vec<&str>>().join(" ");
    
    assert_eq!(iter.next(), "bags");
    assert_eq!(iter.next(), "contain");

    
}