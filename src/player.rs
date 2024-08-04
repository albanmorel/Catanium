use std::collections::HashMap;

use crate::card::{Card, CardType, Cost, RessourceType};
use crate::wonders::{Wonder, Wonders};
use colored::Colorize;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Player {
    id: u32,
    hand: Vec<Card>,
    board: Vec<Card>,
    wonder: Wonder,
}
impl Player {
    pub fn new_player(id: u32, wonder: Wonder) -> Player {
        Player {
            id,
            hand: Vec::new(),
            board: Vec::new(),
            wonder,
        }
    }
    pub fn get_ressource(&self) -> HashMap<RessourceType, u32>{
        let ressource_card: Vec<&Card> = self.board.iter().filter(|card|
            match card.get_ressource_produced() {
                Some(_) => true,
                None => false,
            }).collect();

        let map = ressource_card.iter()
            .fold(HashMap::<RessourceType, u32>::new(), |mut acc, &x| {
                match x.get_ressource_produced() {
                    Some(ressource) => {*acc.entry(ressource).or_default() += 1},
                    None => {}
                }
                acc
            });
        map

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
    pub fn set_board(&mut self, card_list: Vec<Card>) {
        self.board = card_list;
    }
    pub fn add_card_to_board(&mut self, card: Card) {
        self.board.push(card);
    }
    pub fn get_board(&self) -> &Vec<Card> {
        &self.board
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

pub fn create_players(nb_of_player: u32, wonders: &mut Wonders) -> Vec<Player> {
    let mut player_list: Vec<Player> = Vec::new();
    for i in 0..nb_of_player {
        player_list.push(Player::new_player(i + 1, wonders.get_random_wonder()))
    }
    player_list
}