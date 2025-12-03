use std::{
    fs::File,
    io::{self, BufRead as _, BufReader},
    ops::Deref,
};

// change to false for part 1
const PART_2: bool = true;

struct Bank(Vec<u8>);

impl FromIterator<u8> for Bank {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Bank(iter.into_iter().collect())
    }
}

impl From<String> for Bank {
    fn from(value: String) -> Self {
        value
            .chars()
            .filter_map(|char| char.to_string().parse().ok())
            .collect()
    }
}

impl Deref for Bank {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Bank {
    fn get_largest_joltage(self) -> usize {
        let mut bank: Vec<u8> = self.iter().take(self.len() - 1).cloned().collect();

        let mut joltages: usize = 0;
        let num_digits: usize = PART_2.then_some(12).unwrap_or(2);
        for i in (1..num_digits).rev() {
            let mut avail_bank: Vec<(usize, u8)> = bank
                .iter()
                .cloned()
                .enumerate()
                .take(bank.len() + 1 - i)
                .collect();
            avail_bank.sort_by(|(a_i, a), (b_i, b)| {
                let cmp = a.cmp(b);
                match cmp {
                    std::cmp::Ordering::Equal => b_i.cmp(a_i),
                    _ => cmp,
                }
            });

            // get largest digit
            let (index, value) = avail_bank.pop().unwrap();
            joltages += 10_usize.pow(i as u32) * value as usize;
            assert_eq!(bank.drain(..=index).last(), Some(value));
        }

        // remove all indicies higher than the one of first_digit_index
        bank.sort();
        let last_digit = bank.pop().unwrap_or(0).max(*self.last().unwrap());

        joltages + last_digit as usize
    }
}

fn main() -> io::Result<()> {
    let file = File::open("day-three/input.txt")?;
    let reader = BufReader::new(file);

    let mut total_output_joltage = 0;
    for line in reader.lines() {
        let bank = Bank::from(line?);
        total_output_joltage += bank.get_largest_joltage();
    }

    println!("The largest possible joltage is: {}", total_output_joltage);
    Ok(())
}
