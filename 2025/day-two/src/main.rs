use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num,
};

#[derive(Debug)]
struct ID {
    start: u64,
    end: u64,
}

impl TryFrom<String> for ID {
    type Error = num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (start, end) = value.split_once('-').unwrap();
        let start = start.parse()?;
        let end = end.parse()?;

        Ok(Self { start, end })
    }
}

impl ID {
    pub fn get_invalid_ids(&self) -> u64 {
        // change p2 to p1 to get part 1
        (self.start..=self.end).fold(0, |acc, id| match get_duplicates_p2(id) {
            true => acc + id,
            false => acc,
        })
    }
}

fn get_duplicates_p1(num: u64) -> bool {
    let str = num.to_string();
    let (first, last) = str.split_at(str.len() / 2);
    first == last
}

fn get_duplicates_p2(num: u64) -> bool {
    let str: Vec<char> = num.to_string().chars().collect();

    for len in 1..=(str.len() / 2) {
        if str.len() % len != 0 {
            continue;
        }
        let mut chunks = str.chunks_exact(len).map(Vec::from);
        let pattern = chunks.next().unwrap_or_default();

        if chunks.all(|chunk| chunk == pattern) {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    let file = File::open("day-two/input.txt")?;
    let mut id_ranges = BufReader::new(file).split(b',').filter_map(|id_range| {
        id_range
            .map(|id_range| id_range.into_iter().map(char::from).collect::<String>())
            .ok()
    });

    let mut num_invalid_ids = 0;
    while let Some(id_range) = id_ranges.next() {
        let id: ID = id_range.clone().try_into().unwrap();
        num_invalid_ids += id.get_invalid_ids();
    }

    println!("sum of invalid ids: {num_invalid_ids}");

    Ok(())
}
