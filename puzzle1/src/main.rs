use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<T> (filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where T: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_data(filename: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut elves: Vec<u32> = vec![0];
    let mut current_index:usize = 0;

    let lines = read_lines(filename)?;
    for line in lines {
        if let Ok(line) = line {
            if line.trim().is_empty() {
                elves.push(0);
                current_index += 1;
            }
            else {
                elves[current_index] += line.parse::<u32>().unwrap();
            }
        }
    }

    return Ok(elves);
}

fn part1(data: &Vec<u32>) {
    print!("part1:\n{}\n", *data.iter().max().unwrap());
}


fn part2(data: &Vec<u32>) {
    // copy the borrowed vector to a mutable vector so we will be able to perform mutations on it
    let mut internal_data = data.to_vec();
    internal_data.sort();
    internal_data.reverse();

    let result = internal_data[0] + internal_data[1] + internal_data[2];

    print!("part2:\n{}\n", result);
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = read_data("input.txt")?;

    part1(&data);
    part2(&data);

    return Ok(());
}

