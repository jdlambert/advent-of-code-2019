use std::fs;

fn digit(n: usize, data: &Vec<i32>) -> i32 {
    let mut index = 0;
    let mut count = 1;
    let mut sum = 0;
    for num in data {
        if count > n {
            count = 0;
            index += 1;
        }
        sum += [0, 1, 0, -1][index % 4] * num;
        count += 1;
    }
    if sum < 0 {
        (-sum) % 10
    } else {
        sum % 10
    }
}

fn next(data: &Vec<i32>) -> Vec<i32> {
    (0..data.len()).map(|i| digit(i, &data)).collect()
}

fn part1(data: &Vec<i32>) -> Vec<i32> {
    let mut data = data.clone();
    for _ in 0..100 {
        data = next(&data);
    }
    data
}

fn part2(digits: &[i32]) -> i32 {
    let target = digits[..7].iter().fold(0, |n, &d| 10 * n + d) as usize;
    // The following method only works in the second half of a given input
    assert!(target > (digits.len() * 10_000) / 2);

    let mut suffix: Vec<_> = digits
        .iter()
        .copied()
        .rev()
        .cycle()
        .take(digits.len() * 10_000 - target)
        .collect();

    for _ in 0..100 {
        suffix = suffix
            .iter()
            .scan(0, |sum, x| {
                *sum += x;
                Some(sum.abs() % 10)
            })
            .collect();
    }

    suffix.iter().rev().take(8).fold(0, |n, &d| 10 * n + d)
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let data = content
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    println!("Part 1: ");
    for num in part1(&data).iter().take(8) {
        print!("{}", num);
    }
    print!("\n");
    println!("Part 2: {}", part2(&data));
}
