use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(Copy, Clone)]
enum GameResult {
    Lost,
    Draw,
    Won
}

#[allow(dead_code)]
pub fn run() {
    let pathname = "input_day2.txt";
    let file = File::open(pathname).expect("can't open file");
    let parsed : Vec<(char, char)>= BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .filter(|x| x.len() == 3)
        .map(|x| parse_pair(x).unwrap())
        .collect()
        ;

    let part1: i32 = parsed
        .iter()
        .map(|(a, b)| (Move::try_parse(*a).unwrap(), Move::try_parse(*b).unwrap()))
        .map(|(enemy, ours)| (play_game(enemy, ours), ours))
        .map(|(r, m)| result_to_points(r, m))
        .sum();

    let part2: i32 = parsed
        .iter()
        .map(|(a, b)| (Move::try_parse(*a).unwrap(), GameResult::try_parse(*b).unwrap()))
        .map(|(r, m)| result_to_points(m, find_move(r, m)))
        .sum();

    println!("part1 {}", part1); // 14264
    println!("part2 {}", part2); // 11732 too low
}

fn parse_pair(s : String) -> Option<(char, char)> {
    let mut c = s.chars();
    let a = c.nth(0)?;
    let b = c.nth(1)?;
    return Some((a, b));
} 

impl Move {
    fn try_parse(c: char) -> Option<Move> {
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

impl GameResult {
    fn try_parse(c: char) -> Option<GameResult> {
        match c {
            'X' => Some(GameResult::Lost),
            'Y' => Some(GameResult::Draw),
            'Z' => Some(GameResult::Won),
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

fn find_move(enemy: Move, outcome: GameResult) -> Move {
    return match (enemy, outcome) {
        (Move::Rock, GameResult::Lost) => Move::Scissors,
        (Move::Rock, GameResult::Draw) => Move::Rock,
        (Move::Rock, GameResult::Won) => Move::Paper,
        (Move::Paper, GameResult::Lost) => Move::Rock,
        (Move::Paper, GameResult::Draw) => Move::Paper,
        (Move::Paper, GameResult::Won) => Move::Scissors,
        (Move::Scissors, GameResult::Lost) => Move::Paper,
        (Move::Scissors, GameResult::Draw) => Move::Scissors,
        (Move::Scissors, GameResult::Won) => Move::Rock 
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