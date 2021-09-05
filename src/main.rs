use rand::Rng;
use rand::seq::SliceRandom;
mod pandemic;

use pandemic::State;
use pandemic::is_win;
use pandemic::valid_plys;
use pandemic::perform;

fn search(state: &State, rng: &mut impl Rng) -> i32 {
    if let Some(win) = is_win(state) {
        return if win { i32::MAX } else { i32::MIN };
    }

    let plys = valid_plys(state);
    plys.choose(rng);
    return 0;
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut state = pandemic::create(3);
    pandemic::setup(&mut state);
    search(&state, &mut rng);
}
