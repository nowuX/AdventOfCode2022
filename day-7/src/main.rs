use std::{collections::HashMap, fs};

fn main() {
    let text: String = fs::read_to_string("input_2.txt").unwrap();

    // Part 1
    let mut cwd: &str = "/";

    let mut tree: HashMap<&str, i32> = HashMap::new();
    for line in text.lines() {
        let match_1 = line.chars().take(4).collect::<String>();
        match match_1.as_str() {
            "$ cd" => {
                let xd = cwd.clone();
                match line.chars().skip(5).take(2).collect::<String>().as_str() {
                    ".." => cwd = &xd.split("/").collect::<Vec<&str>>()[..xd.len() - 1].join("/"),
                    "/" => cwd = "/",
                    _ => cwd = &format!("{}", xd),
                }
            }
            "$ ls" => {}
            "dir " => {}
            _ => {
                let (a, _) = line.split_once(" ").unwrap();
                let folder_size = tree.entry("").or_insert(0);
                *folder_size += a.to_string().parse::<i32>().unwrap();
            }
        };
    }
}
