use std::fs;

fn main() {
    let path = "day-three/puzzle_input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let sum1 = sum_part_numbers(content.clone());
    // let sum2 = get_sum_of_power_mins(content);

    println!("p1: {sum1}");
    // println!("p2: {sum2}");
}

fn is_not_numeric(char: char) -> bool {
    match char {
        '0'..='9' => false,
        _ => true,
    }
}

fn sum_part_numbers(content: String) -> usize {
    let length = content.lines().next().unwrap().len();
    let content = content.replace('\n', "");

    for (index, number) in content
        .split(is_not_numeric)
        .enumerate()
        .filter(|(_, s)| !s.chars().all(|c| c.is_whitespace()))
    {
        println!("{} {}", index, number)
    }
    unimplemented!()
}

// fn check_condition(content: &str, index_to_check: isize) -> bool {
//     if index_to_check < 0 {
//         return false;
//     }
//     let index_to_check = index_to_check as usize;
//     let character = content.chars().nth(index_to_check).unwrap_or('h');
//     if character != '.' && !character.is_numeric() {
//         return true;
//     }
//     false
// }

// // fn get_index(length: usize, cur_index: usize, next_width: isize, next_height: isize) -> usize {
// //     let new_index = (cur_index as isize + next_width) / length as isize + next_height;
// //     if new_index < 0 {
// //         return cur_index;
// //     }
// //     println!("\nnop: {}", new_index);
// //     new_index as usize
// // }

// fn sum_part_numbers(content: String) -> usize {
//     let length = content.lines().next().unwrap().len();
//     let content = content.replace('\n', "");

//     let mut numbers: Vec<&str> = Vec::new();
//     let mut num_index: usize = 0;

//     for (index, character) in content.chars().enumerate() {
//         if character.is_numeric() {
//             // check conditions
//             if check_condition(&content, (index + 1) as isize) // right
//                 || check_condition(&content, index as isize - 1) // left
//                 || check_condition(&content, (index + length) as isize) // bottom
//                 || check_condition(&content, (index + length + 1) as isize) // bottom right
//                 || check_condition(&content, (index + length) as isize - 1) // bottom left
//                 || check_condition(&content, index as isize - length as isize) // top
//                 || check_condition(&content, index as isize - length as isize + 1) // top right
//                 || check_condition(&content, index as isize - length as isize - 1)
//             // top left
//             {
//                 println!("{} {}", character, index);
//             }
//         }
//         // println!("{}", character);
//     }

//     for (index, character) in content.chars().enumerate() {
//         let line = index / length;
//         let index = index % length;
//         if character == '.' || character.is_numeric() {
//             continue;
//         } else if character.is_ascii() {
//             println!("{}", character);
//             dbg!(line, index);
//         }
//     }
//     println!("{}", length);
//     unimplemented!()
// }
