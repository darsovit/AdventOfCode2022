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

    fn get_pair_ranges(range_pair: &str) -> ((u32, u32), (u32, u32)) {
        let mut range_iter = range_pair.split(',');
        match (range_iter.next(), range_iter.next()) {
            (Some(left), Some(right)) => {
                let left_range = Self::parse_range(left);
                let right_range = Self::parse_range(right);
                (left_range, right_range)
            },
            _ => { panic!("Broken line in {range_pair}"); }
        }
    }
    
    fn determine_if_full_overlap(left: (u32, u32), right: (u32, u32)) -> bool {
        match (Self::range_contains(left, right), Self::range_contains(right, left)) {
            (true, _) => true,
            (false, true) => true,
            (false, false) => false
        }
    }
    pub fn part1(&self) -> u32 {
        let mut sum_of_values = 0;
        for elf_pair in &self.elf_pairs {
            let (left, right) = Self::get_pair_ranges(elf_pair);
            if Self::determine_if_full_overlap(left, right) {
                sum_of_values += 1;
            }
        }
        sum_of_values
    }

    fn determine_partial_overlap(left: (u32, u32), right: (u32, u32)) -> bool {
        if left.0 <= right.0 && left.1 >= right.0 {
            true
        } else if left.0 <= right.1 && left.1 >= right.1 {
            true
        } else if Self::determine_if_full_overlap(left, right) {
            true
        } else {
            println!("left: {:?}, right: {:?}", left, right);
            false
        }
    }

    pub fn part2(&self) -> u32 {
        let mut sum_of_values = 0;
        for elf_pair in &self.elf_pairs {
            let (left, right) = Self::get_pair_ranges(elf_pair);
            if Self::determine_partial_overlap(left, right) {
                sum_of_values += 1;
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

    #[test]
    fn part2_sample_results_in_4() {
        let day = Day04::new(SAMPLE_INPUT.lines());
        assert_eq!(4, day.part2());
    }
}