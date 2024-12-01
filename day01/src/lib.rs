use std::collections::BinaryHeap;

pub struct Day01 {
    heap: BinaryHeap<u32>,
}

impl Day01 {
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut _heap = BinaryHeap::new();

        let mut sum = 0;
        for line in lines {

            match line.parse::<u32>() {
                Ok(i) => {
                    sum += i;
                },
                Err(_) => {
                    _heap.push(sum);
                    sum = 0;
                }
            }
        }

        Day01{heap: _heap}
    }

    pub fn part1(&self) {
        match self.heap.peek() {
            Some(i) => {
                println!("Largest value is {i}");
            },
            None => {
                println!("Cannot find value");
            }
        }
    }

    pub fn part2(&mut self) {
        let elf1 = self.heap.pop();
        let elf2 = self.heap.pop();
        let elf3 = self.heap.pop();
        match (elf1, elf2, elf3) {
            (Some(a), Some(b), Some(c)) => {
                println!("First three elves have: {} calories", a + b + c)
            },
            (_, _, _) => {
                println!("Failed to find three elves worth of calories");
            }
        }
    }
}
/*
pub trait linereader {
    fn lines(&self) -> 
}

fn Day01<T>(reader) {

}

#[cfg(test)]
mod tests {
    
}
*/