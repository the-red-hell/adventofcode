use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
struct Dial {
    pos: isize,
    wraps: usize,
}

impl Dial {
    pub fn new() -> Self {
        Dial { pos: 50, wraps: 0 }
    }

    pub fn turn_right_p1(&mut self, amount: usize) {
        let mut new_pos = self.pos.strict_add_unsigned(amount);
        if new_pos >= 100 {
            new_pos = new_pos % 100;
        }
        self.pos = new_pos;

        self.check_0();
    }

    pub fn turn_left_p1(&mut self, amount: usize) {
        let mut new_pos = self.pos.strict_sub_unsigned(amount);
        if new_pos < 0 {
            new_pos = 100 + (new_pos % 100);
            if new_pos == 100 {
                new_pos = 0
            }
        }
        self.pos = new_pos;

        self.check_0();
    }

    pub fn turn_left_p2(&mut self, amount: usize) {
        for i in 0..amount {
            self.pos -= 1;
            if self.pos == -1 {
                self.pos = 99;
                if i != 0 {
                    self.wraps += 1;
                }
            }
        }
        self.check_0();
    }

    pub fn turn_right_p2(&mut self, amount: usize) {
        for _ in 0..amount {
            self.pos += 1;
            if self.pos == 100 {
                self.pos = 0;
                self.wraps += 1;
            }
        }
    }

    fn check_0(&mut self) {
        if self.pos == 0 {
            self.wraps += 1;
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("day-one/input.txt")?;
    let mut lines = BufReader::new(file)
        .lines()
        .filter(|line| line.as_ref().is_ok_and(|l| !l.is_empty()));

    let mut dial = Dial::new();

    while let Some(Ok(line)) = lines.next() {
        let (operation, amount) = line.split_at(1);
        let Ok(amount) = amount.parse() else {
            panic!("{}", amount);
        };

        // to do part one, replace the `p2` with `p1`
        match operation {
            "L" => dial.turn_left_p2(amount),
            "R" => dial.turn_right_p2(amount),
            _ => continue,
        }
    }

    println!("Finished with wrapping {} times.", dial.wraps);
    Ok(())
}
