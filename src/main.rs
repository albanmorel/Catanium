use std::vec::Vec;
mod card;
mod game;
mod player;
mod wonders;

fn main() {
    let mut wonders = wonders::get_wonders();
    let mut players_list = player::create_players(player::get_nb_of_player(), &mut wonders);

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
//Creer un deck approprié au nombre de joueur
//Attribuer une merveille à chaque joueurs
//récupérer les cartes
//Distribuer les cartes aux joueurs
//Jouer un tour
//Chaque joueurs selectionne une carte
//Vérifier s'il peut la jouer
//Jouer simultanément toutes les cartes et resoudres leurs effets
//Passer la main de chaque joueur à son voisin de droite ou gauche
//Quand plus qu'une carte par main
//defausse
//Résoudre les combats
//distribuer carte de l'âge d'après
//Quand toutes les cartes de tous les âges ont été jouées, compter les points et nommer le gagnant