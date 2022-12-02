use std::collections::HashMap;
use std::fs;
use std::str::Lines;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should be able to read the input file.");

    const ROCK_OPP: &str = "A";
    const PAPER_OPP: &str = "B";
    const SCISSORS_OPP: &str = "C";

    const ROCK_PLAYER: &str = "X";
    const PAPER_PLAYER: &str = "Y";
    const SCISSORS_PLAYER: &str = "Z";

    const LOSE: &str = "X";
    const DRAW: &str = "Y";
    const WIN: &str = "Z";

    const LOSE_PTS: i32 = 0;
    const DRAW_PTS: i32 = 3;
    const WIN_PTS: i32 = 6;

    const ROCK_PTS: i32 = 1;
    const PAPER_PTS: i32 = 2;
    const SCISSORS_PTS: i32 = 3;

    let part_one_rules = HashMap::from([
        ((ROCK_OPP, ROCK_PLAYER), DRAW_PTS + ROCK_PTS),
        ((ROCK_OPP, PAPER_PLAYER), WIN_PTS + PAPER_PTS),
        ((ROCK_OPP, SCISSORS_PLAYER), LOSE_PTS + SCISSORS_PTS),
        ((PAPER_OPP, ROCK_PLAYER), LOSE_PTS + ROCK_PTS),
        ((PAPER_OPP, PAPER_PLAYER), DRAW_PTS + PAPER_PTS),
        ((PAPER_OPP, SCISSORS_PLAYER), WIN_PTS + SCISSORS_PTS),
        ((SCISSORS_OPP, ROCK_PLAYER), WIN_PTS + ROCK_PTS),
        ((SCISSORS_OPP, PAPER_PLAYER), LOSE_PTS + PAPER_PTS),
        ((SCISSORS_OPP, SCISSORS_PLAYER), DRAW_PTS + SCISSORS_PTS),
    ]);
    
    let part_two_rules = HashMap::from([
        ((ROCK_OPP, LOSE), LOSE_PTS + SCISSORS_PTS),
        ((ROCK_OPP, DRAW), DRAW_PTS + ROCK_PTS),
        ((ROCK_OPP, WIN), WIN_PTS + PAPER_PTS),
        ((PAPER_OPP, LOSE), LOSE_PTS + ROCK_PTS),
        ((PAPER_OPP, DRAW), DRAW_PTS + PAPER_PTS),
        ((PAPER_OPP, WIN), WIN_PTS + SCISSORS_PTS),
        ((SCISSORS_OPP, LOSE), LOSE_PTS + PAPER_PTS),
        ((SCISSORS_OPP, DRAW), DRAW_PTS + SCISSORS_PTS),
        ((SCISSORS_OPP, WIN), WIN_PTS + ROCK_PTS),
    ]);

    play_rock_paper_scissors(input.lines(), part_one_rules);
    play_rock_paper_scissors(input.lines(), part_two_rules);
}

fn play_rock_paper_scissors(input: Lines, part_one_rules: HashMap<(&str, &str), i32>) {
    let strategy_guide = input;
    let mut score = 0;
    for round in strategy_guide {
        let mut shapes = round.split(' ');

        let opponent_shape = shapes.next().unwrap();
        let player_shape = shapes.next().unwrap();

        score += part_one_rules[&(opponent_shape, player_shape)];
    }
    println!("{score}");
}
