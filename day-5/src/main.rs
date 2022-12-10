use std::fs;

fn main() {
    let text: String = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let (stacks, moves) = text.split_once("\n\n").unwrap();

    let mut boxes: Vec<Vec<char>> = Vec::new();
    for st in stacks.lines().rev().skip(1) {
        let only_box: Vec<char> = st
            .chars()
            .enumerate()
            .filter(|(idx, _)| idx % 2 != 0 && (1 + idx) % 4 != 0)
            .map(|(_, v)| v)
            .collect();
        boxes.push(only_box);
    }
    boxes = (1..=boxes[0].len())
        .map(|i| boxes.iter_mut().map(|row| row[row.len() - i]).collect())
        .collect();
    boxes.reverse();
    for b in boxes.iter_mut() {
        b.retain(|x| !x.is_whitespace());
        b.reverse();
    }

    for mov in moves.lines() {
        let (_, a) = mov.split_at(5);
        let a: Vec<usize> = a
            .replace(" to ", " from ")
            .split(" from ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        for _ in 0..a[0] {
            let box_from = *boxes[a[1] - 1].first().unwrap();
            boxes[a[2] - 1].insert(0, box_from);
            boxes[a[1] - 1].remove(0);
        }
    }
    let top_creates: String = boxes.iter().map(|x| x.first().unwrap()).collect();
    println!("In top of stacks end {} crates.", top_creates);

    // Part 2 🚧
}
