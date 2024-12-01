use std::fs;

use day03::Day03;

fn main(){
    let datafile = "input.txt";
    let file_content = fs::read_to_string(datafile);

    match file_content {
        Ok(linestring) => {
            {
                let day03: Day03 = Day03::new(linestring.lines());
                println!("part1: {}", day03.part1());
                //println!("part2: {}", day02.part2());
            }            
        },
        Err(e) => {
            println!("Error reading file: {}, {:?}", datafile, e);
        }
    }
}