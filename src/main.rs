const CITIES: usize = 48;
const DISEASES: usize = 4;

#[derive(Clone)]
enum Card {
    City(usize),
}

#[derive(Clone)]
struct Stack {
    cards: Vec<Card>,
}

fn empty() -> Stack {
    return Stack { cards: vec![] };
}
fn full() -> Stack {
    let cards: Vec<Card> = (0..CITIES).into_iter().map(|i| Card::City(i)).collect();
    return Stack { cards: cards };
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
        hands: vec![empty(); players],
        player_cards: full(),
        infection_cards: full(),
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
