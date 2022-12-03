use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}

enum GameResult {
    Lost,
    Draw,
    Won
}

struct Rule {
    a: Move,
    b: Move
}

#[allow(dead_code)]
pub fn run() {
    let pathname = "input_day2.txt";
    let file = File::open(pathname).expect("can't open file");
    let moves : Vec<Rule> = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .filter(|x| x.len() == 3)
        .map(|x| {
            let (a, b) = parse_pair(x).unwrap();
            return Rule { a: Move::char_to_move(a).unwrap(), b: Move::char_to_move(b).unwrap() };
        })
        .collect()
    ;
    
    let points : i32 = moves
        .into_iter()
        .map(|x| (play_game(x.a, x.b), x.b))
        .map(|(r, m)| result_to_points(r, m))
        .sum()
    ;
    
    println!("part1 {}", points); // 14264
    

}

fn parse_pair(s : String) -> Option<(char, char)> {
    let mut c = s.chars();
    let a = c.nth(0)?;
    let b = c.nth(1)?;
    return Some((a, b));
} 

impl Move {
    fn char_to_move(c: char) -> Option<Move> {
        match c {
            'A' => Some(Move::Rock),
            'B' => Some(Move::Paper),
            'C' => Some(Move::Scissors),
            'X' => Some(Move::Rock),
            'Y' => Some(Move::Paper),
            'Z' => Some(Move::Scissors),
            _ => None
        }
    }
}

fn play_game(enemy: Move, ours: Move) -> GameResult {
    return match (ours, enemy) {
        (Move::Rock, Move::Rock) => GameResult::Draw,
        (Move::Rock, Move::Paper) => GameResult::Lost,
        (Move::Rock, Move::Scissors) => GameResult::Won,
        (Move::Paper, Move::Rock) => GameResult::Won,
        (Move::Paper, Move::Paper) => GameResult::Draw,
        (Move::Paper, Move::Scissors) => GameResult::Lost, 
        (Move::Scissors, Move::Rock) => GameResult::Lost,
        (Move::Scissors, Move::Paper) => GameResult::Won,
        (Move::Scissors, Move::Scissors) => GameResult::Draw
    }
}

fn result_to_points(result: GameResult, m: Move) -> i32 {
    let result_score = match result {
        GameResult::Lost => 0,
        GameResult::Draw => 3,
        GameResult::Won => 6
    };
    let move_score = match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }; 
    return result_score + move_score;
}