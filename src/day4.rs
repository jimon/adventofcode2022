use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use regex::Regex;

#[allow(dead_code)]
pub fn run() {
    let pathname = "input_day4.txt";
    let file = File::open(pathname).expect("can't open file");
    
    let pairs = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse(x))
        .filter_map(|x| x)
        .collect::<Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>>();

    let part1 = pairs
        .iter()
        .filter(|(l, r)| contained_in_each_other(l, r))
        .count();

    let part2 = pairs
        .iter()
        .filter(|(l, r)| overlap(l, r))
        .count();
    
    println!("part1 {}", part1); // 490
    println!("part2 {}", part2); // 921
}

fn parse(s: String) -> Option<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }

    for cap in RE.captures_iter(&s) {
        let al = cap[1].parse::<i32>().unwrap();
        let ar = cap[2].parse::<i32>().unwrap();
        let bl = cap[3].parse::<i32>().unwrap();
        let br = cap[4].parse::<i32>().unwrap();
        return Some((RangeInclusive::new(al,ar), RangeInclusive::new(bl, br)));
    }

    return None;
}

fn contained_in_each_other(l : &RangeInclusive<i32>, r : &RangeInclusive<i32>) -> bool {
    return (l.start() <= r.start() && l.end() >= r.end()) || (r.start() <= l.start() && r.end() >= l.end());
}

fn overlap(l : &RangeInclusive<i32>, r : &RangeInclusive<i32>) -> bool {
    return l.start() <= r.end() && l.end() >= r.start();
}
