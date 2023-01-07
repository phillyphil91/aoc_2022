use std::io::prelude::*;
use std::fs::File;
use itertools::Itertools;

pub fn day1() {
    let mut file = File::open("./input/day1_a.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let part_a = contents
        .split("\n\n")
        .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
        .max()
        .unwrap()
        .to_string();

    println!("Part A: {}", part_a);


    let part_b: Vec<u32> = contents
        .split("\n\n")
        .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
        .sorted()
        .collect();

    let part_b_solution = &part_b[part_b.len() - 3 .. part_b.len()];
    let part_b_solution_sum: u32 = part_b_solution.iter().sum();

    println!("Part B: {}", part_b_solution_sum);
        
}
