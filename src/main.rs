extern crate rand;

use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use std::io;

// tuple type containing card suit + card number
type Card = (i32, &'static str);

// Primary game loop
fn main() {
  let max_value: i32         = 21;
  let dealer_stop_value: i32 = 17;
  let mut hand_count: usize  = 1;

  // Vector contains type Card which is a tuple i32 and string
  let mut hand = vec![generate_card(), generate_card()];
  let mut dealer_hand = vec![generate_card(), generate_card()];

  // Prints hand
  print_hand(hand.as_mut_slice(), false);
  while hand_value(hand.as_mut_slice()) < max_value {
    // Declare empty buffer each iteration
    let mut buffer: String = String::new();

    // Player turn
    println!("Would you like to hit? [y/n]");
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    let buffer: &str = buffer.trim();
    
    if buffer == "y" {
      hand.push(generate_card());
      hand_count += 1;
      println!("Added {} to hand", format!("{} {}", hand[hand_count].0, hand[hand_count].1));
      print_hand(hand.as_mut_slice(), false);
    } else if buffer == "n" {
      println!("You stayed your turn.");
    } else {
      println!("Incorrect input.");
    }

    // Dealer turn
    if hand_value(dealer_hand.as_mut_slice()) < dealer_stop_value {
      dealer_hand.push(generate_card());
      print_hand(dealer_hand.as_mut_slice(), true);
    } else if hand_value(dealer_hand.as_mut_slice()) == dealer_stop_value {
      println!("Dealer stopped at {}", hand_value(dealer_hand.as_mut_slice()))
    } 
  }

  if hand_value(hand.as_mut_slice()) == max_value {
    println!("{}", "Winner Winner");
  } else {
    println!("{}", "Chicken Dinner");
  }
}

fn generate_card() -> Card {
  let suites: [&str; 4]  = ["of Spades", "of Hearts", "of Clubs", "of Diamonds"];
  let mut rng: ThreadRng = thread_rng();
  let card: Card         = (rng.gen_range(1..11), suites[rng.gen_range(0..3)]);
  card
}

fn print_hand(hand_to_print: &mut [Card], is_dealer: bool) {
  if is_dealer {
    println!("{}", "\nCards in dealer's hand:");
    for &(number, suit) in hand_to_print.iter() {
      println!("* {} {}", number, suit)
    }
    println!("Dealer's hand value: {}\n", hand_value(hand_to_print));
  } else {
    println!("{}", "\nCards in your hand:");
    for &(number, suit) in hand_to_print.iter() {
      println!("* {} {}", number, suit)
    }
    println!("Player's hand value: {}\n", hand_value(hand_to_print));
  }
}

fn hand_value(hand_to_print: &mut [Card]) -> i32 {
  let mut card_values: i32 = 0;
  for &(number, _suit) in hand_to_print.iter() {
    card_values += number;
  }
  return card_values
}