use crate::board::Board;

mod card;
mod game;
mod player;
mod board;

fn main() {
    let mut players_list = player::create_players(player::get_nb_of_player());

    let game_board = Board::create_board();
    card::give_cards(&mut players_list);
    for player in &players_list {
        game::display_cards(&player)
    }
    //println!("{:?}", get_nb_of_player());
}

pub fn get_and_check_userinput() -> usize{
    loop{

        let mut user_input= String::new();

        std::io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim().parse::<usize>() {
            Ok(user_input) => {
                user_input
            }
            Err(_) => {
                println!("ce n'es pas un nombre");
                continue;
            }
        };
    }
}
//Savoir combien de joueurs jouent
//Attribuer/Laisser choisir une couleur à chaque joueurs
//Générer la map -> Preset ou random
//Laisser les joueurs placer leurs pions chacun leur tour
//Creer un deck approprié au nombre de joueur
//Donner les ressources de départ correspondantes
//Jouer un tour
//Joueur lance les dés
//Attribuer ressource en fonction du résultat
//Session échange
//Construction
//Check points
//Quand 12 points, fin