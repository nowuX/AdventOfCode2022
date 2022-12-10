use std::{collections::HashSet, fs};

fn main() {
    // Part 1
    // a-z => 1-26, A-Z => 27-52
    let text: String = fs::read_to_string("input.txt").unwrap();

    let mut item_priority: Vec<i32> = Vec::new();
    let abc = format!(
        "{}{}",
        ('a'..='z').collect::<String>(),
        ('A'..='Z').into_iter().collect::<String>(),
    );

    for rucksack in text.lines() {
        let (a, b) = rucksack.split_at(rucksack.len() / 2);
        let (a_hash, b_hash): (HashSet<char>, HashSet<char>) =
            (a.chars().collect(), b.chars().collect());

        let repited: Vec<&char> = a_hash.intersection(&b_hash).collect();
        let index = abc.chars().position(|x| x == *repited[0]).unwrap();

        item_priority.push(1 + index as i32);
    }
    println!(
        "The sum of priorities is {}.",
        item_priority.iter().sum::<i32>()
    );

    // Part 2
    let elves: Vec<&str> = text.lines().collect();
    let elves_groups: Vec<&[&str]> = elves.chunks(3).collect();
    let mut badges_priority: Vec<i32> = Vec::new();

    for group in elves_groups {
        let mut hash_rucksacks = group
            .iter()
            .map(|line| line.chars().collect::<HashSet<char>>());
        let mut repeated: HashSet<char> = hash_rucksacks.next().unwrap();
        for rucksack in hash_rucksacks {
            repeated = repeated.intersection(&rucksack).copied().collect();
        }
        for i in &repeated {
            let index = abc.chars().position(|x| x == *i).unwrap();
            badges_priority.push(1 + index as i32);
        }
    }
    println!(
        "The sum of priorities(2) is {}.",
        badges_priority.iter().sum::<i32>()
    );
}
