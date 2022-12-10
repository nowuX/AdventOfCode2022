use std::fs;

fn main() {
    // Part 1
    let text: String = fs::read_to_string("input.txt").unwrap();
    let mut calories: Vec<i32> = Vec::new();

    for elf in text.split("\n\n") {
        let vec_calories: Vec<i32> = elf.lines().map(|x| x.parse::<i32>().unwrap()).collect();
        calories.push(vec_calories.iter().sum::<i32>());
    }
    println!(
        "That elf is carrying {} calories.",
        calories.iter().max().unwrap()
    );

    // Part 2
    calories.sort();
    let mut top_3: Vec<i32> = Vec::new();

    for i in 1..=3 {
        top_3.push(calories[calories.len() - i])
    }
    println!(
        "The top 3 elves is carrying {} calories.",
        top_3.iter().sum::<i32>()
    );
}
