use std::fs;

fn main() {
    // Part 1
    // A, X => Rock(1), B, Y => Paper(2), C, Z => Scissors(3)
    let text: String = fs::read_to_string("input.txt").unwrap();

    let mut score: Vec<i32> = Vec::new();
    for i in text.lines() {
        score.push(match i {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        });
    }
    println!("My total score is {}.", score.iter().sum::<i32>());

    // Part 2
    // X Lose(1), Y Draw(3), Z Win(6)
    let mut new_score: Vec<i32> = Vec::new();
    for i in text.lines() {
        new_score.push(match i {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        });
    }
    println!("My total score(2) is {}.", new_score.iter().sum::<i32>());
}
