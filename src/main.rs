const CITIES: usize = 48;
const DISEASES: usize = 4;

struct Card {

}

#[derive(Clone)]
struct Stack {

}
fn empty() -> Stack {
    return Stack {};
}

struct Disease {
    cured: bool,
    eradicated: bool,
}

struct State {
    hands: Vec<Stack>,
    player_cards: Stack,
    infection_cards: Stack,
    infection_discard: Stack,
    diseases: [Disease; DISEASES],
    cubes: [u32; CITIES],
    outbreaks: i32,
}

fn initial(players: usize) -> State  {
    let diseases = [
        Disease { cured: false, eradicated: false},
        Disease { cured: false, eradicated: false},
        Disease { cured: false, eradicated: false},
        Disease { cured: false, eradicated: false},
    ];
    return State {
        hands: vec![Stack {}; players],
        player_cards: empty(),
        infection_cards: empty(),
        infection_discard: empty(),
        diseases: diseases,
        cubes: [0; CITIES],
        outbreaks: 0,
     };
}

fn main() {
    let state = initial(3);
    println!("Hello, world!");
}
