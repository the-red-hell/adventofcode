use std::{fs, iter::Iterator};

fn main() {
    let path = "day-one/puzzle_input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    // let sum1 = denoise_and_get_sum1(content.clone());
    let sum2 = denoise_and_get_sum2(content);

    // println!("p1: {sum1}");
    println!("p2: {sum2}");
}

// p1
fn denoise_and_get_sum1(content: String) -> usize {
    let mut values = Vec::new();
    for line in content.lines() {
        let all_numbers: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();
        let first = all_numbers.first().unwrap();
        let last = all_numbers.last().unwrap();

        values.push(format!("{first}{last}"))
    }

    let sum = values.iter().map(|x| x.parse::<usize>().unwrap()).sum();

    return sum;
}

// p2 (not from me :( )
fn denoise_and_get_sum2(mut content: String) -> usize {
    for (i, number) in vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    {
        content = content.replace(
            number,
            &format!("{}{}{}", number, (i + 1).to_string(), number), // this is were I have failed
        );
    }
    let mut numbers: Vec<usize> = vec![];

    for line in content.lines() {
        let numbers_in_line: Vec<usize> = line
            .chars()
            .filter(|x| x.is_numeric())
            .map(|x| x.to_string().parse().unwrap())
            .collect();
        numbers.push(numbers_in_line.first().unwrap() * 10 + numbers_in_line.last().unwrap())
    }
    numbers.iter().sum()
}

// // p2
// fn denoise_and_get_sum2(content: String) -> usize {
//     let letter_numbers: Vec<String> = vec![
//         "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
//     ]
//     .iter()
//     .map(|x| x.to_string())
//     .collect();
//     // let mut values = Vec::new();

//     for line in content.lines() {
//         let how_much_till_number_front = line.chars().position(char::is_numeric).unwrap_or(5);
//         let how_much_till_number_back = line.chars().rev().position(char::is_numeric).unwrap_or(5);
//         let mut number_front: usize = 0;
//         let mut number_back: usize = 0;
//         for (index, number_letter) in letter_numbers.iter().enumerate() {
//             if let Some(_) = line
//                 .chars()
//                 .take(how_much_till_number_front)
//                 .collect::<String>()
//                 .find(number_letter)
//             {
//                 if number_front < index + 1 {
//                     number_front = index + 1;
//                 }
//             }
//         }
//         for (index, number_letter) in letter_numbers.iter().enumerate() {
//             if let Some(_) = line
//                 .chars()
//                 .rev()
//                 .take(how_much_till_number_back)
//                 .collect::<Vec<char>>()
//                 .iter()
//                 .rev()
//                 .map(|x| x.to_owned())
//                 .collect::<String>()
//                 .find(number_letter)
//             {
//                 if number_back < index + 1 {
//                     number_back = index + 1;
//                 }
//             }
//         }

//         if number_front == 0 {
//             let first = line.find(char::is_numeric).unwrap();
//             number_front = first;
//         }
//         if number_back == 0 {
//             let last = line.find(char::is_numeric).unwrap();
//             number_back = last
//         }

//         dbg!(&number_front, number_back);

//         // let all_numbers: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();
//         // let first = all_numbers.first().unwrap();
//         // let last = all_numbers.last().unwrap();

//         // values.push(format!("{first}{last}"))
//     }

//     // let sum = values.iter().map(|x| x.parse::<usize>().unwrap()).sum();

//     return 0;
// }

// // p2
// fn denoise_and_get_sum2(content: String) -> usize {
//     let letter_numbers: Vec<String> = vec![
//         "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
//     ]
//     .iter()
//     .rev()
//     .map(|x| x.to_string())
//     .collect();

//     let mut values = Vec::new();
//     for line in content.lines() {
//         // let mut line = line.to_string();

//         let mut new_lines = String::new();

//         let iterator = line.chars().collect::<Vec<char>>();
//         let mut iterator = iterator.windows(5).into_iter();
//         while let Some(word) = iterator.next() {
//             let word: String = word.iter().collect();
//             let mut biggest_num = String::new();
//             letter_numbers.iter().enumerate().for_each(|(index, x)| {
//                 if let Some(_) = word.find(x) {
//                     biggest_num = (9 - index).to_string();
//                 }
//             });
//             if !biggest_num.is_empty() {
//                 iterator.nth(biggest_num.parse::<usize>().unwrap() - 1);
//             }
//             dbg!(&biggest_num, &word);
//             // if let Some(index) = word.
//         }

//         // letter_numbers
//         //     .windows(5)
//         //     .enumerate()
//         //     .for_each(|(index, &num_name)| {
//         //         line = line.clone().replace(num_name, &(index + 1).to_string())
//         //     });

//         dbg!(&line);
//         let all_numbers: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();

//         let first = all_numbers.first().unwrap();
//         let last = all_numbers.last().unwrap();

//         values.push(format!("{first}{last}"))
//     }

//     dbg!(&values);
//     let sum = values.iter().map(|x| x.parse::<usize>().unwrap()).sum();

//     return sum;
// }
