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
            //symbol one line above ?
            if line_indexed.0 > 0 {
                o_line = contents.lines().nth(line_indexed.0-1);
                if o_line.is_some() && adjacent_to_symbol_above(&part_number, o_line.unwrap().to_string()){
                    //print!("{},", part_number.value);
                    total_score += part_number.value;
                    continue;
                }
            }

            //symbol on same line ?
            if adjacent_to_symbol_above(&part_number, line_indexed.1.to_string()) {
                //print!("{},", part_number.value);
                total_score += part_number.value;
                continue;
            }

            //symbole one line below ?
            if line_indexed.1.to_string() != contents.lines().last().unwrap().to_string() {
                o_line = contents.lines().nth(line_indexed.0+1);
                if o_line.is_some() && adjacent_to_symbol_above(&part_number, o_line.unwrap().to_string()){
                    //print!("{},", part_number.value);
                    total_score += part_number.value;
                    continue;
                }
            }
            //println!("Number {}, column {} not adjacent", part_number.value, part_number.column);
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

    if !tmp.is_empty() {
        let part_number = PartNumber {
            column : input.to_string().len() - tmp.len(),
            value : tmp.parse::<i32>().unwrap()
        };
        numbers.push(part_number);
    }

    return numbers;
}

fn adjacent_to_symbol_above(current_part_ptr : &PartNumber, above_line : String) -> bool {
    let range_min = get_range_min(current_part_ptr);
    let range_max = get_range_max(above_line.len(), current_part_ptr);
    

    for idx in range_min..range_max {
        let ch = above_line.chars().nth(idx).unwrap_or('.');
        if ch != '.' && !ch.is_digit(10) {
            return true;
        }
    }
    return false;
}

fn get_range_max(line_len : usize, part_number : &PartNumber) -> usize {
    return std::cmp::min(line_len, part_number.column + part_number.value.to_string().len() + 1);
}

fn get_range_min(part_number : &PartNumber) -> usize {
    if part_number.column > 0 {
        return part_number.column - 1;
    } 
    return 0;
}

#[test]
fn extract_numbers(){
    let line  = "467....114";
    let part_numbers = read_numbers(line);
    assert_eq!(part_numbers.len(), 2);
    assert_eq!(part_numbers.first().unwrap().value, 467);
    assert_eq!(part_numbers.first().unwrap().column, 0);
    assert_eq!(part_numbers.last().unwrap().value, 114);
    assert_eq!(part_numbers.last().unwrap().column, 7);
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

#[test]
fn part1_puzzle(){
    assert_eq!(part1("input_puzzle.txt"), 549908);
}