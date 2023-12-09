use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Read the engine schematic from the file
    let engine_schematic = read_engine_schematic_from_file("input.txt")?;

    // Calculate the sum of part numbers
    let sum_of_part_numbers = calculate_sum_of_part_numbers(&engine_schematic);

    println!("Sum of part numbers: {}", sum_of_part_numbers);

    Ok(())
}

fn read_engine_schematic_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut engine_schematic = Vec::new();

    for line in reader.lines() {
        let row: Vec<char> = line?.chars().collect();
        engine_schematic.push(row);
    }

    Ok(engine_schematic)
}

fn calculate_sum_of_part_numbers(engine_schematic: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    for i in 0..engine_schematic.len() {
        for j in 0..engine_schematic[i].len() {
            if engine_schematic[i][j].is_digit(10) {
                let mut num_str = String::new();
                let mut col = j;

                // Collect the entire number
                while col < engine_schematic[i].len() && (engine_schematic[i][col].is_digit(10) || engine_schematic[i][col] == '.') {
                    num_str.push(engine_schematic[i][col]);
                    col += 1;
                }

                // Parse the number and add to the sum
                if let Ok(num) = num_str.parse::<usize>() {
                    sum += num;
                }
            }
        }
    }

    sum
}

#[test]
fn test_calculate_sum_of_part_numbers() {
    // Example engine schematic
    let engine_schematic = vec![
        vec!['4', '6', '7', '.', '.', '1', '1', '4', '.'],
        vec!['.', '.', '.', '*', '.', '.', '.', '.', '.'],
        vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
        vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
        vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
        vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
        vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
        vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
    ];

    // Calculate the sum of part numbers
    let sum = calculate_sum_of_part_numbers(&engine_schematic);

    assert_eq!(sum, 4361);
}