use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let elve_list = match File::open("elf_list.txt") {
        Ok(file) => file,
        Err(_) => panic!("unable to read file"),
    };
    let mut calories = 0;
    let mut max_calories = 0;
    let mut elve_id = 0;
    let reader =  BufReader::new(elve_list);

    for line in reader.lines(){
        match line.as_ref().unwrap().parse::<i32>() {
            Ok(carried_calories) => {
                // calories = calories + line.unwrap().parse::<i32>().unwrap();
                calories = calories + carried_calories;
                elve_id = elve_id + 1;
            },
            Err(_) => {
                if max_calories <= calories{
                    max_calories = calories;
                }
                calories = 0;
            }
        };
    }
    println!("elves_id {} has {} calories", elve_id, max_calories);
}
