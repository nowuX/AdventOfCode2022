use std::{collections::HashSet, fs};

fn marker(input: &Vec<char>, legth: usize) -> usize {
    let mut tmp = input.clone();
    for i in 0..tmp.len() {
        let buffer: HashSet<char> = tmp.iter().map(|x| *x).take(legth).collect();
        tmp.remove(0);
        if buffer.len() == legth {
            return i + legth;
        }
    }
    return 0;
}

fn main() {
    let text: String = fs::read_to_string("input.txt").unwrap();

    let input: Vec<char> = text.chars().collect();
    // Parts 1 and 2
    println!("{} chars to process.", marker(&input, 4));
    println!("{} chars to process(2).", marker(&input, 14));
}
