use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

// TODO this is a fairly bad rust code, make it more canonical

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    let pathname = "input_day7.txt";
    let file = File::open(pathname).expect("can't open file");
    let mut line = String::new();
    BufReader::new(file).read_to_string(&mut line).expect("expected line");
    
    println!("part1 {}", part1(&line));
    println!("part2 {}", part2(&line));
}

fn to_files(lines_concat: &str) -> HashMap<PathBuf, u64> {
    let lines = lines_concat.lines().collect::<Vec<&str>>();

    let mut files : HashMap<PathBuf, u64> = HashMap::new();
    let mut cwd = PathBuf::new();

    let mut i = 0;
    while i < lines.len()
    {
        let line = &lines[i];
        if line.starts_with("$ cd") {
            i += 1;

            let cd = line.strip_prefix("$ cd ").expect("");
    
            if cd == "/" {
                cwd = PathBuf::new();
            } else if cd == ".." {
                cwd.pop();
            } else {
                cwd.push(cd);
            }
        }
        else if line.starts_with("$ ls") {
            i += 1;
            
            if i >= lines.len() {
                continue;
            }

            while lines[i].as_bytes()[0] != b'$' {
                let line2 = lines[i];
                i += 1;
                if i >= lines.len() {
                    break;
                }
                if line2.as_bytes()[0] == b'd' {
                    continue;
                }

                let size = line2.split_once(' ').expect("split").0.parse::<u64>().expect("size");

                let mut cwdcopy = cwd.clone();

                loop {
                    
                    
                    if files.contains_key(&cwdcopy) {
                        *files.get_mut(&cwdcopy).expect("abc") += size;
                    } else {
                        files.insert(cwdcopy.clone(), size);
                    }

                    if !cwdcopy.pop() {
                        break;
                    }
                }
            }
            
        } else {
            println!("not expected line '{}'", line);
            return HashMap::new();
        }
    }
    
    return files;

}

fn part1(lines_concat: &str) -> u64 {
    to_files(lines_concat).into_values().filter(|&x| x < 100000).sum()
}

fn part2(lines_concat: &str) -> u64 {
    let files = to_files(lines_concat);
    let total = files.get(&PathBuf::new()).expect("abc").clone();

    return files.into_values().filter(|x|  70000000 - (total - x) >= 30000000 ).min().expect("abc");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"), 95437);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"), 17719346);
    }
}