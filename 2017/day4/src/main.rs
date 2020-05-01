use std::fs;

fn main() {
    let input_file = fs::read_to_string("/Users/zachspalding/Documents/Advent_of_Code/2017/AdventofCode2017/2017/Input Files/day4.txt").unwrap();
    let mut total = 0;
    for line in input_file.lines() {
        // perform logic on each line to check for duplicte words
        let mut words = Vec::new();
        let mut dups = false;
        for word in line.split_ascii_whitespace() {
            
            if words.contains(&word) {
                dups = true;
            } else {
                words.push(&word)
            }
        }
        if dups == false {
            total += 1;
        }
    }
    println!("{}", total);
}
