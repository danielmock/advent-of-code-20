fn main() {
    let input = include_str!("../input");
    
    let mut input = input.lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    input.push(0);
    input.sort();
    input.push(input.iter().max().unwrap() + 3);

    let (a, b) = input.windows(2).map(|v| {
        v[1]-v[0]
    }).map(|x| {
        match x {
            1 => (1usize,0usize),
            3 => (0,1),
            _ => (0,0)
        }
    }).fold((0, 0), |(acc1, acc2), (el1, el2)| {
        (acc1 + el1, acc2 + el2)
    });

    println!("{}", a * (b+1));
    
    println!("part 2: {}", part2(&input));
    // let mut cache = vec![0; *input.last().unwrap()];
    // cache.push(1);

    // for x in input.into_iter().rev().skip(1) {
    //     cache[x] = cache[x+1] + cache[x+2] + cache[x+3];
    // }

    // // let mut result = vec![0; *input.last().unwrap() + 1];
    // // if let Some(last) = result.last_mut() {
    // //     *last = 1;
    // // }

    // // for &i in input.iter().rev().skip(1) {
    // //     result[i] = result[i+1] + result[i+2] + result[i+3]
    // // }

    // println!("{}", cache[0]);
    // println!("test");
}

fn part2(input: &Vec<usize>) -> usize {
    let mut cache = vec![0; *input.last().unwrap()];
    cache.push(1);

    for &x in input.iter().rev().skip(1) {
        cache[x] = cache[x+1] + cache[x+2] + cache[x+3];
    }
    cache[0]
}