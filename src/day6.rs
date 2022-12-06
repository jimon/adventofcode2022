use std::fs::File;
use std::io::{BufReader, Read};

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    let pathname = "input_day6.txt";
    let file = File::open(pathname).expect("can't open file");
    let mut line = String::new();
    BufReader::new(file).read_to_string(&mut line).expect("expected line");
    
    let part1 = part1(&line);
    println!("part1 {}", part1); // 1655
}

fn part1(s: &str) -> i32 {
    let b = s.as_bytes();
    for i in 4..b.len() {
        if  (b[i - 4] != b[i - 3]) &&
            (b[i - 4] != b[i - 2]) &&
            (b[i - 3] != b[i - 2]) &&
            (b[i - 4] != b[i - 1]) &&
            (b[i - 3] != b[i - 1]) &&
            (b[i - 2] != b[i - 1]) {
            return i as i32;
        }
    }
    
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}