use std::iter::repeat;

use crate::player::Player;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Card {
    name: String,
    cost: Cost,
    card_type: CardType,
}

impl Card {
    pub fn new(name: String, cost: Cost, card_type: CardType) -> Self {
        Self {
            name,
            cost,
            card_type,
        }
    }

    pub fn get_cost(&self) -> &Vec<RessourceType>{
        todo!()
    }

    pub fn get_ressource_produced(&self) -> Option<RessourceType> {
        match self.card_type {
            CardType::Gray{ressource_type} | CardType::Brown{ressource_type} => {
                return Some(ressource_type)
            },
            _ => {None}
        }
    }

    pub fn display(&self) {
        println!("{}", self.name)
    }
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum CardType {
    Blue { victory_point: u32 },
    Green { symbol: GreenSymbol },
    Yellow { gold_gain: u32 },
    Gray { ressource_type: RessourceType },
    Brown { ressource_type: RessourceType },
    Red { combat_point: u32 },
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum GreenSymbol {
    Gear,
    Tablet,
    Compass,
}
#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub enum RessourceType {
    Stone,
    Wood,
    Ore,
    Clay,
    Glass,
    Papyrus,
    Textile,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Cost(pub Vec<RessourceType>);
impl Cost {
    pub fn get_how_much_of_ressource(&self, ressources: RessourceType) -> u32{
        let mut i:u32 =0;
        for ressource in &self.0{
            if ressource == &ressources{
                i+=1;
            }
        }
        i
    }
}

fn get_cards(nb_player: usize) -> Vec<Card> {
    let test_card = Card::new(
        "yo test name là".to_string(),
        Cost(vec![]),
        CardType::Brown {
            ressource_type: RessourceType::Stone,
        },
    );

    repeat(test_card).take(nb_player * 7).collect()
}

pub fn give_cards(players: &mut Vec<Player>) {
    let mut card_list: Vec<Card> = get_cards(players.len());
    card_list.shuffle(&mut thread_rng());

    for player in players {
        player.set_hand(card_list.drain(0..6).collect())
    }
}