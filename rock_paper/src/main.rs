use rock_paper::input;
#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors
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
   println!("{}", score)
    
}

fn match_hand(input: &str) -> Hand {
    match input {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => panic!("Invalid input"),
    }
}