use rand::Rng;
use rand::seq::SliceRandom;
mod pandemic;

use pandemic::State;
use pandemic::is_win;
use pandemic::valid_plys;
use pandemic::perform;

fn playout(from: &State, rng: &mut impl Rng) -> i32 {
    let mut state = from.clone();

    while is_win(&state).is_none() {
        let plys = valid_plys(&state);
        let ply = plys.choose(rng).unwrap();
        perform(&mut state, ply);
    }

    return 0;
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut state = pandemic::create(3);
    pandemic::setup(&mut state);
    playout(&state, &mut rng);
}
