use rand::Rng;
use rand::seq::SliceRandom;
use std::fmt;

mod pandemic;

use pandemic::State;
use pandemic::Ply;
use pandemic::is_win;
use pandemic::valid_plys;
use pandemic::perform;


impl fmt::Display for Ply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Ply::Drive(city) => write!(f, "Drive to {:?}", city),
            Ply::DirectFlight(city) => write!(f, "Direct flight to {:?}", city),
            Ply::CharteredFlight(city) => write!(f, "Chartered flight to {:?}", city),
            Ply::Treat(disease, city) => write!(f, "Treat {:?} in {:?}", disease, city),
            Ply::Cure(disease) => write!(f, "Cure {:?}", disease),
            Ply::Construct(city) => write!(f, "Construct station in {:?}", city),
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
    let mut rng = rand::thread_rng();
    let mut state = pandemic::create(3);
    pandemic::setup(&mut state);
    playout(&state, &mut rng);
}
