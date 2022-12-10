use std::fs;

fn main() {
    let text: String = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let mut overlap_full = 0;
    for section in text.lines() {
        let pair: Vec<Vec<i32>> = section
            .split(",")
            .map(|x| x.split("-").map(|x| x.parse::<i32>().unwrap()).collect())
            .collect();

        let is_first_contain = (pair[0][0] <= pair[1][0]) && (pair[0][1] >= pair[1][1]);
        let is_second_contain = (pair[0][0] >= pair[1][0]) && (pair[0][1] <= pair[1][1]);
        if is_first_contain || is_second_contain {
            overlap_full += 1;
        }
    }
    println!("I have overlap in {} pairs of assignments.", overlap_full);

    // Part 2
    let mut overlap = 0;
    for section in text.lines() {
        let pair: Vec<Vec<i32>> = section
            .split(",")
            .map(|x| x.split("-").map(|x| x.parse::<i32>().unwrap()).collect())
            .collect();

        let first_overlap = pair[0][0] <= pair[1][1];
        let second_overlap = pair[0][1] >= pair[1][0];
        if first_overlap && second_overlap {
            overlap += 1;
        }
    }
    println!("I have overlap(2) in {} pairs of assignments.", overlap);
}
