use std::{fs, num::ParseIntError};

fn main() {
    part1("input_tuto.txt");
    part2("input_tuto.txt");
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

fn part2(file_path : &str) -> i32{
    println!("Day 2 - Part 2");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_score = 0;
    
    for line_indexed in contents.lines().enumerate() {
        //println!("{}", line_indexed.1);
        let mut numbers : Vec<i32> = Vec::new();
        //println!("line {}, star indices {:?}", line_indexed.0, get_star_indices(contents.lines().nth(line_indexed.0).unwrap().to_string()));
        for idx in get_star_indices(contents.lines().nth(line_indexed.0).unwrap().to_string()){
            //look above
            if line_indexed.0 > 0 {
                let o_line = contents.lines().nth(line_indexed.0-1);
                for number in get_numbers_around(o_line.unwrap(), idx) {
                    numbers.push(number);
                }
            }
            for number in get_numbers_around(line_indexed.1, idx) {
                numbers.push(number);
            }
            if !line_indexed.1.eq(contents.lines().last().unwrap()) {
                //look below
                let o_line = contents.lines().nth(line_indexed.0+1);
                //println!("get_numbers_around({}, {idx})", o_line.unwrap());
                for number in get_numbers_around(o_line.unwrap(), idx) {
                    numbers.push(number);
                }
            }
            if numbers.len() < 2 {
                numbers.clear();
                continue;
            }
            //println!("{} * {}", numbers.first().unwrap(), numbers.last().unwrap());
            total_score += numbers.first().unwrap() * numbers.last().unwrap();
            numbers.clear();
        }

        //total_score += String::from(id).parse::<i32>().unwrap();
    }
    println!("Total score: {}", total_score);
    return total_score;
}

#[derive(PartialEq)]
struct PartNumber {
    column : usize,
    value: i32,
}

fn get_number_around(line : &str, index: usize) -> Result<PartNumber, &'static str> {
    let mut start = index-1;
    let mut end = index;
    while start >= 0 {
        //println!("start {start}");
        if line.chars().nth(start).unwrap().is_digit(10){
            if start > 0 {
                start = start - 1;
            }else{
                break;
            }
        }else{
            start = start + 1;
            break;
        }
    }
    while end < line.len() {
        if line.chars().nth(end).unwrap().is_digit(10){
            end = end + 1;
        }else{
            break;
        }
    }
    //println!("{}..{} is -{}-", start, end, line[start..end].to_string());
    let value = line[start..end].parse::<i32>();
    if value.is_err() {
        return Err("");
    }
    let pn = PartNumber { column:start, value: value.unwrap_or(0)};
    return Ok(pn);
}

fn get_numbers_around(line : &str, index: usize) -> Vec<i32>{
    let mut numbers = Vec::new();
    if index > 0 {
        let get_number_around = get_number_around(line, index);
        if get_number_around.is_ok() {
            numbers.push(get_number_around.unwrap());
        }
    }
    if index < line.len() {
        let get_number_around = get_number_around(line, index+1);
        if get_number_around.is_ok() {
            if !numbers.contains(get_number_around.as_ref().unwrap()) {
                numbers.push(get_number_around.unwrap());
            }
        }
    }
    //println!("line {} - index {} - numbers {:?}", line, index, numbers);
    return numbers.iter().map(|pn| pn.value).collect();
}

fn get_star_indices (line : String) -> Vec<usize> {
    return line.chars().into_iter()
        .enumerate()
        .filter(|(_, c)| c.to_string() == "*")
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
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

#[test]
fn all_indexes() {
    let test = "....*...*..*";
    let indices = get_star_indices(test.to_string());
    dbg!(indices); // 0, 3, 4
}

#[test]
fn number_around() {
    assert_eq!(get_number_around("467..114..", 2).unwrap().value, 467);
    assert_eq!(get_number_around("..35..633.", 6).unwrap().value, 633);
    assert_eq!(get_number_around("..35..633", 6).unwrap().value, 633);
    assert_eq!(get_number_around("35..633.", 1).unwrap().value, 35);
}

#[test]
fn part2_tuto(){
    assert_eq!(part2("input_tuto.txt"), 467835);
}

#[test]
fn part2_exrtact(){
    assert_eq!(part2("input_extract.txt"), 124*246);
}

#[test]
fn part2_puzzle(){
    assert_eq!(part2("input_puzzle.txt"), 81166799);
}

#[test]
fn part2_both_below(){
    let numbers = get_numbers_around(".883.561..*....428.........../14...742...........@.....654.....809../716.......*456.....=....*........$..............................607....", 4);
    println!("{:?}", numbers);
    assert_eq!(numbers.len(), 2);
}

#[test]
fn part2_both_duplicate(){
    let numbers = get_numbers_around("...............874...129.......................739*971.......*.......176.............3.@...........*..219..40..........#.............168....", 70);
    println!("{:?}", numbers);
    assert_eq!(numbers.len(), 1);
}

#[test]
fn test_contains(){
    let mut numbers = Vec::new();
    let part_number = PartNumber {
        column : 1,
        value : 664
    };
    numbers.push(part_number);
    let part_number2 = PartNumber {
        column : 1,
        value : 664
    };
    assert_eq!(numbers.contains(&part_number2), true);
}