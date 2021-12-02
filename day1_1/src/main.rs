use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut input: Vec<i32> = contents.trim().lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut previous: i32 = i32::MAX;
    let mut increases: i32 = 0;

    for line in input {
        if line > previous {
            increases += 1;
        }
        previous = line;
    }

    println!("increases:{}", increases);

    return Ok(());
}
