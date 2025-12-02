use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
struct ID {
    start: usize,
    end: usize,
}

impl TryFrom<String> for ID {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (start, end) = value.split_once('-').ok_or(())?;
        let start = start.parse().map_err(|_| ())?;
        let end = end.parse().map_err(|_| ())?;

        Ok(Self { start, end })
    }
}

impl ID {
    pub fn get_invalid_ids(&self) -> usize {
        (self.start..=self.end).fold(0, |acc, id| match get_duplicates(id) {
            true => acc + id,
            false => acc,
        })
    }
}

fn get_duplicates(num: usize) -> bool {
    let str = num.to_string();
    let (first, last) = str.split_at(str.len() / 2);
    first == last
}

fn get_duplicates_prev(num: usize) -> bool {
    let str: Vec<char> = num.to_string().chars().collect();

    for len in 1..(str.len() / 2) {
        let mut patterns: Vec<Vec<char>> = Vec::new();

        for chunk in str.chunks_exact(len).map(Vec::from) {
            if patterns.as_slice().contains(&chunk) {
                return true;
            } else {
                patterns.push(chunk)
            }
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
        let id: ID = id_range.try_into().unwrap_or(ID { start: 0, end: 0 });
        num_invalid_ids += id.get_invalid_ids();
    }

    println!("sum of invalid ids: {num_invalid_ids}");

    Ok(())
}
