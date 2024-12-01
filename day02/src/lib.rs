#[derive(Copy, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors
}

#[derive(Copy, Clone)]
enum WinLossDraw {
    Win,
    Loss,
    Draw
}

pub struct Day02 {
    rounds: Vec<(char, char)>,
}

impl Day02 {

    fn translate_opponent(value: char) -> Rps {
        match value {
            'A' => Rps::Rock,
            'B' => Rps::Paper,
            'C' => Rps::Scissors,
            _ => {panic!("Invalid opponent move: {}", value); }
        }
    }

    fn translate_our_move(value: char) -> Rps {
        match value {
            'X' => Rps::Rock,
            'Y' => Rps::Paper,
            'Z' => Rps::Scissors,
            _ => { panic!("Invalid choice: {}", value); }
        }
    }

    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut vec = Vec::<(char, char)>::new();
        for line in lines {
            let mut iter = line.chars();
            let opponent = if let Some(value) = iter.next() { value } else { panic!("Failed to parse line {line}"); };
            iter.next();
            let play = if let Some(value) = iter.next() { value } else { panic!("Failed to parse line {line}"); };
            vec.push((opponent, play));            
        }
        Day02{rounds: vec}
    }

    pub fn part1(&self) -> u32 {
        let mut sum_of_rounds = 0;
        for round in &self.rounds {
            let opponent = Self::translate_opponent(round.0);
            let our_move = Self::translate_our_move(round.1);
            sum_of_rounds += Self::evaluate_round(opponent, our_move);
        }
        sum_of_rounds
    }


    fn determine_score(play: Rps, outcome: WinLossDraw) -> u32 {
        let play_score = match play {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3
        };
        let outcome_score = match outcome {
            WinLossDraw::Win => 6,
            WinLossDraw::Loss => 0,
            WinLossDraw::Draw => 3,
        };
        play_score + outcome_score
    }

    fn determine_outcome(opponent: Rps, play: Rps) -> WinLossDraw {
        match (opponent, play) {
            (Rps::Rock, Rps::Scissors) => WinLossDraw::Loss,
            (Rps::Rock, Rps::Rock) => WinLossDraw::Draw,
            (Rps::Rock, Rps::Paper) => WinLossDraw::Win,
            (Rps::Paper, Rps::Rock) => WinLossDraw::Loss,
            (Rps::Paper, Rps::Paper) => WinLossDraw::Draw,
            (Rps::Paper, Rps::Scissors) => WinLossDraw::Win,
            (Rps::Scissors, Rps::Paper) => WinLossDraw::Loss,
            (Rps::Scissors, Rps::Scissors) => WinLossDraw::Draw,
            (Rps::Scissors, Rps::Rock) => WinLossDraw::Win,
        }
    }
    fn evaluate_round(opponent: Rps, play: Rps) -> u32 {
        Self::determine_score(play, Self::determine_outcome(opponent, play))
    }

    fn pt2_translate_outcome(value: char) -> WinLossDraw {
        match value {
            'X' => WinLossDraw::Loss,
            'Y' => WinLossDraw::Draw,
            'Z' => WinLossDraw::Win,
            _ => { panic!("Unexpected value in translate_outcome: {value}"); }
        }
    }

    fn determine_our_play(opponent: Rps, outcome: WinLossDraw) -> Rps {
        match (opponent, outcome) {
            (Rps::Rock, WinLossDraw::Win) => Rps::Paper,
            (Rps::Rock, WinLossDraw::Loss) => Rps::Scissors,
            (Rps::Paper, WinLossDraw::Win) => Rps::Scissors,
            (Rps::Paper, WinLossDraw::Loss) => Rps::Rock,
            (Rps::Scissors, WinLossDraw::Win) => Rps::Rock,
            (Rps::Scissors, WinLossDraw::Loss) => Rps::Paper,
            (_, WinLossDraw::Draw) => opponent,
        }
    }

    fn pt2_evaluate_round(opponent: Rps, outcome: WinLossDraw) -> u32 {
        Self::determine_score(Self::determine_our_play(opponent, outcome), outcome)
    }

    pub fn part2(&self) -> u32 {
        let mut sum_of_rounds = 0;
        for round in &self.rounds {
            let outcome = Self::pt2_translate_outcome(round.1);
            sum_of_rounds += Self::pt2_evaluate_round(Self::translate_opponent(round.0), outcome)
        }
        sum_of_rounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_ROUNDS: &str =
"A Y
B X
C Z";

    #[test]
    fn part1_sample_results_in_15() {
        let day02 = Day02::new(SAMPLE_ROUNDS.lines());
        assert_eq!(day02.part1(), 15);
    }

    #[test]
    fn part2_sample_results_in_12() {
        let day02 = Day02::new(SAMPLE_ROUNDS.lines());
        assert_eq!(day02.part2(), 12);
    }
}