use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub fn run() {
    let pathname = "input_day1.txt";
    let file = File::open(pathname).expect("can't open file");
    let max = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap_or_default().parse::<i32>().unwrap_or(0))
        .group_by(|x| *x == 0)
        .into_iter()
        .map(|(_, group)| group.sum::<i32>())
        .max()
        .unwrap_or_default();
    println!("part1 {}", max);
}