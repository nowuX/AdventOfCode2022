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
    let mut boxes_2 = boxes.clone();

    let ins = moves
        .lines()
        .map(|x| x.split_at(5))
        .map(|(_, x)| {
            x.replace(" to ", " from ")
                .split(" from ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();
    for mov in ins.iter() {
        let (ins, from, to) = (mov[0], mov[1], mov[2]);
        for _ in 0..ins {
            let box_from = *boxes[from - 1].first().unwrap();
            boxes[to - 1].insert(0, box_from);
            boxes[from - 1].remove(0);
        }
    }
    let top_creates: String = boxes.iter().map(|x| x.first().unwrap()).collect();
    println!("{} end at the top of the stack of crates.", top_creates);

    // Part 2
    for mov in ins.iter() {
        let (ins, from, to) = (mov[0], mov[1], mov[2]);
        let removed_from: Vec<char> =
            (*boxes_2[from - 1].splice(0..ins, []).collect::<Vec<char>>()).to_vec();
        boxes_2[to - 1].splice(0..0, removed_from.iter().cloned());
    }
    let top_creates: String = boxes_2.iter().map(|x| x.first().unwrap()).collect();
    println!("{} end at the top of the stack of crates(2).", top_creates);
}
