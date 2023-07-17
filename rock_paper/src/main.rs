use rock_paper::input;
#[derive(Debug, PartialEq,Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}
    #[derive(Debug, PartialEq ,Clone)]
enum Round {
    Draw,
    Win,
    Lose
}
    


fn main() {
   let input = input();
  
   let rounds = input.split("\n")
   .map(|x| x.split_whitespace().collect::<Vec<&str>>())
   .map(|x| (match_hand(x[0]), match_hand(x[1])))
   .collect::<Vec<(Hand, Hand)>>();

   let mut score:u32 = 0;
 
   for round in rounds{
        match round {
            // If round is a draw
            (Hand::Rock, Hand::Rock) => score += 1 + 3,
            (Hand::Paper, Hand::Paper) => score += 2 + 3,
            (Hand::Scissors, Hand::Scissors) => score += 3 + 3,
            // If you win the round
            (Hand::Scissors , Hand::Rock) | (Hand::Rock , Hand::Paper) | (Hand::Paper , Hand::Scissors) => {
                match round.1 {
                    Hand::Rock => score += 1 + 6,
                    Hand::Paper => score += 2 + 6,
                    Hand::Scissors => score += 3 + 6,
                }
            },
            // If you lose the round
            _ => {
                match round.1 {
                    Hand::Rock => score += 1,
                    Hand::Paper => score += 2,
                    Hand::Scissors => score += 3,
                }
            }
        }
   }
   println!("{}", score);
   println!("{}", round2(input))
}

fn match_hand(input: &str) -> Hand {
    match input {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn match_round(input: &str) -> Round {
    match input {
        "X" => Round::Lose,
        "Y" => Round::Draw,
        "Z" => Round::Win,
        _ => panic!("Invalid input"),
    }
}

// fn round2 (input: String) -> u32 {
//     let mut score:u32 = 0;
//    let rounds = input.split("\n")
//    .map(|x| x.split_whitespace().collect::<Vec<&str>>())
//    .map(|x| (match_hand(x[0]), match_round(x[1])))
//    .collect::<Vec<(Hand, Round)>>();
//     for round in rounds {
//         match round {
//             (Hand::Rock, Round::Win) => score += 6 + 1,
//             (Hand::Rock, Round::Draw) => score += 1 + 3,
//             (Hand::Rock, Round::Lose) => score += 1 ,
//             (Hand::Paper, Round::Win) => score +=  6+2,
//             (Hand::Paper, Round::Draw) => score += 2 + 3,
//             (Hand::Paper, Round::Lose) => score += 2 ,
//             (Hand::Scissors, Round::Win) => score += 3 + 6,
//             (Hand::Scissors, Round::Draw) => score += 3 + 3,
//             (Hand::Scissors, Round::Lose) => score += 3,
//         }
            
//         }
    
//     score
// }
fn round2(input: String) -> u32 {
    let mut score: u32 = 0;
    let rounds = input.split("\n")
    .map(|x| x.split_whitespace().collect::<Vec<&str>>())
    .map(|x| (match_hand(x[0]), match_round(x[1])))
    .collect::<Vec<(Hand, Round)>>();

    for round in rounds {
        let chosen_hand = choose_hand(round.0, round.1.clone());

        match round.1 {
            Round::Win => score += score_of_hand(&chosen_hand) + 6,
            Round::Draw => score += score_of_hand(&chosen_hand) + 3,
            Round::Lose => score += score_of_hand(&chosen_hand),
        }
    }

    score
}

fn score_of_hand(hand: &Hand) -> u32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}
fn choose_hand(opponent: Hand, round: Round) -> Hand {
    match round {
        Round::Win => match opponent {
            Hand::Rock => Hand::Paper,       // To win against Rock, choose Paper
            Hand::Paper => Hand::Scissors,   // To win against Paper, choose Scissors
            Hand::Scissors => Hand::Rock,    // To win against Scissors, choose Rock
        },
        Round::Draw => opponent,             // To draw, choose the same as the opponent
        Round::Lose => match opponent {      // To lose, choose the option that loses to opponent
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        },
    }
}