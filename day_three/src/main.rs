use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let rucksack_file = File::open("rucksack_list.txt").expect("error while opening file");
    let buff_reader = BufReader::new(rucksack_file);

    for (rucksack_item_one,rucksack_item_two) in buff_reader
        .lines()
        .map(| rucksack_parts| -> (String,String) {
            let len = rucksack_parts.as_ref().unwrap().len();
            let (item_one,item_two) =  match &rucksack_parts {
                // Ok(parts) => parts.chars().collect::<&str>().split_at((len/2)),
                Ok(parts) => parts.split_at((len/2)),
                Err(_) => panic!("error while splitting rucksack compartments"),
            };
        (item_one.to_string(),item_two.to_string())
        // println!("items {:?}",item_one);
        // println!("items {:?}",item_two);
        })
    {

        // println!("{}",rucksack_item_one);
        println!("{:?} {:?}",rucksack_item_one,rucksack_item_two);
        // println!("{:?}",rucksack);
    }

}
