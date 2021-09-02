use std::fmt;

const DISEASES: usize = 4;

#[derive(Clone)]
enum Card {
    City(usize),
}

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

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::City(i) => write!(f, "{}", CITIES[*i]),
        }
    }
}

#[derive(Clone)]
struct Stack {
    cards: Vec<Card>,
}
trait Drawable {
    fn draw(&mut self) -> Option<Card>;
}

impl Drawable for Stack {
    fn draw(&mut self) -> Option<Card> {
        return self.cards.drain((self.cards.len() - 1)..).next();
    }
}

fn empty() -> Stack {
    return Stack { cards: vec![] };
}
fn full() -> Stack {
    let cards: Vec<Card> = (0..CITIES.len()).into_iter().map(|i| Card::City(i)).collect();
    return Stack { cards: cards };
}

struct Disease {
    cured: bool,
    eradicated: bool,
}

struct State {
    hands: Vec<Stack>,
    player_cards: Stack,
    player_discard: Stack,
    infection_cards: Stack,
    infection_discard: Stack,
    diseases: [Disease; DISEASES],
    cubes: [u32; CITIES.len()],
    outbreaks: i32,
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
        player_cards: full(),
        player_discard: empty(),
        infection_cards: full(),
        infection_discard: empty(),
        diseases: diseases,
        cubes: [0; CITIES.len()],
        outbreaks: 0,
     };
}

fn setup(state: &mut State) {
    let card = state.infection_cards.draw();
    println!("{:?}", card);
}

fn main() {
    let mut state = create(3);
    setup(&mut state);
}
