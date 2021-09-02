const DISEASES: usize = 4;

#[derive(Clone)]
enum PlayerCard {
    City(usize),
}

type InfectionCard = usize; // index to cities array

const CITIES: &[&str] = &[
  "Atlanta",
  "San Franscisco",
  "Chicago",
  "Montreál",
  "New York",
  "Washington",
  "Los Angeles",
  "Mexico City",
  "Miami",
  "Bogotá",
  "Lima",
  "São Paulo",
  "Santiago",
  "Buenos Aires",
  "London",
  "Essen",
  "St. Petersburg",
  "Madrid",
  "Paris",
  "Milan",
  "Algiers",
  "Istanbul",
  "Moscow",
  "Cairo",
  "Baghdad",
  "Tehran",
  "Riyadh",
  "Karachi",
  "Delhi",
  "Mumbai",
  "Kolkata",
  "Chennai",
  "Lagos",
  "Khartoum",
  "Kinshasa",
  "Johannesburg",
  "Bejing",
  "Seoul",
  "Shanghai",
  "Tokyo",
  "Bankok",
  "Hong Kong",
  "Taipei",
  "Osaka",
  "Jakarta",
  "Ho Chi Min City",
  "Manila",
  "Sidney",
];

#[derive(Clone)]
struct Stack<T> {
    cards: Vec<T>,
}

trait Drawable<T> {
    fn draw(&mut self) -> T;
}

impl Drawable<InfectionCard> for Stack<InfectionCard> {
    fn draw(&mut self) -> InfectionCard {
        let card = self.cards.drain((self.cards.len() - 1)..).next();
        match card {
            Some(card) => return card,
            None => return std::panic::panic_any(-1),
        }
    }
}

fn empty<T>() -> Stack<T> {
    return Stack { cards: vec![] };
}

fn full() -> Stack<InfectionCard> {
    let cards: Vec<InfectionCard> = (0..CITIES.len()).collect();
    return Stack { cards: cards };
}

struct Disease {
    cured: bool,
    eradicated: bool,
}

struct State {
    hands: Vec<Stack<PlayerCard>>,
    player_cards: Stack<PlayerCard>,
    player_discard: Stack<PlayerCard>,
    infection_cards: Stack<InfectionCard>,
    infection_discard: Stack<InfectionCard>,
    infection_rate: usize,
    diseases: [Disease; DISEASES],
    cubes: [u32; CITIES.len()],
    outbreaks: usize,
}

// Create an empty game state with unshuffled decks, etc.
fn create(players: usize) -> State  {
    let diseases = [
        Disease { cured: false, eradicated: false},
        Disease { cured: false, eradicated: false},
        Disease { cured: false, eradicated: false},
        Disease { cured: false, eradicated: false},
    ];
    return State {
        hands: vec![empty(); players],
        player_cards: empty(),
        player_discard: empty(),
        infection_cards: full(),
        infection_discard: empty(),
        infection_rate: 0,
        diseases: diseases,
        cubes: [0; CITIES.len()],
        outbreaks: 0,
     };
}

fn infect(state: &mut State, infection_card: InfectionCard) {
    state.cubes[infection_card] += 1;
}

fn setup(state: &mut State) {
    // Infect three cities with three cubes, three cities with two cubes and three cities
    //  with one cube
    for i in 0..3 {
        for _ in 0..3 {
            let infection_card = state.infection_cards.draw();
            for _ in 0..(3 - i) {
                infect(state, infection_card);
            }
        }
    }
}

fn main() {
    let mut state = create(3);
    setup(&mut state);
}
