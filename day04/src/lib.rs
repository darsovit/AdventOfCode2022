pub struct Day04<'a> {
    elf_pairs: Vec::<&'a str>,
}

impl<'a> Day04<'a> {
    pub fn new(lines: std::str::Lines<'a>) -> Self {
        Day04{elf_pairs: lines.collect()}
    }

    fn parse_range(range: &str) -> (u32, u32) {
        let mut range_iter = range.split('-').map(&str::parse::<u32>);
        match (range_iter.next(), range_iter.next()) {
            (Some(Ok(small)), Some(Ok(large))) => { 
                (small, large)
            },
            _ => { panic!("Failed to split and parse range: {range}"); }
        }
    }

    fn range_contains(big: (u32, u32), small: (u32, u32)) -> bool {
        (big.0 <= small.0) && (big.1 >= small.1)
    }

    pub fn part1(&self) -> u32 {
        let mut sum_of_values = 0;
        for elf_pair in &self.elf_pairs {
            let mut elf_iter = elf_pair.split(',');
            match (elf_iter.next(), elf_iter.next()) {
                (Some(left), Some(right)) => {
                    let left_range = Self::parse_range(left);
                    let right_range = Self::parse_range(right);

                    match (Self::range_contains(left_range, right_range), Self::range_contains(right_range, left_range)) {
                        (true, _) => { sum_of_values += 1; },
                        (false, true) => { sum_of_values += 1; },
                        (false, false) => {}
                    }
                },
                _ => { panic!("Broken line in {elf_pair}"); }
            }
        }
        sum_of_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str =
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_sample_results_in_2() {
        let day = Day04::new(SAMPLE_INPUT.lines());
        assert_eq!(2, day.part1());
    }
}