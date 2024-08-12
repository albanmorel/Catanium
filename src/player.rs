use crate::card::{Card, Cost};
use colored::Colorize;
use crate::board::{create_road, create_settlement, Road, Settlement};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Player {
    id: u32,
    hand: Vec<Card>,
    settlements: Vec<Settlement>,
    roads: Vec<Road>
}
impl Player {
    pub fn new_player(id: u32) -> Player {
        Player {
            id,
            hand: Vec::new(),
            settlements: vec![create_settlement(1);5].extend(vec![create_settlement(2);4]).collect() ,
            roads: vec![create_road();15]
        }
    }

    pub fn diplay_hand(&self){
        for card in &self.hand{
            card.display();
        }
    }

    pub fn set_hand(&mut self, card_list: Vec<Card>) {
        self.hand = card_list;
    }
    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }
}

pub fn get_nb_of_player() -> u32 {
    loop {
        println!("Combien de joueur meirveilleux qui jouent ?");
        let mut user_input: String = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim().parse::<u32>() {
            Ok(user_input) if user_input <= 7 => return user_input,
            Ok(_) => {
                println!(
                    "{}",
                    ("Yoo very desoled mais c'est 7 joueurs maximum :/ (C'est dans le titre du jeu, fais un effort)")
                        .italic()
                        .red()
                );
                continue;
            }
            Err(_) => {
                println!("{}", ("Yooo g pa kompri").italic().red());
                continue;
            }
        };
    }
}

pub fn create_players(nb_of_player: u32) -> Vec<Player> {
    let mut player_list: Vec<Player> = Vec::new();
    for i in 0..nb_of_player {
        player_list.push(Player::new_player(i + 1))
    }
    player_list
}