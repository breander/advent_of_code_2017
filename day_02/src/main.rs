use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("File not found!");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Something went wrong!");

    let rows: Vec<&str> = contents.split("\n").collect();

    part1(&rows);
    part2(&rows);
}

fn part1(rows: &Vec<&str>) {
    let mut sum: i32 = 0;
    for row in rows {
       let numbers: Vec<&str> = row.split("\t").collect();
       let mut large: i32 = i32::min_value();
       let mut small: i32 = i32::max_value();

       for num in numbers {
            let current: i32 = num.parse().unwrap();

            if current > large {
                large = current;
            }

            if current < small {
                small = current;
            }
       } 

       sum += large - small;
    }

    println!("Part1: {}", sum);
}

fn part2(rows: &Vec<&str>) {
    let mut sum: i32 = 0;
    for row in rows {
       let numbers: Vec<&str> = row.split("\t").collect();

       for x in 0..numbers.len(){
            let current: i32 = numbers[x].parse().unwrap();
            let mut found: bool = false;

            for y in 0..numbers.len() {
                if x == y {
                    continue;
                }

                let current2: i32 = numbers[y].parse().unwrap();

                if current > current2 {
                    if current % current2 == 0 {
                        sum += current / current2;
                        found = true;
                        break;
                    }
                }
            }

            if found {
                break;
            }
       } 
    }

    println!("Part2: {}", sum);
}
