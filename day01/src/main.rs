use std::fs;

use day01::Day01;

fn main() -> std::io::Result<()> {
    let datafile = "input.txt";
    let file_content = fs::read_to_string(datafile);

    match file_content {
        Ok(linestring) => {
            {
                let day01: Day01 = Day01::new(linestring.lines());
                day01.part1();
            }
            {
                let mut day01 = Day01::new(linestring.lines());
                day01.part2();
            }
        },
        Err(e) => {
            println!("Error reading file: {}, {:?}", datafile, e);
        }
    }
     //Day01(reader);
    Ok(())
}
