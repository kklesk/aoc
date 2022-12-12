use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let rucksack_file = File::open("rucksack_list.txt").expect("error while opening file");
    let buff_reader = BufReader::new(rucksack_file);

    for (rucksack_item_one, rucksack_item_two) in buff_reader
        .lines()
        .map(|rucksack_parts| -> (String, String) {
            let len = match &rucksack_parts {
                Ok(parts) => parts.len(),
                Err(_) => panic!("error while get string len"),
            };
            let (item_one, item_two) = match &rucksack_parts {
                Ok(parts) => parts.split_at((len / 2)),
                Err(_) => panic!("error while splitting rucksack compartments"),
            };
            if item_two.contains(item_one){

            }
            //FIXME return Vec<String>, Vec<String>
            (item_one.to_string(), item_two.to_string())
            // println!("items {:?}",item_one);
            // println!("items {:?}",item_two);
        })
    {
        let mut found_char: Vec<char> = Vec::new();
        for item_a in rucksack_item_one.chars() {
            if rucksack_item_two.contains(item_a) {
                println!("{}", item_a);
                found_char.push(item_a);
            }
        }
        //debug print
        println!("{:?}", found_char.iter());
    }
}