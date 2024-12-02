use std::fs;

use day04::Day04;

fn main(){
    let datafile = "input.txt";
    let file_content = fs::read_to_string(datafile);

    match file_content {
        Ok(linestring) => {
            {
                let day = Day04::new(linestring.lines());
                println!("part1: {}", day.part1());
                println!("part2: {}", day.part2());
            }            
        },
        Err(e) => {
            println!("Error reading file: {}, {:?}", datafile, e);
        }
    }
}