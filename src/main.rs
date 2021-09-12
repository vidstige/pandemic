use std::fmt;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

mod pandemic;
mod cards;

use pandemic::State;
use pandemic::Ply;
use pandemic::is_win;
use pandemic::valid_plys;
use pandemic::perform;
use pandemic::city_name;

impl fmt::Display for Ply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Ply::Discard(player_index, card_index) => write!(f, "Player {:?} discards cards #{:?}", player_index, card_index),
            Ply::Drive(city) => write!(f, "Drive to {:?}", city_name(city)),
            Ply::DirectFlight(city) => write!(f, "Direct flight to {:?}", city_name(city)),
            Ply::CharteredFlight(city) => write!(f, "Chartered flight to {:?}", city_name(city)),
            Ply::Treat(disease, city) => write!(f, "Treat {:?} in {:?}", disease, city_name(city)),
            Ply::Cure(disease, _) => write!(f, "Cure {:?}", disease),
            Ply::Construct(city) => write!(f, "Construct station in {:?}", city_name(city)),
       }
    }
}

fn playout(from: &State, rng: &mut impl Rng) -> i32 {
    let mut state = from.clone();

    while is_win(&state).is_none() {
        let plys = valid_plys(&state);
        let ply = plys.choose(rng).unwrap();
        println!("{}", ply);
        perform(&mut state, ply);
    }
    println!("outcome: {:?}", if is_win(&state).unwrap() { "win" } else { "loss" });

    return 0;
}

fn main() {
    let seed: [u8; 32] = [17; 32]; // fixed seed
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut state = pandemic::create(3);
    pandemic::setup(&mut state, 4);
    playout(&state, &mut rng);
}
