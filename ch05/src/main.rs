fn main() {
    let input = include_str!("../input");

    let min = input.lines().map(to_seat_id).min().unwrap();
    let max = input.lines().map(to_seat_id).max().unwrap();
    let expected_sum = max * (max+1) / 2 - ( (min-1) * min / 2);
    let actual_sum : usize = input.lines().map(to_seat_id).sum();
    println!("{}", expected_sum - actual_sum);
}

fn to_seat_id(s: &str) -> usize {
    let bin_row = s[..7].chars().map(|x| match x {
        'F' => '0',
        'B' => '1',
        _ => unreachable!()}).collect::<String>();
    let row = usize::from_str_radix(&bin_row, 2).unwrap();

    let bin_col = s[7..10].chars().map(|x| match x {
        'L' => '0',
        'R' => '1',
        _ => unreachable!()}).collect::<String>();
    let col = usize::from_str_radix(&bin_col, 2).unwrap();
    
    8* row + col
}
