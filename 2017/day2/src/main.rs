// the goal for this program is to find difference between the max and min number from each row, 
// then add them all together

use std::fs;
fn main() {
   
    let input_file = fs::read_to_string("/Users/zachspalding/Documents/Advent_of_Code/2017/AdventofCode2017/2017/Input Files/day2.txt").unwrap();

    println!("Part 1: {}", part1(&input_file)); 
    println!("Part 2: {}", part2(&input_file));   
}


fn part1(s: &String) -> u16 {
    let mut total = 0;

    for line in s.lines() {
        // try to collect the line by seperating by whitespace?
        let v: Vec<u16> = line.split_whitespace().map(|x| x.parse::<u16>().unwrap()).collect();
        
        total += v.iter().max().unwrap() - v.iter().min().unwrap()
    }
    
    total
}

fn part2(s: &String) -> u16 {
    let mut total = 0;

    for line in s.lines() {
        let v: Vec<u16> = line.split_whitespace().map(|x| x.parse::<u16>().unwrap()).collect();

        for i in 0..v.len() {
            for j in i+1..v.len() {
                if v[i] % v[j] == 0 {
                    total += v[i] / v[j];
                } else if v[j] % v[i] == 0 {
                    total += v[j] / v[i];
                }
            }
        }
    }

    total
}