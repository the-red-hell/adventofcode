use std::{
    fs::File,
    io::{self, BufReader},
};

struct PaperRollGrid {
    width: usize,
    height: usize,
    // true if there's a roll
    grid: Vec<Vec<bool>>,
}

impl PaperRollGrid {}

fn main() -> io::Result<()> {
    let file = File::open("day-three/input.txt")?;
    let reader = BufReader::new(file);

    println!("The largest possible joltage is: {}", 9);
    Ok(())
}
