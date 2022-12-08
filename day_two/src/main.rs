use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {

    let game_list = File::open("gamelist.txt").expect("could not load file");
    let reader = BufReader::new(game_list);

    //save all lines from gamelist.txt into Vec<string>
    let games: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|characters| characters.starts_with("A") || characters.starts_with("B") || characters.starts_with("C"))
        .map(|split| split.split_whitespace().collect::<String>()).collect();

    part_one(&games);
    part_two(&games);

}

fn part_two(games: &Vec<String>) {

    let mut total_score = 0;
    let mut total_counter = 0;

    for game in games.iter() {
        if game.chars().nth(1).unwrap() == 'X' {
            // println!("{}",game);
            // total_score = total_score + 12;
        } else if game.chars().nth(1).unwrap() == 'Y' {
            // println!("{}",game);
            // total_score = total_score + 15;
        } else if game.chars().nth(1).unwrap() == 'Z' {
            // println!("{}",game);
            // total_score = total_score + 18;
        } else {
            println!("errror {}", game);
        }
        // println!("{}",game);
        total_counter = total_counter + 1;
    }
    println!("#part_two total_score: {}", total_score);
    println!("#part_two total_score: {}", total_counter);

}

fn part_one(games: &Vec<String>) {
    let mut total_score = 0;
    let mut total_counter = 0;

    for game in games {
        if game == "AX" {
            total_score = total_score + 4;
        } else if game == "AY" {
            total_score = total_score + 8;
        } else if game == "AZ" {
            total_score = total_score + 3;
        } else if game == "BX" {
            total_score = total_score + 1;
        } else if game == "BY" {
            total_score = total_score + 5;
        } else if game == "BZ" {
            total_score = total_score + 9;
        } else if game == "CX" {
            total_score = total_score + 7;
        } else if game == "CY" {
            total_score = total_score + 2;
        } else if game == "CZ" {
            total_score = total_score + 6;
        } else {
            println!("error {}", game);
        }
        // println!("{}",game);
        total_counter = total_counter + 1;
    }
    println!("#part_one total_score: {}", total_score);
    println!("#part_one total_score: {}", total_counter);

}
