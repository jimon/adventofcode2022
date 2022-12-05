use std::fs::File;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    let pathname = "input_day5.txt";
    let file = File::open(pathname).expect("can't open file");

    let lines = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<String>>();
    
    let mut split = lines.split(|x| x.len() == 0);
    let stack_str = split.next().expect("stack");
    let commands_str = split.next().expect("commands");

    let towers = (stack_str[0].len() + 1) / 4;
    println!("towers {}", towers);
    
    let mut stacks : Vec<Vec<u8>> = Vec::new();
    stacks.resize(towers, Vec::new());
    
    for column in 0..towers {
        for row in (0..stack_str.len()).rev() {
            let char = stack_str[row].as_bytes()[column * 4 + 1];
            if char == b' ' {
                continue;
            }
            stacks[column].push(char);
        }
    }

    let re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let part2 = true;

    for command_str in commands_str {
        for cap in re.captures_iter(command_str) {
            if cap.len() < 3 {
                continue;
            }
            
            let count = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let to = cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

            if part2 {
                // TODO find better way
                let mut t = Vec::new();
                for i in 0..count {
                    let char = stacks[from].pop().unwrap();
                    t.push(char);
                }
                for el in t.iter().rev() {
                    stacks[to].push(*el);
                }
            } else {
                for i in 0..count {
                    let char = stacks[from].pop().unwrap();
                    stacks[to].push(char);
                }
            }
        }
    }

    let result = String::from_iter((0..towers)
        .map(|x| stacks[x].last().unwrap().clone())
        .map(|x| char::from(x)));
    
    println!("{}", result);
}

