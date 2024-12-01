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
    rounds: Vec<(Rps, Rps)>,
}

impl Day02 {

    
    fn translate_opponent(value: Option<char>) -> Rps {
        match value {
            Some('A') => Rps::Rock,
            Some('B') => Rps::Paper,
            Some('C') => Rps::Scissors,
            Some(value) => {panic!("Invalid opponent move: {}", value); }
            None => {panic!("Opponent move not found"); }
        }
    }

    fn translate_move(value: Option<char>) -> Rps {
        match value {
            Some('X') => Rps::Rock,
            Some('Y') => Rps::Paper,
            Some('Z') => Rps::Scissors,
            Some(value) => { panic!("Invalid choice: {}", value); }
            None => { panic!("Choice not found"); }
        }
    }
    pub fn new(lines: std::str::Lines<'_>) -> Self {
        let mut vec = Vec::<(Rps, Rps)>::new();
        for line in lines {
            let mut iter = line.chars();
            let opponent = Self::translate_opponent(iter.next());
            iter.next();
            let play = Self::translate_move(iter.next());
            vec.push((opponent, play));            
        }
        Day02{rounds: vec}
    }

    pub fn part1(&self) -> u32 {
        let mut sum_of_rounds = 0;
        for round in &self.rounds {
            sum_of_rounds += Self::evaluate_round(round.0, round.1);
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

}