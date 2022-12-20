use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let rucksack_file = File::open("rucksack_list.txt").expect("error while opening file");
    let buff_reader = BufReader::new(rucksack_file);
    let mut res_part_one: i32 = 0;

    for (rucksack_item_one, rucksack_item_two) in buff_reader
        .lines()
        .map(|rucksack_parts| -> (String, String) {
            let len = match &rucksack_parts {
                Ok(parts) => parts.len(),
                Err(_) => panic!("error while get string len"),
            };
            let (item_one, item_two) = match &rucksack_parts {
                Ok(parts) => parts.split_at(len / 2),
                Err(_) => panic!("error while splitting rucksack compartments"),
            };
            (item_one.to_string(), item_two.to_string())
        })
    {
        for item_a in rucksack_item_one.chars() {
            if rucksack_item_two.contains(item_a) {
                    if item_a.is_lowercase(){
                        res_part_one = res_part_one + item_a as i32 - 'a' as i32 + 1;
                        //take the first element ¯\_(ツ)_/¯ ugly
                        break;
                    }else {
                        res_part_one = res_part_one + item_a as i32 - 'A' as i32 + 27 ;
                        //take the first element  ¯\_(ツ)_/¯ ugly
                        break;
                    }
            }
        }
    }
    {
            println!("part_one: {}",res_part_one);
    }
}
