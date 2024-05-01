use std::io;
include!("card.rs");

fn main() {
    let mut hole_cards: Vec<Card> = Vec::new();
    let mut community_cards: Vec<Card> = Vec::new();
    println!("Hi! Welcome to the probability calculation for a game of Texas Hold'em Poker!");
    println!("Note: input your card in this format:  Heart 1 , Diamond 7, Club 10, Spade 13. One card per line.");
    println!("Please select your hole cards. After you finish your input, type 'done'.");

    let mut input = String::new();
    let mut num_lines:i64 = 0;
    while input !="done\n"&&num_lines<5{
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if(input.trim() == "done"){
            break;
        }
        let c: Vec<&str> = input.split(',').collect();
        for ele in c {
            let two: Vec<&str> = ele.split(' ').collect();
            if two.len()!=2{
                for ele in two{
                    println!("{}", ele);
                }
                panic!("Invalid Input! Too many element for one card.");
            }
            let c_suit: String = two[0].chars().filter(|c| !c.is_whitespace()).collect();
            let c_rank: String = two[1].chars().filter(|c| !c.is_whitespace()).collect();
            let mut card = Card {rank: 1,suit: Suit::Hearts,};
            let num: Result<u32, _> = c_rank.parse();
            match num {
                Ok(num) => {
                    card.rank = num;
                }
                Err(err) => {
                    panic!("Invalid Input! '{}' is not a number.", c_rank);
                }
            }
            match c_suit.as_str(){
                "Heart" => card.suit = Suit::Hearts,
                "Diamond" => card.suit = Suit::Diamonds,
                "Club" => card.suit = Suit::Clubs,
                "Spade" => card.suit = Suit::Spades,
                _ => panic!("Invalid Input! '{}' is not a valid suit. Please input Heart, Diamond, Club, or Spade.", c_suit),
            }
            hole_cards.push(card);
        }
        num_lines+=1;
    }
    println!("Please select your community cards. After you finish your input, type 'done'. If you don't have any community cards to input, simply type 'done'.");
    input = "".to_string();
    num_lines = 0;
    println!("{}",input);
    while input!="done\n"&&num_lines<5{
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if(input == "done\n"){
            break;
        }
        let c: Vec<&str> = input.split(',').collect();
        for ele in c {
            let two: Vec<&str> = ele.split(' ').collect();
            if two.len()!=2{
                for ele in two{
                    println!("{}", ele);
                }
                panic!("Invalid Input! Too many element for one card.");
            }
            let c_suit: String = two[0].chars().filter(|c| !c.is_whitespace()).collect();
            let c_rank: String = two[1].chars().filter(|c| !c.is_whitespace()).collect();
            let mut card = Card {rank: 1,suit: Suit::Hearts,};
            let num: Result<u32, _> = c_rank.parse();
            match num {
                Ok(num) => {
                    card.rank = num;
                }
                Err(err) => {
                    panic!("Invalid Input! '{}' is not a number.", c_rank);
                }
            }
            match c_suit.as_str(){
                "Heart" => card.suit = Suit::Hearts,
                "Diamond" => card.suit = Suit::Diamonds,
                "Club" => card.suit = Suit::Clubs,
                "Spade" => card.suit = Suit::Spades,
                _ => panic!("Invalid Input! '{}' is not a valid suit. Please input Heart, Diamond, Club, or Spade.", c_suit),
            }
            community_cards.push(card);
        }
        num_lines+=1;
    }
    match hand_odds(&hole_cards,&community_cards) {
        Ok(hash_map) => {
            let max_key_len = hash_map.keys().map(|k| k.len()).max().unwrap_or(0);
            for (key, value) in hash_map{
                println!("{:<width$}    {}%", key, format!("{:.4}", value*100.0), width = max_key_len);
            }
        }
        Err(err) => {
            panic!("Error: {}", err);
        }
    }


    let mut yes_to_com: i64 = 0;

    while yes_to_com<5{
        println!("Do you want to continue inputing community cards? y/n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.as_str().trim(){
            "y" => {},
            "n" => break,
            _ => {
                println!("Invalid input. Try again.");
                continue;
            },
        }
        let mut num_lines:i64 = 0;
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if(input == "done\n"){
            break;
        }
        let c: Vec<&str> = input.split(',').collect();
        for ele in c {
            let two: Vec<&str> = ele.split(' ').collect();
            if two.len()!=2{
                for ele in two{
                    println!("{}", ele);
                }
                panic!("Invalid Input! Invalid number of elements for one card.");
            }
            let c_suit: String = two[0].chars().filter(|c| !c.is_whitespace()).collect();
            let c_rank: String = two[1].chars().filter(|c| !c.is_whitespace()).collect();
            let mut card = Card {rank: 1,suit: Suit::Hearts,};
            let num: Result<u32, _> = c_rank.parse();
            match num {
                Ok(num) => {
                    card.rank = num;
                }
                Err(err) => {
                    panic!("Invalid Input! '{}' is not a number.", c_rank);
                }
            }
            match c_suit.as_str(){
                "Heart" => card.suit = Suit::Hearts,
                "Diamond" => card.suit = Suit::Diamonds,
                "Club" => card.suit = Suit::Clubs,
                "Spade" => card.suit = Suit::Spades,
                _ => panic!("Invalid Input! '{}' is not a valid suit. Please input Heart, Diamond, Club, or Spade.", c_suit),
            }
            community_cards.push(card);
        }
        num_lines+=1;
        match hand_odds(&hole_cards,&community_cards) {
            Ok(hash_map) => {
                let max_key_len = hash_map.keys().map(|k| k.len()).max().unwrap_or(0);
                for (key, value) in hash_map{
                    println!("{:<width$}:   {}", key, format!("{:.4}", value*100.0), width = max_key_len);
                }
            }
            Err(err) => {
                panic!("Error: {}", err);
            }
        }
        yes_to_com +=1;
        for card in &community_cards {
            println!("{}", card.rank);
        }
        
    }
}