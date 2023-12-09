use std::fs;

fn main() {
    part1("input_tuto.txt");
}

fn part1(file_path : &str) -> i32{
    println!("Day 2 - Part 1");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_score = 0;
    
    for line_indexed in contents.lines().enumerate() {
        let part_numbers = read_numbers(line_indexed.1);
        for part_number in part_numbers {
            let mut o_line;
            if line_indexed.0 > 0 {
                o_line = contents.lines().nth(line_indexed.0-1);
                if o_line.is_some() && adjacent_to_symbol_above(&part_number, o_line.unwrap().to_string()){
                    println!("Adding {}", part_number.value);
                    total_score += part_number.value;
                    continue;
                }
            }

            if line_indexed.0 > 0 {
                let ch = line_indexed.1.chars().nth(line_indexed.0-1).unwrap_or('.');
                if ch != '.' && !ch.is_digit(10) {
                    println!("Adding {}", part_number.value);
                    total_score += part_number.value;
                    continue;
                }
            }

            if line_indexed.0 < line_indexed.1.len()-1 {
                let ch = line_indexed.1.chars().nth(line_indexed.0+1).unwrap_or('.');
                if ch != '.' && !ch.is_digit(10) {
                    println!("Adding {}", part_number.value);
                    total_score += part_number.value;
                    continue;
                }
            }

            if line_indexed.1.to_string() != contents.lines().last().unwrap().to_string() {
                o_line = contents.lines().nth(line_indexed.0+1);
                if o_line.is_some() && adjacent_to_symbol_above(&part_number, o_line.unwrap().to_string()){
                    println!("Adding {}", part_number.value);
                    total_score += part_number.value;
                    continue;
                }
            }
        }

        //total_score += String::from(id).parse::<i32>().unwrap();
    }
    println!("Total score: {}", total_score);
    return total_score;
}

struct PartNumber {
    column : usize,
    value: i32,
}

fn read_numbers( input : &str) -> Vec<PartNumber> {
    let mut numbers = Vec::new();
    let mut tmp = String::from("");

    for idx in 0..input.to_string().len() {
        let char = input.chars().nth(idx);
        if char.unwrap().is_digit(10) {
            tmp.push(char.unwrap());
        }else if !tmp.is_empty() {
            let part_number = PartNumber {
                column : idx - tmp.len(),
                value : tmp.parse::<i32>().unwrap()
            };
            numbers.push(part_number);
            tmp = String::from("");
        }
    }

    return numbers;
}

fn adjacent_to_symbol_above(current_part_ptr : &PartNumber, above_line : String) -> bool {
    let current_part = current_part_ptr.to_owned();
    let range_min;
    if current_part.column > 0 {
        range_min = current_part.column-1;
    }else{
        range_min = 0;
    }
    let range_max;
    if above_line.len() > current_part.value.to_string().len()+2 {
        range_max = current_part.value.to_string().len()+2;
    }else{
        range_max = above_line.len();
    }

    for idx in range_min..range_max {
        let ch = above_line.chars().nth(idx).unwrap_or('.');
        if ch != '.' && !ch.is_digit(10) {
            return true;
        }

    }
    return false;
}


#[test]
fn extract_numbers(){
    let line  = "467..114..";
    let part_numbers = read_numbers(line);
    assert_eq!(part_numbers.len(), 2);
    assert_eq!(part_numbers.first().unwrap().value, 467);
    assert_eq!(part_numbers.first().unwrap().column, 0);
    assert_eq!(part_numbers.last().unwrap().value, 114);
    assert_eq!(part_numbers.last().unwrap().column, 5);
}

#[test]
fn test_adjacent_to_symbol_above(){
    let above_line_ok  = "...$.*....";
    let part_number = PartNumber {
        column : 1,
        value : 664
    };
    assert_eq!(adjacent_to_symbol_above(&part_number, String::from(above_line_ok)), true);
}

#[test]
fn test_adjacent_to_symbol_above_ko(){
    let part_number = PartNumber {
        column : 1,
        value : 664
    };
    let above_line_ko  = ".....*....";
    assert_eq!(adjacent_to_symbol_above(&part_number, String::from(above_line_ko)), false);
}

#[test]
fn part1_tuto(){
    assert_eq!(part1("input_tuto.txt"), 4361);
}