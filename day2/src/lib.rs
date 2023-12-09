//#![feature(test)]

use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn part1(file_path : &str) -> i32{
    println!("Day 2 - Part 1");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_score = 0;
    let bag1 = "12 red";
    let bag2 = "13 green";
    let bag3 = "14 blue";

    for line in contents.split("\r\n") {
        let revealed = get_revealed(line);
        let mut game_possible = true;
        for out in revealed.split(", ") {
            if !possible(out, bag1){
                //println!("not possible1 '{}' - '{}'", out, bag1);
                game_possible = false;
                break;
            }
            if !possible(out, bag2){
                //println!("not possible2 '{}' - '{}'", out, bag2);
                game_possible = false;
                break;
            }
            if !possible(out, bag3){
                //println!("not possible3 '{}' - '{}'", out, bag3);
                game_possible = false;
                break;
            }
        }
        //println!("Game {}: {}", get_game_id(line), game_possible);
        if game_possible {
            let id = get_game_id(line);
            total_score += String::from(id).parse::<i32>().unwrap();
            //println!("Total score: {}", total_score);
        }
    }
    println!("Total score: {}", total_score);
    return total_score;
}

fn get_revealed(input : &str) -> String {
    return input.split(": ")
    .nth(1)
    .unwrap()
    .replace(";", ",");
}

fn get_game_id(input : &str) -> &str {
    return input.split(':')
    .next()
    .unwrap()
    .split(' ')
    .nth(1)
    .unwrap();
}

fn extract_number(input : &str) -> &str{
    return input.split(' ')
    .next()
    .unwrap();
}

fn extract_color(input : &str) -> &str{
    return input.split(' ')
    .nth(1)
    .unwrap();
}

pub fn part2(file_path : &str) -> i32{
    println!("Day 2 - Part 2");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_score = 0;

    for line in contents.split("\r\n") {
        let mut max_red = String::from("0 red");
        let mut max_green = String::from("0 green");
        let mut max_blue = String::from("0 blue");
        let revealed = get_revealed(line);
        for out in revealed.split(", ") {
            max_red = get_max(max_red, out.to_string());
            max_green = get_max(max_green, out.to_string());
            max_blue = get_max(max_blue, out.to_string());
        }
        //println!("Game {}: {}", get_game_id(line), game_possible);
        println!("Game {}, {}, {}, {}", get_game_id(line), max_red, max_green, max_blue);
        total_score += extract_number(&max_red).parse::<i32>().unwrap() * extract_number(&max_green).parse::<i32>().unwrap() * extract_number(&max_blue).parse::<i32>().unwrap();
        //println!("Total score: {}", total_score);
    }
    println!("Total score: {}", total_score);
    return total_score;
}

fn get_max(input1 : String, input2 : String) -> String{
    if !extract_color(&input1).eq(extract_color(&input2)) {
        return input1;
    }
    if extract_number(&input1).parse::<i32>().unwrap() > extract_number(&input2).parse::<i32>().unwrap() {
        //println!("{} > {}", input, input2);
        return input1;
    }
    return input2;
}

/**
 * returns true if color is not the same or if number1 < number2
 */
fn possible(input : &str, input2 : &str) -> bool{
    if !extract_color(input).eq(extract_color(input2)) {
        return true;
    }
    if extract_number(input).parse::<i32>().unwrap() > extract_number(input2).parse::<i32>().unwrap() {
        //println!("{} > {}", input, input2);
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    //extern crate test;
    //use test::Bencher;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_game_id(){
        assert_eq!(get_game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), "1");
    }

    #[test]
    fn test_extract_color(){
        assert_eq!(extract_color("3 blue"), "blue");
    }

    #[test]
    fn test_extract_number(){
        assert_eq!(extract_number("3 blue"), "3");
    }

    #[test]
    fn test_possible(){
        assert_eq!(possible("20 red", "12 red"), false);
        assert_eq!(possible("12 red", "20 red"), true);
        assert_eq!(possible("20 red", "12 blue"), true);
    }

    #[test]
    fn test_sanitize(){
        assert_eq!(get_revealed("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), "3 blue, 4 red, 1 red, 2 green, 6 blue, 2 green");
    }

    #[test]
    fn test_tuto(){
        assert_eq!(part1("input_tuto.txt"), 8);
    }

    
    /*#[bench]
    fn test_part1(b: &mut Bencher){
        b.iter(|| {
            part1_bench()
        });
        assert_eq!(part1("input_puzzle.txt"), 2061);
    }
    fn part1_bench(){
        part1("input_puzzle.txt");
    }*/

    #[test]
    fn test_tuto2(){
        assert_eq!(part2("input_tuto.txt"), 2286);
    }

    #[test]
    fn test_part2(){
        assert_eq!(part2("input_puzzle.txt"), 72596);
    }

}
