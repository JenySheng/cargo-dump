pub mod card;
use std::collections::HashMap;
use crate::card::Card;
use crate::card::Suit;
use crate::card::hand_odds;

fn main() {
    let mut hole: Vec<Card> = vec![];
    hole.push(Card {rank: 7,suit: Suit::Spades,});
    hole.push(Card {rank: 7,suit: Suit::Hearts,});
    let mut comm: Vec<Card> = vec![];
    comm.push(Card {rank: 10,suit: Suit::Diamonds,});
    comm.push(Card {rank: 11,suit: Suit::Spades,});
    comm.push(Card {rank: 12,suit: Suit::Hearts,});
    
    println!("{}", comm.len());
    let mut result: Result<HashMap<String, f64>, String> = hand_odds(&hole,&comm);
    for (key, value) in &result.unwrap() {
        println!("{}: {}", key, value);
    }
    pub fn C(n:i32,k:i32) ->f64{
        let mut ans = 1.0;
        for i in 1..n+1{
            ans*=i as f64;
        }
        for i in 1..k+1{
            ans/=i as f64;
        }
        for i in 1..n-k+1{
            ans/=i as f64;
        }
        return ans;
     }
     
}
