use std::fs;
use std::collections::HashMap;

fn main() {

    compute_number("input.txt".to_owned());

    compute_number_part2("input_puzzle.txt".to_owned());

}

fn compute_number(file_path : String) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_score = 0;
    
    for line in contents.split('\n') {
        let first = find_first_number(line.to_string());
        let second = find_last_number(line.to_string());
        let mut number = String::from(first);
        number.push(second);
        println!("Adding {number}");
        total_score += number.parse::<i32>().unwrap();
    }

    println!("Total: {total_score}");
    return total_score;
}

fn find_first_number(input : String) -> char {
    for idx in 0..input.len(){
        if input.chars().nth(idx).unwrap().is_digit(10) {
            return input.chars().nth(idx).unwrap();
        }
    }
    return 'a';
}

fn find_last_number(input : String) -> char {
    for idx in (0..input.len()).rev(){
        if input.chars().nth(idx).unwrap().is_digit(10) {
            return input.chars().nth(idx).unwrap();
        }
    }
    return 'a';
}

///////////////////////////////////////////////////////////////////////////////
/// PART 2  ///////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////

fn compute_number_part2(file_path : String) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_score = 0;
    
    for line in contents.split('\n') {
        let first = find_first_number_in_word(line.to_string());
        let second = find_last_number_in_word(line.to_string());
        let mut number = String::from(first);
        number.push(second);
        println!("Adding {number}");
        total_score += number.parse::<i32>().unwrap();
    }

    println!("Total: {total_score}");
    return total_score;
}

fn find_first_number_in_word(input : String) -> char {
    for idx in 0..input.len(){
        if input.chars().nth(idx).unwrap().is_digit(10) {
            return input.chars().nth(idx).unwrap();
        }
        let slice = &input[..idx+1];
        //println!("Slice is {slice}");
        let replace_words = replace_words(slice.to_string());
        if replace_words.is_digit(10) {
            return replace_words;
        }
    }
    return 'a';
}

fn find_last_number_in_word(input : String) -> char {
    for idx in (0..input.len()).rev(){
        if input.chars().nth(idx).unwrap().is_digit(10) {
            return input.chars().nth(idx).unwrap();
        }
        let slice = &input[idx..];
        //println!("Slice is {slice}");
        let replace_words = replace_words(slice.to_string());
        if replace_words.is_digit(10) {
            return replace_words;
        }
    }
    return 'a';
}

fn replace_words(input : String) -> char{
    let mut word_to_digit = HashMap::new();

    // Review some books.
    word_to_digit.insert("one".to_string(), "1".to_string());
    word_to_digit.insert("two".to_string(), "2".to_string());
    word_to_digit.insert("three".to_string(), "3".to_string());
    word_to_digit.insert("four".to_string(), "4".to_string());
    word_to_digit.insert("five".to_string(), "5".to_string());
    word_to_digit.insert("six".to_string(), "6".to_string());
    word_to_digit.insert("seven".to_string(), "7".to_string());
    word_to_digit.insert("eight".to_string(), "8".to_string());
    word_to_digit.insert("nine".to_string(), "9".to_string());

    for word in word_to_digit.keys(){
        if input.contains(word) {
            return word_to_digit.get(word).unwrap().chars().next().unwrap();
        }
    }

    return 'a';
}

#[cfg(test)]
mod tests_part1 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use crate::compute_number;
    use crate::compute_number_part2;
    use crate::find_last_number;
    use crate::find_first_number;
    use crate::find_first_number_in_word;
    use crate::find_last_number_in_word;

    #[test]
    fn find_last_number_test() {
        assert_eq!(find_last_number(String::from("a2bnc1e")), '1');
        assert_eq!(find_last_number(String::from("a2bnc1")), '1');
    }

    #[test]
    fn find_first_number_test() {
        assert_eq!(find_first_number(String::from("a2bnc1e")), '2');
        assert_eq!(find_first_number(String::from("2bnc1")), '2');
    }

    #[test]
    fn part1_intro() {
        assert_eq!(compute_number("input.txt".to_owned()), 142);
    }

    #[test]
    fn part1_puzzle() {
        assert_eq!(compute_number("input_puzzle.txt".to_owned()), 54159);
    }

    #[test]
    fn part2_unit1() {
        assert_eq!(find_first_number_in_word(String::from("a1bcgdf2")), '1');
        assert_eq!(find_first_number_in_word(String::from("one2bcgdf2")), '1');
    }

    #[test]
    fn part2_unit2() {
        assert_eq!(find_last_number_in_word(String::from("two1nine")), '9');
        assert_eq!(find_last_number_in_word(String::from("7pqrstsixteen")), '6');
        assert_eq!(find_last_number_in_word(String::from("7twone")), '1');
    }

    #[test]
    fn part2_intro() {
        assert_eq!(compute_number_part2("input_part2.txt".to_owned()), 281);
    }

    #[test]
    fn part2_puzzle() {
        assert_eq!(compute_number_part2("input_puzzle.txt".to_owned()), 281);
    }
}
