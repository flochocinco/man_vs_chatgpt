use std::io;

fn part1() {
    let target_counts = vec![12, 13, 14]; // Target cube counts for red, green, and blue respectively

    let games = &[
        &[&[3, 4], &[1, 2, 6], &[2]],
        &[&[1, 2], &[3, 4, 1], &[1, 1]],
        &[&[8, 6, 20], &[5, 4, 13], &[5, 1]],
        &[&[1, 3, 6], &[3, 6], &[3, 15, 14]],
        &[&[6, 1, 3], &[2, 1, 2]],
    ];

    let possible_games: Vec<usize> = games
        .iter()
        .enumerate()
        //.filter(|(_, game)| is_possible_game(game, &target_counts))
        .map(|(index, _)| index + 1)
        .collect();

    let sum_of_ids: usize = possible_games.iter().sum();

    println!("Games that would have been possible: {:?}", possible_games);
    println!("Sum of the IDs of possible games: {}", sum_of_ids);
}

fn is_possible_game(game: &Vec<impl AsRef<[(usize, usize, usize)]>>, target_counts: &Vec<usize>) -> bool {
    for set in game {
        for &(red, green, blue) in set.as_ref() {
            if red > target_counts[0] || green > target_counts[1] || blue > target_counts[2] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        part1();
    }
}
