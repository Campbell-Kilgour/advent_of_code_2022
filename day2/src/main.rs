fn main() {
    println!("Hello World!");
}

enum Move {
    Paper,
    Rock,
    Scissors,
}

// A - Rock
// B - Paper
// C - Scissors
//
// Y - Paper
// X - Rock


// dumb shit



// Z - Scissors

fn decrypt_strategy(letter: &str) -> Result<Move, &'static str> {
    match letter {
        "A" => Ok(Move::Rock),
        "B" => Ok(Move::Paper),
        "C" => Ok(Move::Scissors),
        "Y" => Ok(Move::Paper),
        "X" => Ok(Move::Rock),
        "Z" => Ok(Move::Scissors),
        _ => Err("Invalid input!"),
    }
}

fn get_score(input_move: Move) -> i32 {
    match input_move {
        Move::Paper => 2,
        Move::Rock => 1,
        Move::Scissors => 3,
    }
}
