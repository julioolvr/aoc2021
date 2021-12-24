use std::env;

fn main() {
    let (player_1, player_2) = if env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--sample")
    {
        (4, 8)
    } else {
        (9, 3)
    };

    let mut game = DeterministicGame::new(player_1, player_2);
    let (player_1_final_score, player_2_final_score) = game.by_ref().last().unwrap();
    let part_1 = [player_1_final_score, player_2_final_score]
        .iter()
        .min()
        .unwrap()
        * game.total_rolls();
    println!("Part 1: {}", part_1);
}

struct DeterministicGame {
    player_1_position: usize,
    player_2_position: usize,
    player_1_score: usize,
    player_2_score: usize,
    dice: DeterministicDice,
}

impl DeterministicGame {
    fn new(player_1_position: usize, player_2_position: usize) -> Self {
        DeterministicGame {
            player_1_position: player_1_position - 1,
            player_2_position: player_2_position - 1,
            player_1_score: 0,
            player_2_score: 0,
            dice: DeterministicDice::new(),
        }
    }

    fn total_rolls(&self) -> usize {
        self.dice.roll
    }
}

impl Iterator for DeterministicGame {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.player_1_score >= 1000 || self.player_2_score >= 1000 {
            return None;
        }

        let player_1_roll: usize = self.dice.by_ref().take(3).sum();
        self.player_1_position = (self.player_1_position + player_1_roll) % 10;
        self.player_1_score += self.player_1_position + 1;

        if self.player_1_score >= 1000 {
            return Some((self.player_1_score, self.player_2_score));
        }

        let player_2_roll: usize = self.dice.by_ref().take(3).sum();
        self.player_2_position = (self.player_2_position + player_2_roll) % 10;
        self.player_2_score += self.player_2_position + 1;
        Some((self.player_1_score, self.player_2_score))
    }
}

struct DeterministicDice {
    roll: usize,
}

impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice { roll: 0 }
    }
}

impl Iterator for DeterministicDice {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let roll = self.roll % 100 + 1;
        self.roll += 1;
        Some(roll)
    }
}
