use std::collections::{HashMap, HashSet};
use std::vec::Vec;
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Card {
    pub rank: u32,
    pub suit: Suit,
}


impl Card {
    fn new(rank: u32, suit: Suit) -> Self {
        Card { rank, suit }
    }
}


pub fn hand_odds(hole_cards: &Vec<Card>, community_cards: &Vec<Card>) -> Result<HashMap<String, f64>, String> {
    if hole_cards.len() != 2 {
        return Err(String::from("Invalid number of hole cards. Expected 2."));
    }


    if !(community_cards.len() >= 3 && community_cards.len() <= 5) && !(community_cards.len() == 0)  {
        return Err(String::from("Invalid number of community cards. Expected at least 3."));
    }
    if community_cards.len() == 0 {
        return Ok(zero_cards(hole_cards, community_cards));
    }
    if community_cards.len() == 3 {
        return Ok(three_cards(hole_cards, community_cards));
    }
    if community_cards.len() == 4 {
        return Ok(four_cards(hole_cards, community_cards));
    }
    if community_cards.len() == 5 {
        return Ok(five_cards(hole_cards, community_cards));
    }
    return Err(String::from("unknown"));
}




pub fn is_straight(mut num:Vec<u32>) -> bool{
    num[0] = num[13];
    for i in 1..11 {
        if num[i]==0 {
        continue;
        }
        let mut yes:bool = true;
        for j in 0..5 {
        if num[(i+j)%13] == 0{
            yes = false;
            break;
        }
        }
        if yes == true {
            num[0] = 0;
            return true;
        }
    }
    num[0] = 0;
    return false;
}
   
pub fn is_flush(suit:Vec<u32>) -> bool{
    for num in suit{
        if num >= 5 {
            return true;
        }
    }
    return false;
}
 
 
pub fn is_four(num:Vec<u32>) -> bool{
    for n in num{
        if n == 4 {
            return true;
        }
    }
    return false;
}
pub fn is_three(num:Vec<u32>) -> bool{
    for n in num{
        if n == 3 {
            return true;
        }
    }
    return false;
}
pub fn is_pair(num:Vec<u32>) -> bool{
    for n in num{
        if n == 2 {
            return true;
        }
    }
    return false;
}
pub fn is_two_pair(num:Vec<u32>) -> bool{
    let mut two:u32 = 0;
    for n in num{
        if n == 2 {
            two+=1;
        }
    }
    return two >= 2;
}
pub fn five_cards(hole: &Vec<Card>, comm: &Vec<Card>) -> HashMap<String, f64> {
    let mut al:Vec<Card> = vec![];
    al.push(hole[0].clone());
    al.push(hole[1].clone());
    al.push(comm[0].clone());
    al.push(comm[1].clone());
    al.push(comm[2].clone());
    al.push(comm[3].clone());
    al.push(comm[4].clone());
    let mut result = HashMap::new();
    for i in 0..7{
        for j in i..7{
            let mut all:Vec<Card> = vec![];
            for n in 0..7{
                if n!=i && n!=j{
                    all.push(al[n].clone());
                }
            }
            let mut num: Vec<u32> = vec![0; 14];
            let mut hua: Vec<u32> = vec![0; 4];
            for card in all{
                num[card.rank as usize]+=1;
                match card.suit {
                    Suit::Hearts => hua[0]+=1,
                    Suit::Diamonds => hua[1]+=1,
                    Suit::Clubs => hua[2]+=1,
                    Suit::Spades => hua[3]+=1,
                }
            }
           
            if is_straight(num.clone())&&is_flush(hua.clone()){
                if num[1]!=0 && num[13]!=0{
                    result.insert(String::from("Royal Flush"), 1.0);
                }
                result.insert(String::from("Straight Flush"), 1.0);
            }
            if is_four(num.clone()){
                result.insert(String::from("Four of a Kind"), 1.0);
            }
            if is_three(num.clone())&&is_pair(num.clone()){
                result.insert(String::from("Full House"), 1.0);
            }
            if is_flush(hua.clone()){
                result.insert(String::from("Flush"), 1.0);
            }
            if is_straight(num.clone()){
                result.insert(String::from("Straight"), 1.0);
            }
            if is_three(num.clone()){
                result.insert(String::from("Three of a Kind"), 1.0);
            }
            if is_two_pair(num.clone()){
                result.insert(String::from("Two Pair"), 1.0);
            }
            if is_pair(num.clone()){
                result.insert(String::from("Pair"), 1.0);
            }
            result.insert(String::from("High Card"), 1.0);
        }
    }
    let mut ret = HashMap::new();
    if result.contains_key("Royal Flush"){
        ret.insert(String::from("Royal Flush"), 1.0);
        return ret;
    }
    if result.contains_key("Straight Flush"){
        ret.insert(String::from("Straight Flush"), 1.0);
        return ret;
    }
    if result.contains_key("Four of a Kind"){
        ret.insert(String::from("Four of a Kind"), 1.0);
        return ret;
    }
    if result.contains_key("Full House"){
        ret.insert(String::from("Full House"), 1.0);
        return ret;
    }
    if result.contains_key("Flush"){
        ret.insert(String::from("Flush"), 1.0);
        return ret;
    }
    if result.contains_key("Straight") {
        ret.insert(String::from("Straight"), 1.0);
        return ret;
    }
    if result.contains_key("Three of a Kind"){
        ret.insert(String::from("Three of a Kind"), 1.0);
        return ret;
    }
    if result.contains_key("Two Pair") {
        ret.insert(String::from("Two Pair"), 1.0);
        return ret;
    }
    if result.contains_key("Pair"){
        ret.insert(String::from("Pair"), 1.0);
        return ret;
    }
    if result.contains_key("High Card"){
        ret.insert(String::from("High Card"), 1.0);
        return ret;
    }
    return ret;
}
 
pub fn four_cards(hole: &Vec<Card>, comm: &Vec<Card>) -> HashMap<String, f64> {
    let mut result = HashMap::new();
    result.insert(String::from("Royal Flush"), 0.0);
    result.insert(String::from("Straight Flush"), 0.0);
    result.insert(String::from("Four of a Kind"), 0.0);
    result.insert(String::from("Full House"), 0.0);
    result.insert(String::from("Flush"), 0.0);
    result.insert(String::from("Straight"), 0.0);
    result.insert(String::from("Three of a Kind"), 0.0);
    result.insert(String::from("Two Pair"), 0.0);
    result.insert(String::from("Pair"), 0.0);
    result.insert(String::from("High Card"), 0.0);
    let mut adcom:Vec<Card> = vec![];
    adcom.push(comm[0].clone());
    adcom.push(comm[1].clone());
    adcom.push(comm[2].clone());
    adcom.push(comm[3].clone());
    for i in 1..14 {
        for j in 0..4 {
            let mut same: bool = false;
            let mut add = Card {rank: i,suit: Suit::Spades,};
            if j == 1 {
                add.suit = Suit::Hearts;
            }
            if j == 2 {
                add.suit = Suit::Diamonds;
            }
            if j == 3 {
                add.suit = Suit::Clubs;
            }
            for card in hole {
                if card.rank == i && card.suit == add.suit {
                    same = true;
                }
            }
            if same {
                continue;
            }
            for card in &adcom {
                if card.rank == i && card.suit == add.suit {
                    same = true;
                }
            }
            if same {
                continue;
            }
            adcom.push(add);
            let mut fivec: HashMap<String, f64> = five_cards(&hole,&adcom);
            adcom.pop();
            for key in fivec.keys() {
                *result.get_mut(key).unwrap() += 1.0;
            }
        }
    }
    let mut total: f64 = 0.0;


    for (_, val) in result.iter_mut() {
        total += *val;
    }
    for (_, val) in result.iter_mut() {
        *val /= total;
    }
    return result;
}


pub fn three_cards(hole: &Vec<Card>, comm: &Vec<Card>) -> HashMap<String, f64> {
    let mut result = HashMap::new();
    result.insert(String::from("Royal Flush"), 0.0);
    result.insert(String::from("Straight Flush"), 0.0);
    result.insert(String::from("Four of a Kind"), 0.0);
    result.insert(String::from("Full House"), 0.0);
    result.insert(String::from("Flush"), 0.0);
    result.insert(String::from("Straight"), 0.0);
    result.insert(String::from("Three of a Kind"), 0.0);
    result.insert(String::from("Two Pair"), 0.0);
    result.insert(String::from("Pair"), 0.0);
    result.insert(String::from("High Card"), 0.0);
    let mut adcom:Vec<Card> = vec![];
    adcom.push(comm[0].clone());
    adcom.push(comm[1].clone());
    adcom.push(comm[2].clone());
    for i in 1..14 {
        for j in 0..4 {
            let mut same: bool = false;
            let mut add = Card {rank: i,suit: Suit::Spades,};
            if j == 1 {
                add.suit = Suit::Hearts;
            }
            if j == 2 {
                add.suit = Suit::Diamonds;
            }
            if j == 3 {
                add.suit = Suit::Clubs;
            }
            for card in hole {
                if card.rank == i && card.suit == add.suit {
                    same = true;
                }
            }
            if same {
                continue;
            }
            for card in &adcom {
                if card.rank == i && card.suit == add.suit {
                    same = true;
                }
            }
            if same {
                continue;
            }
            adcom.push(add);
            let mut fivec: HashMap<String, f64> = four_cards(&hole,&adcom);
            adcom.pop();
            for key in fivec.keys() {
                *result.get_mut(key).unwrap() += fivec.get(key).unwrap();
            }
        }
    }
    let mut total: f64 = 0.0;


    for (_, val) in result.iter_mut() {
        total += *val;
    }
    for (_, val) in result.iter_mut() {
        *val /= total;
    }
    return result;
}

pub fn C(n:i32,k:i32) ->f32{
    let mut ans = 1.0;
    for i in 1..n+1{
        ans*=i;
    }
    for i in 1..k+1{
        ans/=i;
    }
    for i in 1..n-k+1{
        ans/=i;
    }
    return ans;
 }

 pub fn royal_flush(c: &Vec<Card>) -> f64 {
    //两张同时参与
    if c[0].suit == c[1].suit && (c[0].rank>=10||c[0].rank == 1)&&(c[1].rank>=10||c[1].rank == 1){
        return C(47,2)/C(50,5)+3.0/C(50,5);
    }
    //一张都不参与
    if(c[0].rank<10&&c[0].rank!=1)&&(c[1].rank!=1&&c[1].rank<10){
        return 4.0/C(50,5);
    }
    //只有一张可以参与
    if(c[0].rank>=10||c[0].rank == 1)&&(c[1].rank!=1&&c[1].rank<10){
        return C(46,1)/C(50,5)+3.0/C(50,5);
    }
    if(c[1].rank>=10||c[1].rank == 1)&&(c[0].rank!=1&&c[0].rank<10){
        return C(46,1)/C(50,5)+3.0/C(50,5);
    }
    //♥️K 和 ♠️Q
    return 2.0*C(46,1)/C(50,5)+2.0/C(50,5);
 }

 
 

pub fn four_of_kind(c: &Vec<Card>) -> f64 {
    // 两张卡数字相同
    if c[0].rank == c[1].rank {
        //五张里面需要两张一定的 or 五张里面有四张一样的
        return C(3/48)/C(50,5) + 12/C(50,5);
    }
    //两张卡数字不相同
    if c[0].rank != c[1].rank {
        //五张里面需要两张一定的 * 2 or 五张里面有四张一样的
        return 2 * (C(47,2)/C(50,5)) + 11/C(50,5);
    }
 }
 
 
 
 pub fn straight_flush(c: &Vec<Card>) -> f64 {
     let a_p : f64 = 0.0;
     let b_p : f64 = 0.0;
     let intersection : f64 = 0.0;
     if c[0].rank <= 4 && c[1].rank <= 4 {
         a_p = c[0].rank;
         b_p = c[1].rank;
     } else if c[0].rank >= 10 && c[1].rank >= 10 {
         a_p = 14.0 - c[0].rank;
         b_p = 14.0 - c[1].rank;
     } else {
         a_p = 5.0;
         b_p = 5.0;
     }
     if ((c[0].rank - c[1].rank).abs() < 5) && (c[1].suit == c[0].suit) {
         if (c[0].rank <= 4 && c[1].rank <= 4) {
             intersection = 4 - (c[0].rank - c[1].rank).abs();
         } else if (c[0].rank >= 10 && c[1].rank >= 10) {
             intersection = 4 - (c[0].rank - c[1].rank).abs();
         } else {
             intersection = 5 - (c[0].rank - c[1].rank).abs();
         }
     }
     return intersection * C(47, 2) / C(50, 5) + (a_p + b_p - 2 * intersection) * C(46, 1)/C(50, 5) + (36.0 - a_p - b_p + intersection) / C(50, 5);
 }
 
 pub fn flush(c: &Vec<Card>) -> f64 {
    if c[1].suit == c[0].suit {
        return C(11, 3) * C(47, 2) / C(50, 5) + 3.0 * C(13, 5) / C(50, 5) - straight_flush(c) - royal_flush(c);
    } else {
        return 2.0 * C(12, 4) * C(46, 1)/ C(50, 5) + 2.0 * C(13, 5) / C(50, 5) - straight_flush(c) - royal_flush(c);
    }
}

