use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("File not found!");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Something went wrong!");

    part1(&contents);

    part2(&contents);
}

fn part1(contents: &String) {
    let mut sum: u64 = 0;
    let numbers = contents.chars();
    let mut first = 0;
    let mut previous = 0;

    for (pos, c) in numbers.enumerate() {
        let current = c as u64 - '0' as u64;

        if pos == 0 // FIRST
        {
            first = c as u64 - '0' as u64;
        }
        else if pos == (contents.len() - 1) //LAST
        {
            if current == previous {
                sum = sum + current;
            }

            if first == current{ //CHECK LOOP AROUND
                sum = sum + current;
            }
        }
        else
        { //MIDDLE
            if current == previous {
                sum = sum + current;
            }
        }

        previous = current;
    }

    println!("Part 1 Sum: {}", sum);
}

fn part2(contents: &String){
    let mut sum: u32 = 0;
    let number_strings = contents.chars();
    let mut numbers = Vec::new();
    let length: usize = contents.len() as usize;

    for c in number_strings {
        let current = c as u32 - '0' as u32;

        numbers.push(current);
    }

    let mut pos: usize = 0;
    for x in &numbers{
        let mut look_ahead = pos + (length / 2);

        if look_ahead > length - 1 {
            look_ahead = look_ahead - length;
        }
        
        if x == &numbers[look_ahead]{
            sum = sum + x;
        }

        pos = pos + 1;
    }

    println!("Part 2 Sum: {}", sum);
}
