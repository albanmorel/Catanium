use std::iter::repeat;
use std::string::ToString;
use crate::player::Player;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Card {
    name: String,
    card_type: CardType,
}

impl Card {
    pub fn new(name: String, card_type: CardType) -> Self {
        Self {
            name,
            card_type,
        }
    }

    pub fn display(&self) {
        println!("{}", self.name)
    }
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum CardType {
    Ressource { ressource_type: RessourceType },
    Developement { developement_type: DevelopementType },
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum DevelopementType {
    VictoryPoint {victory_point: i32 },
    Knight,
    Monopoly { ressource_type: RessourceType},
    FreeRoad,
    YearOfPlenty { ressource_type: (RessourceType, RessourceType)}
}
#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub enum RessourceType {
    Ore,
    Wood,
    Brick,
    Wheat,
    Sheep
}

fn get_cards(nb_player: usize) -> Vec<Card> {
    let test_card = Card::new(
        "yo test name là".to_string(),
        CardType::Ressource {
            ressource_type: RessourceType::Ore,
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