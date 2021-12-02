use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut input: Vec<i32> = contents.trim().lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut count = input.windows(2).filter(|x| x[0] < x[1]).count();
    println!("count:{}", count);

    return Ok(());
}
