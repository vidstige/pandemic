use rand::Rng;
mod pandemic;

use pandemic::State;
use pandemic::is_win;
use pandemic::valid_plys;
use pandemic::perform;

fn search(state: &State) -> i32 {
    if let Some(win) = is_win(state) {
        return if win { i32::MAX } else { i32::MIN };
    }

    for ply in valid_plys(state) {
        search(&perform(&state, ply));
    }
    return 0;
}

fn main() {
    let mut rgn = rand::thread_rng();
    let mut state = pandemic::create(3);
    pandemic::setup(&mut state);
    search(&state);
}
