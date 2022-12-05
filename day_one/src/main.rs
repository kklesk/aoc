use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let elve_list = match File::open("elf_list.txt") {
        Ok(file) => file,
        Err(_) => panic!("unable to read file"),
    };
    let mut calories = 0;
    let mut max_calories = 0;
    let mut max_calories_top_three = Vec::new();
    let mut elve_id = 0;
    let reader =  BufReader::new(elve_list);

    for line in reader.lines(){
        match line.as_ref().unwrap().parse::<i32>() {
            Ok(carried_calories) => {
                // calories = calories + line.unwrap().parse::<i32>().unwrap();
                calories = calories + carried_calories;
                elve_id = elve_id + 1;
                max_calories_top_three.push(calories);
            },
            Err(_) => {
                if max_calories <= calories{
                    max_calories = calories;
                }
                calories = 0;
            }
        };
    }
    max_calories_top_three.sort_by(|a,b| b.cmp(a));
    println!("elves_id {} has {} calories", elve_id, max_calories);
    println!("top three carried calories {}", (max_calories_top_three[0]+max_calories_top_three[1]+max_calories_top_three[2]));
}
