use std::{collections::HashMap, fs, path::PathBuf};

fn main() {
    let text: String = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let mut tree: HashMap<PathBuf, i32> = HashMap::new();
    let mut route: Vec<&str> = Vec::new();

    for line in text.lines() {
        let xs: Vec<&str> = line.split_whitespace().collect();
        match xs[..] {
            ["$", "ls"] => continue,
            ["dir", _folder] => continue,
            ["$", "cd", ".."] => {
                route.pop();
            }
            ["$", "cd", folder] => {
                route.push(folder);
            }
            [size, _file] => {
                let size: i32 = size.parse().unwrap();
                for i in 0..route.len() {
                    let path = PathBuf::from_iter(&route[..=i]);
                    *tree.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }
    println!(
        "Sum of directories {}.",
        tree.into_values().filter(|x| *x < 100000).sum::<i32>()
    );
}
