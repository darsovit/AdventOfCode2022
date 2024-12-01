use std::fs;

use day02::Day02;

fn main(){
    let datafile = "input.txt";
    let file_content = fs::read_to_string(datafile);

    match file_content {
        Ok(linestring) => {
            {
                let day02: Day02 = Day02::new(linestring.lines());
                println!("{}", day02.part1());
            }
            /*
            {
                let mut day01 = Day01::new(linestring.lines());
                day01.part2();
            }
            */
        },
        Err(e) => {
            println!("Error reading file: {}, {:?}", datafile, e);
        }
    }
}
