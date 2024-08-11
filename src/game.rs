use std::collections::HashMap;
use std::hash::Hash;

use crate::card::RessourceType;
use crate::get_and_check_userinput;
use crate::player::Player;

pub fn display_cards(player: &Player){
    for i in 0..player.get_hand().len(){
        print!("carte n°{} ", i);
        player.get_hand()[i].display();
    }
}

pub fn check_resourses_card(cost: Vec<RessourceType>, player: &Player){
    let cost_map = cost.iter().fold(HashMap::<RessourceType, u32>::new(), |mut acc, &x| {
        *acc.entry(x).or_default() += 1;
        acc
    });

}

pub fn choose_card(player: &Player){
    player.diplay_hand();
    println!("Choisie une carte: donne le numéro");
    let user_input: usize = get_and_check_userinput();
    loop{

        let user_input = player.get_hand().get(user_input - 1);

        match user_input {
            Some(_) => todo!(),
            None => continue,
        }
    }
}