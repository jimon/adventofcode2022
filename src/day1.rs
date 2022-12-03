use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn run() {
    let pathname = "input_day1.txt";
    let file = File::open(pathname).expect("can't open file");
    let mut sums : Vec<i32> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap_or_default().parse::<i32>().unwrap_or(0))
        .group_by(|x| *x == 0)
        .into_iter()
        .map(|(_, group)| group.sum::<i32>())
        .collect::<Vec<i32>>();

    sums.sort_by(|a, b| b.cmp(a));

    let part1 = sums
        .first()
        .copied()
        .unwrap_or_default();
    println!("part1 {}", part1);

    let part2 = sums[0] + sums[1] + sums[2];
    println!("part2 {}", part2);
}