/*

read in each line and split it on the space
first half, we store:
    the original string
    a map of the characters and their count (could just calculate this on scoring the card)
    the bet

    ranking the cards:
    first we histogram it and determine the type:
    5 of a kind
    4 of a kind
    full house (3 + 2)
    three of a kind
    pair
    high card

    then we score them based on their order -> 
    have a map for value to card
    cast the card as an int from a hex expression

*/

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::{Reverse, Ordering};

fn main() -> Result<(), Box<dyn Error>> {
    
    const DEBUG:bool = false;

    let mut original: Vec<String> = Vec::new();

    let file_path = if DEBUG {"input_debug.txt"} else {"input.txt"};
    // Open the file
    let file = File::open(file_path)?;
    let mut hands:Vec<(String, i32)> = Vec::new();

    // Build the dataset
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let orig_line = line?;
            original.push(orig_line);
    }

    for line in &original {
        let mut chunks = line.split(' ');
        let mut hand_str:String = " ".to_string();
        if let Some(hand) = chunks.next() {
            hand_str = hand.to_string();
        }
        if let Some(bet) = chunks.next() {
            hands.push((hand_str, bet.parse::<i32>().unwrap()));
        }
    }

    if DEBUG {
        for hand in &hands {
            println!("{}", "hand: ".to_string() + &hand.0.to_string() + " " + &rank_hand(hand.0.to_string()).to_string());
        }

         println!("{}","====================================================================================");
    }
    hands.sort_by(compare_hands2);
    if DEBUG {
        for hand in &hands {
            println!("{}", "hand: ".to_string() + &hand.0.to_string() + " " + &rank_hand2(hand.0.to_string()).to_string());
        }
    }

    let mut rank = 0;
    let mut total = 0;
    for hand in &hands{
        rank += 1;
        total += hand.1 * rank
    }

    println!("{}", "Total winnings: ".to_string() + &total.to_string());

Ok(())
}

fn compare_hands(hand1:&(String, i32), hand2:&(String, i32)) -> Ordering {
    return rank_hand(hand1.0.to_string()).cmp(&rank_hand(hand2.0.to_string()));
}

fn compare_hands2(hand1:&(String, i32), hand2:&(String, i32)) -> Ordering {
    return rank_hand2(hand1.0.to_string()).cmp(&rank_hand2(hand2.0.to_string()));
}

fn rank_hand(hand:String) -> u32 {
    let card_ranks = HashMap::from([('2','2'), ('3','3'), ('4','4'), ('5','5'), ('6','6'), ('7','7'), ('8','8'), ('9','9'), ('T','A'), ('J','B'), ('Q','C'), ('K','D'), ('A','E')]);
    let mut cards = HashMap::new();
    let hex_str = hand.chars().map(|chr| card_ranks[&chr] ).collect::<String>();
    let hex_hand = u32::from_str_radix(&hex_str, 16).unwrap();
    let mut rank = hex_hand;
    for char in hand.chars(){
        cards.entry(char).or_insert(0);
        cards.entry(char).and_modify(|x| *x += 1);
    }
    let mut score:Vec<u32> = cards.iter().map(|(_key,val)| *val as u32).collect::<Vec<u32>>();
    score.sort_by_key(|w| Reverse(*w));
    
    let mut hand_type:u8 = 0;

    if score[0] == 5 {hand_type = 7;}

    if score[0] == 4 {hand_type = 6;}

    if score[0] == 3 && score[1] == 2 {hand_type = 5;}
    else if score[0] == 3 && score[1] == 1 {hand_type = 4;}

    if score[0] == 2 && score[1] == 2 {hand_type = 3;}
    else if score[0] == 2 && score[1] == 1 {hand_type = 2;}

    if score[0] == 1 {hand_type = 1;};

    rank = rank | (hand_type as u32) << 24;

    return rank;

}

fn rank_hand2(hand:String) -> u32 {
    let card_ranks = HashMap::from([('2','2'), ('3','3'), ('4','4'), ('5','5'), ('6','6'), ('7','7'), ('8','8'), ('9','9'), ('T','A'), ('J','1'), ('Q','C'), ('K','D'), ('A','E')]);
    let mut cards = HashMap::new();
    let hex_str = hand.chars().map(|chr| card_ranks[&chr] ).collect::<String>();
    let hex_hand = u32::from_str_radix(&hex_str, 16).unwrap();
    let mut rank = hex_hand;
    for char in hand.chars(){
        cards.entry(char).or_insert(0);
        cards.entry(char).and_modify(|x| *x += 1);
    }

    
    let mut card_map = cards.iter().map(|(key, val)| (*key, *val)).collect::<Vec<(char,i32)>>();
    
    card_map.sort_by(|a,b| b.1.cmp(&a.1));
    if cards.contains_key(&'J') { 
        let mut keep_joker = false;
        if card_map[0].0 == 'J' && card_map[0].1 == 5 {
            keep_joker = true;
        }
        else if card_map[0].0 == 'J' {
            let jokers = cards[&'J'];
            cards.entry(card_map[1].0).and_modify(|x| *x += jokers);
        }
        else {
            let jokers = cards[&'J'];
            cards.entry(card_map[0].0).and_modify(|x| *x += jokers);
        }
        if !keep_joker {cards.remove(&'J');}
    
    }
    let mut hand_type:u8 = 0;
    let mut score:Vec<u32> = cards.iter().map(|(_key,val)| *val as u32).collect::<Vec<u32>>();
    score.sort_by_key(|w| Reverse(*w));
    if score[0] == 5 {hand_type = 7;}

    if score[0] == 4 {hand_type = 6;}

    if score[0] == 3 && score[1] == 2 {hand_type = 5;}
    else if score[0] == 3 && score[1] == 1 {hand_type = 4;}

    if score[0] == 2 && score[1] == 2 {hand_type = 3;}
    else if score[0] == 2 && score[1] == 1 {hand_type = 2;}

    if score[0] == 1 {hand_type = 1;};

    rank = rank | (hand_type as u32) << 24;

    return rank;

}