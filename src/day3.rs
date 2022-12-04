use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn run() {

    let pathname = "input_day3.txt";
    let file = File::open(pathname).expect("can't open file");
    let lines : Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .collect();
    
    let part1 : i32 = lines
        .iter()
        .map(|x| str_to_compartments(x))
        .map(|(l, r)| l.intersection(&r).last().unwrap().clone())
        .map(|x| char_to_score(x))
        .sum()
        ;

    let part2 : i32 = lines
        .iter()
        .map(|x| str_to_hashset(x))
        .collect::<Vec<HashSet<char>>>()
        //.slice()
        .chunks_exact(3)
        .map(|x| {
            let (a, b, c) = (&x[0], &x[1], &x[2]);
            let ab : HashSet<char> = a.intersection(b).map(|x| *x).collect();
            let abc = ab.intersection(c).map(|x| *x).last().unwrap();
            return char_to_score(abc);
        })
        .sum()
        ;

    println!("part1 {}", part1); // 7746
    println!("part2 {}", part2); // 2604 
}

fn str_to_compartments(x : &String) -> (HashSet<char>, HashSet<char>) {
    let size = x.len();
    assert_eq!(size % 2, 0);

    let (l, r) = x.split_at(size / 2);
    assert_eq!(l.len(), r.len());

    let lhs = l.chars().collect::<HashSet<char>>();
    let rhs = r.chars().collect::<HashSet<char>>();
    return (lhs, rhs);
}

fn str_to_hashset(x : &String) -> HashSet<char> {
    return x.chars().collect::<HashSet<char>>();
}

fn char_to_score(c : char) -> i32 {
    let u = c as u8;
    let ca = 'a';
    let cua = 'A';
    let cau = ca as u8;
    let cuau = cua as u8;
    if c >= 'a' && c <= 'z' {
        return (u - cau) as i32 + 1;
    } else if c >= 'A' && c <= 'Z' {
        return (u - cuau) as i32 + 27;
    } else {
        return 0;
    }
}