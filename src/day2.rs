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
            let mut c = x.chars(); // can we avoid mut?
            let m1 = char_to_move(c.nth(0).unwrap_or_default()).unwrap();
            let m2 = char_to_move(c.nth(1).unwrap_or_default()).unwrap();
            return Rule { a: m1, b: m2 };
        })
        .collect()
    ;
    
    let points : i32 = moves
        .into_iter()
        .map(|x| (play_game(x.a, x.b), x.b))
        .map(|(r, m)| result_to_points(r, m))
        .sum()
    ;
    
    println!("points {}", points);
    
    // 9456 too low
    
    //foo.fi

}

#[derive(Debug)]
pub enum MoveParsingError {
    UnknownMove
}

fn char_to_move(c : char) -> Result<Move, MoveParsingError> {
    match c {
        'A' => Ok(Move::Rock),
        'B' => Ok(Move::Paper),
        'C' => Ok(Move::Scissors),
        'X' => Ok(Move::Rock),
        'Y' => Ok(Move::Paper),
        'Z' => Ok(Move::Scissors),
        _ => Err(MoveParsingError::UnknownMove)
    }
}

fn play_game(enemy: Move, ours: Move) -> GameResult {
    return match ours {
        Move::Rock => {
            match enemy {
                Move::Rock => GameResult::Draw,
                Move::Paper => GameResult::Lost,
                Move::Scissors => GameResult::Won
            }
        }
        Move::Paper => {
            match enemy {
                Move::Rock => GameResult::Won,
                Move::Paper => GameResult::Draw,
                Move::Scissors => GameResult::Lost
            }
        }
        Move::Scissors => {
            match enemy {
                Move::Rock => GameResult::Lost,
                Move::Paper => GameResult::Won,
                Move::Scissors => GameResult::Draw
            }
        }
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