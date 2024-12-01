pub struct Day03<'a> {
    rucksacks: Vec::<&'a str>,
}

impl<'a> Day03<'a> {
    pub fn new(lines: std::str::Lines<'a>) -> Self {
        Day03{ rucksacks: lines.collect() }
    }

    fn find_duplicate(first: &str, second: &str) -> char {
        let mut first_chars: Vec<char> = first.chars().collect();
        first_chars.sort();
        let mut second_chars: Vec<char> = second.chars().collect();
        second_chars.sort();
        while first_chars.len() > 0 && second_chars.len() > 0 {
            if first_chars.last() == second_chars.last() {
                let duplicate = first_chars.last();
                match duplicate {
                    Some(value) => { return value.clone(); },
                    _ => { panic!("Got to end of list"); }
                }
            }
            if first_chars.last() > second_chars.last() { 
                first_chars.pop(); 
            } else {
                second_chars.pop();
            }
        }
        panic!("Got to the end of a list");
    }

    fn get_priority(value: char) -> u32 {
        if value >= 'a' && value <= 'z' {
            1 + (value as u32 - 'a' as u32)
        } else {
            assert!(value >= 'A' && value <= 'Z');
            27 + (value as u32 - 'A' as u32)
        }
    }

    pub fn handle_rucksack(rucksack: &str) -> u32 {
        let compartment_size = rucksack.len() / 2;
        let first_compartment = &rucksack[0..compartment_size];
        let second_compartment = &rucksack[compartment_size..];
        let duplicate = Self::find_duplicate(&first_compartment, &second_compartment);
        Self::get_priority(duplicate)
    }
    pub fn part1(&self) -> u32 {
        let mut sum_of_priorities = 0;
        for rucksack in &self.rucksacks {
            sum_of_priorities += Self::handle_rucksack(rucksack);
        }
        sum_of_priorities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str =
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn first_part_sample_is_157() {
        let day03 = Day03::new(SAMPLE_INPUT.lines());
        assert_eq!(day03.part1(), 157);
    }

    #[test]
    fn first_sample_input_dup_is_16() {
        let line_vec: Vec<&str> = SAMPLE_INPUT.lines().collect();
        assert_eq!(16, Day03::handle_rucksack(line_vec[0]));
    }

    #[test]
    fn second_sample_input_dup_is_38() {
        let line_vec: Vec<&str> = SAMPLE_INPUT.lines().collect();
        assert_eq!(38, Day03::handle_rucksack(line_vec[1]));
    }
}