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
pub fn check_validity(hole_cards: &Vec<Card>, community_cards: &Vec<Card>) -> i64{
    //check for redundant cards
    let mut c: Vec<i32> = Vec::with_capacity(53);
    let mut r: Vec<i32> = Vec::with_capacity(14);
    for i in 0..53 {
        c.push(0);
    }
    for i in 0..14 {
        r.push(0);
    }
    for ele in hole_cards  {
        if ele.suit == Suit::Hearts{
            c[ele.rank as usize]+=1;
            r[ele.rank as usize]+=1;
        }
        if ele.suit == Suit::Diamonds{
            c[ele.rank as usize+13]+=1;
            r[ele.rank as usize]+=1;
        }
        if ele.suit == Suit::Clubs{
            c[ele.rank as usize+26]+=1;
            r[ele.rank as usize]+=1;
        }
        if ele.suit == Suit::Spades{
            c[ele.rank as usize+39]+=1;
            r[ele.rank as usize]+=1;
        }
    }
    for ele in community_cards {
        if ele.suit == Suit::Hearts{
            c[ele.rank as usize]+=1;
            r[ele.rank as usize]+=1;
        }
        if ele.suit == Suit::Diamonds{
            c[ele.rank as usize+13]+=1;
            r[ele.rank as usize]+=1;
        }
        if ele.suit == Suit::Clubs{
            c[ele.rank as usize+26]+=1;
            r[ele.rank as usize]+=1;
        }
        if ele.suit == Suit::Spades{
            c[ele.rank as usize+39]+=1;
            r[ele.rank as usize]+=1;
        }
    }
    for ele in c {
        if ele>1{
            return 1;
        }
    }
    for ele in r {
        if ele>4{
            return 2;
        }
    }
    return 0;
}

pub fn hand_odds(hole_cards: &Vec<Card>, community_cards: &Vec<Card>) -> Result<HashMap<String, f64>, String> {
    if hole_cards.len() != 2 {
        return Err(String::from("Invalid number of hole cards. Expected 2."));
    }
    if !(community_cards.len() >= 3 && community_cards.len() <= 5) && !(community_cards.len() == 0)  {
        return Err(String::from("Invalid number of community cards. Expected at least 3 and at most 5."));
    }
    match check_validity(hole_cards, community_cards){
        1 => return Err(String::from("Error: Same card appeared twice.")),
        2 => return Err(String::from("Error: Some numbers appeared more than 4 times.")),
        _ => {},
    }
    if community_cards.len() == 0 {
        return Ok(zero_cards(hole_cards));
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
                if num[1]!=0 && num[13]!=0 &&num[12] !=0{
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
pub fn zero_cards(hole: &Vec<Card>) -> HashMap<String, f64> {
    println!("{}", String::from("Enter zero card"));
    let mut al:Vec<Card> = vec![];
    al.push(hole[0].clone());
    al.push(hole[1].clone());
    let mut result = HashMap::new();
    result.insert(String::from("Royal Flush"), royal_flush(&al));
    result.insert(String::from("Straight Flush"), straight_flush(&al));
    result.insert(String::from("Four of a Kind"), four_of_kind(&al));
    result.insert(String::from("Full House"), full_house(&al));
    result.insert(String::from("Flush"), flush(&al));
    result.insert(String::from("Straight"), straight(&al));
    result.insert(String::from("Three of a Kind"), three_of_kind(&al));
    result.insert(String::from("Two Pair"), two_pairs(&al));
    result.insert(String::from("Pair"), pair(&al));
    result.insert(String::from("High Card"), high_card(&al));
    return result;
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

pub fn royal_flush(c: &Vec<Card>) -> f64 {
    //using two hole cards
    if c[0].suit == c[1].suit && (c[0].rank>=10||c[0].rank == 1)&&(c[1].rank>=10||c[1].rank == 1){
        return C(47,2)/C(50,5)+3.0/C(50,5);
    }
    //using no hole cards
    if(c[0].rank<10&&c[0].rank!=1)&&(c[1].rank!=1&&c[1].rank<10){
        return 4.0/C(50,5);
    }
    //using one hole card
    if(c[0].rank>=10||c[0].rank == 1)&&(c[1].rank!=1&&c[1].rank<10){
        return C(46,1)/C(50,5)+3.0/C(50,5);
    }
    if(c[1].rank>=10||c[1].rank == 1)&&(c[0].rank!=1&&c[0].rank<10){
        return C(46,1)/C(50,5)+3.0/C(50,5);
    }
    //♥️K and ♠️Q
    return 2.0*C(46,1)/C(50,5)+2.0/C(50,5);
}

pub fn four_of_kind(c: &Vec<Card>) -> f64 {
    // hole cards have the same rank
    if c[0].rank == c[1].rank {
        // there needs to be 2 cards with the same rank as the hole cards or 4 cards of the same rank.
        return C(48,3)/C(50,5) + 12 as f64/C(50,5);
    }
    // hole cards have different ranks
    return 2.0* (C(47,2)/C(50,5)) + 11.0/C(50,5);
}
 
 
pub fn straight_flush(c: &Vec<Card>) -> f64 {
    let mut a_p : f64 = 0.0;
    let mut b_p : f64 = 0.0;
    let mut intersection : f64 = 0.0;
    if c[0].rank <= 4 && c[1].rank <= 4 {
        a_p = c[0].rank as f64;
        b_p = c[1].rank as f64;
    } else if c[0].rank >= 10 && c[1].rank >= 10 {
        a_p = 14.0 - c[0].rank as f64;
        b_p = 14.0 - c[1].rank as f64;
    } else {
        a_p = 5.0;
        b_p = 5.0;
    }
    if ((c[0].rank as f64 - c[1].rank as f64).abs() < 5.0) && (c[1].suit == c[0].suit) {
        if c[0].rank <= 4 && c[1].rank <= 4 {
            intersection = 4.0 - (c[0].rank as f64- c[1].rank as f64).abs();
        } else if c[0].rank >= 10 && c[1].rank >= 10 {
            intersection = 4.0 - (c[0].rank as f64- c[1].rank as f64).abs();
        } else {
            intersection = 5.0 - (c[0].rank as f64 - c[1].rank as f64).abs();
        }
    }
    return intersection * C(47, 2) / C(50, 5) + (a_p + b_p - 2.0 * intersection) * C(46, 1)/C(50, 5) + (36.0 - a_p - b_p + intersection) / C(50, 5);
}
 
pub fn flush(c: &Vec<Card>) -> f64 {
    if c[1].suit == c[0].suit {
        return C(11, 3) * C(47, 2) / C(50, 5) + 3.0 * C(13, 5) / C(50, 5) - straight_flush(c) - royal_flush(c);
    } else {
        return 2.0 * C(12, 4) * C(46, 1)/ C(50, 5) + 2.0 * C(13, 5) / C(50, 5) - straight_flush(c) - royal_flush(c);
    }
}

pub fn full_house(c: &Vec<Card>) -> f64 {
    let mut p1:f64 = 0.0;
    let mut p2:f64 = 0.0;
    let mut p3:f64 = 0.0;
    let mut p4:f64 = 0.0;
    if c[0].rank != c[1].rank {
        //AAABB or AABBB: (sequantially choose A-A-B-any card save A and B-any card save A）* number of permutations.
        //2 * (3/50 * 2/49 * 3/48) * 44/47 * 45/46 * A(5,5)
        //using two hole cards
        p1 = 2.0 * ((C(3, 1) * C(3, 2) * C(45, 2) - 11.0) / C(50, 5)); 
        //using one hole card
        p2 = (2.0 * C(3, 1) * C(4, 3) * 11.0 * C(46, 1) - C(3, 1) * C(3, 1) * C(4,3) * 11.0) / C(50,5); 
        p3 = 2.0 * C(3, 2) * C (4, 2) * C(11, 1) * C(40,1) / C(50,5); 
        //using zero hole cards
        p4 = C(4, 2)*C(4,3)*C(11,2)/ C(50,5); 
        return p1 + p2 + p3 + p4;  
    }
    //hole cards have the same rank（0 + 3 cards with the same rank/ 1 + 2cards with the same rank）
    if c[0].rank == c[1].rank {
        //(AAABB+xx）
		//p1 = 2/50 * 48/49 * 3/48 * 44/47 * 45/46
        p1 = C(2,1) * C(12,1)*C(4,2) * C(44,1) * C(40,1) / 2.0 / C(50,5);  		
        //(AABBB+xx
		//p2 = 48/50 * 3/49 * 2/48 * 44/47 * 45/46
	    p2 = C(12,1)*C(4,3) * C(44,2)/C(50,5); 
        //AAABBB+X
	    p3 = C(2,1) * C(12,1)*C(4,3) * C(44,1) / C(50,5); 
        return p1 + p2 + p3; 
    }
    return -1.0; 
}

pub fn three_of_kind (c: &Vec<Card>)  -> f64 {
    let mut p1:f64 = 0.0;
    let mut p2:f64 = 0.0;
	if c[0].rank == c[1].rank {
        // a card + three of a kind - four of kind -full house
        //AA ABCDE
        return 2.0 * C(50,4) / C(50,5) - four_of_kind(c) - full_house(c);
    }
    if c[0].rank != c[1].rank {
        //AB AACDE
        p1= 2.0 * C(3,2) * C(44,1) * C(40,1)*C(36,1)/ 6.0 /C(50,5);
        //AB CCCDE
        p2= 4.0*11.0*C(40,1)*C(36,1)/C(50,5);
        return p1 + p2;
    }
    return -1.0;
}

pub fn straight(c: &Vec<Card>) -> f64 {
    let mut a_p : f64 = 0.0;
    let mut b_p : f64 = 0.0;
    let mut intersection : f64 = 0.0;
    if c[0].rank <= 4 && c[1].rank <= 4 {
        a_p = c[0].rank as f64;
        b_p = c[1].rank as f64;
    } else if c[0].rank >= 10 && c[1].rank >= 10 {
        a_p = 14.0 - c[0].rank as f64;
        b_p = 14.0 - c[1].rank as f64;
    } else {
        a_p = 5.0;
        b_p = 5.0;
    }
    if ((c[0].rank as f64 - c[1].rank as f64).abs() < 5.0) && (c[1].suit == c[0].suit) {
        if c[0].rank <= 4 && c[1].rank <= 4 {
            intersection = 4.0 - (c[0].rank as f64- c[1].rank as f64).abs();
        } else if c[0].rank >= 10 && c[1].rank >= 10 {
            intersection = 4.0 - (c[0].rank as f64- c[1].rank as f64).abs();
        } else {
            intersection = 5.0 - (c[0].rank as f64 - c[1].rank as f64).abs();
        }
    }
    return intersection * C(47, 2) / C(50, 5)*60.0  + (a_p + b_p - 2.0 * intersection) * C(46, 1)/C(50, 5)*252.0 + (36.0 - a_p - b_p + intersection) / C(50, 5)*1020.0 + royal_flush(c)*1020.0;
}

pub fn two_pairs(c: &Vec<Card>) -> f64 {
    //AA BBCDE
    if c[0].rank == c[1].rank {
        return 12.0*C(4,2)*C(50,3)/C(50,5)- three_of_kind(c)-full_house(c) - four_of_kind(c);
    }
    //AB ABCDE
    let p1:f64 = 3.0*3.0*C(44,3)/C(50,5) - three_of_kind(c)-full_house(c) - four_of_kind(c);
    //AB ACCDE
    let p2:f64 = 2.0*3.0*C(4,2)*11.0*C(40,2)/C(50,5);
    //AB CCDDE
    let p3:f64 = C(11,2)*C(4,2)*C(4,2)*C(36,1)/C(50,5);
    return p1 + p2 + p3;
}

pub fn pair(c: &Vec<Card>)  -> f64 {
    if c[0].rank == c[1].rank {
        return 1.0- two_pairs(c) - straight(c) - three_of_kind(c) - flush(c)-full_house(c) - four_of_kind(c) - straight_flush(c) - royal_flush(c);
    }
    return (11.0*C(4,2)*C(50,3)+(1.0-44.0/50.0*43.0/49.0*42.0/48.0*41.0/47.0*40.0/46.0))/C(50,5) - two_pairs(c) - three_of_kind(c)-full_house(c) - four_of_kind(c);
}
  
  
pub fn high_card(c: &Vec<Card>) -> f64 {
    if c[0].rank == c[1].rank {
        return 0.0;
    }
    return 1.0 - pair(c) - two_pairs(c) - straight(c) - three_of_kind(c) - flush(c)-full_house(c) - four_of_kind(c) - straight_flush(c) - royal_flush(c);
}
  