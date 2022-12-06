use std::fs::File;
use std::io::{BufReader, Read};

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    let pathname = "input_day6.txt";
    let file = File::open(pathname).expect("can't open file");
    let mut line = String::new();
    BufReader::new(file).read_to_string(&mut line).expect("expected line");
    
    println!("part1 {}", part1(&line)); // 1655
    println!("part2 {}", part2(&line)); // 2665
}

fn count(s: &str, length: usize) -> i32 {
    let mut histogram : [i32; 26] = [0; 26];
    let mut doubles = 0;
    
    let b = s.as_bytes();
    for i in 0..b.len() {
        if b[i] < b'a' || b[i] > b'z' {
            return -1;
        }

        let a = (b[i] - b'a') as usize;
        histogram[a] += 1;
        if histogram[a] > 1 {
            doubles += 1;
        }

        if i >= length {
            let c = (b[i - length] - b'a') as usize;
            histogram[c] -= 1;
            if histogram[c] >= 1 {
                doubles -= 1;
            }

            if doubles <= 0 {
                return (i + 1) as i32;
            }
        }
    }

    return -1;
}

fn part1(s: &str) -> i32 { count(s, 4) }
fn part2(s: &str) -> i32 { count(s, 14) }

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

    #[test]
    fn part2_examples() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}