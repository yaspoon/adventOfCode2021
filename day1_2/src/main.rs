use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let input: Vec<i32> = contents.trim().lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut increases: i32 = 0;

    for i in 0..input.len() {
        if (i + 3) < input.len() {
            let window1: &[i32] = &input[i..=i+2];
            let window2: &[i32] = &input[i+1..=i+3];

            let sum1: i32 = window1.into_iter().sum();
            let sum2: i32 = window2.into_iter().sum();
            if sum2 > sum1 {
                increases += 1;
            }
        }
    }


    println!("increases:{}", increases);

    return Ok(());
}
