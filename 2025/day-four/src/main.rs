use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
struct PaperRollGrid {
    width: usize,
    height: usize,
    // true if there's a roll
    grid: Vec<Vec<bool>>,
}

impl PaperRollGrid {
    fn new(str: String) -> Self {
        let height = str.lines().count();
        let width = str.lines().next().unwrap().len();
        let mut grid: Vec<Vec<bool>> = Vec::new();

        grid.push(vec![false; width + 2]);
        grid.append(
            &mut str
                .lines()
                .map(|line| {
                    let mut padded_line = String::new();
                    padded_line.push('.');
                    padded_line.push_str(line);
                    padded_line.push('.');

                    padded_line
                        .chars()
                        .map(|char| match char {
                            '.' => false,
                            '@' => true,
                            _ => panic!("unknown character: {char}"),
                        })
                        .collect()
                })
                .collect(),
        );
        grid.push(vec![false; width + 2]);

        Self {
            width,
            height,
            grid,
        }
    }

    /// this is hell, don't have time rn tbh, it's still very fast tbh
    fn num_forklift_can_access_p1(&self) -> usize {
        let mut number = 0;

        for (line_index, line) in self.grid.iter().enumerate().skip(1).take(self.height) {
            for (item_index, _) in line
                .into_iter()
                .enumerate()
                .skip(1)
                .take(self.width)
                .filter(|(_, item)| **item)
            {
                let top_left = self.grid[line_index - 1][item_index - 1];
                let top_middle = self.grid[line_index - 1][item_index];
                let top_right = self.grid[line_index - 1][item_index + 1];

                let left = self.grid[line_index][item_index - 1];
                let right = self.grid[line_index][item_index + 1];

                let bottom_left = self.grid[line_index + 1][item_index - 1];
                let bottom_middle = self.grid[line_index + 1][item_index];
                let bottom_right = self.grid[line_index + 1][item_index + 1];
                // oh man

                let adjacent = [
                    top_left,
                    top_middle,
                    top_right,
                    left,
                    right,
                    bottom_left,
                    bottom_middle,
                    bottom_right,
                ];
                if adjacent.iter().filter(|item| **item).count() < 4 {
                    number += 1;
                }
            }
        }

        number
    }

    /// this is hell, don't have time rn tbh, it's still very fast tbh
    fn num_forklift_can_access_p2(&mut self) -> usize {
        let mut number = 0;

        loop {
            let (mut line_i_to_remove, mut item_i_to_remove) = (Vec::new(), Vec::new());

            for (line_index, line) in self.grid.iter().enumerate().skip(1).take(self.height) {
                for (item_index, _) in line
                    .into_iter()
                    .enumerate()
                    .skip(1)
                    .take(self.width)
                    .filter(|(_, item)| **item)
                {
                    let top_left = self.grid[line_index - 1][item_index - 1];
                    let top_middle = self.grid[line_index - 1][item_index];
                    let top_right = self.grid[line_index - 1][item_index + 1];

                    let left = self.grid[line_index][item_index - 1];
                    let right = self.grid[line_index][item_index + 1];

                    let bottom_left = self.grid[line_index + 1][item_index - 1];
                    let bottom_middle = self.grid[line_index + 1][item_index];
                    let bottom_right = self.grid[line_index + 1][item_index + 1];
                    // oh man

                    let adjacent = [
                        top_left,
                        top_middle,
                        top_right,
                        left,
                        right,
                        bottom_left,
                        bottom_middle,
                        bottom_right,
                    ];
                    if adjacent.iter().filter(|item| **item).count() < 4 {
                        number += 1;
                        line_i_to_remove.push(line_index);
                        item_i_to_remove.push(item_index);
                    }
                }
            }

            if line_i_to_remove.is_empty() {
                break;
            }

            for (line_i, item_i) in line_i_to_remove
                .into_iter()
                .zip(item_i_to_remove.into_iter())
            {
                self.grid[line_i][item_i] = false;
            }
        }

        number
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("day-four/input.txt")?;
    let mut str_buf = String::new();
    file.read_to_string(&mut str_buf)?;

    let mut paper_roll_grid = PaperRollGrid::new(str_buf);
    // Change `_p2` to `_p1` for part one
    let number_accessable_rolls = paper_roll_grid.num_forklift_can_access_p2();

    println!(
        "The forklift can access {} paper rolls.",
        number_accessable_rolls
    );
    Ok(())
}
