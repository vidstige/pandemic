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

impl Drawable<PlayerCard> for Stack<PlayerCard> {
    fn draw(&mut self) -> PlayerCard {
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

fn player_cards() -> Stack<PlayerCard> {
    let cards: Vec<PlayerCard> = (0..CITIES.len()).map(|i| PlayerCard::City(i)).collect();
    return Stack { cards: cards }; 
}

#[derive(Clone)]
struct Player {
    hand: Stack<PlayerCard>,
    location: usize, // city index
}

struct State {
    players: Vec<Player>,
    player_cards: Stack<PlayerCard>,
    player_discard: Stack<PlayerCard>,
    infection_cards: Stack<InfectionCard>,
    infection_discard: Stack<InfectionCard>,
    infection_rate: usize,
    cured: [bool; DISEASES],
    cubes: [u32; CITIES.len()],
    outbreaks: usize,
}

fn city_by_name(name: &str) -> Option<usize> {
    return CITIES.iter().position(|&city| city == name);
}

// Create an empty game state with unshuffled decks, etc.
fn create(players: usize) -> State  {
    let atlanta = city_by_name("Atlanta").unwrap();
    return State {
        players: vec![Player { hand: empty(), location: atlanta }; players],
        player_cards: player_cards(),
        player_discard: empty(),
        infection_cards: full(),
        infection_discard: empty(),
        infection_rate: 0,
        cured: [false; DISEASES],
        cubes: [0; CITIES.len()],
        outbreaks: 0,
     };
}

fn infect(state: &mut State, infection_card: InfectionCard) {
    state.cubes[infection_card] += 1;
}

fn deal(deck: &mut Stack<PlayerCard>, hand: &mut Stack<PlayerCard>) {
    let card = deck.draw();
    hand.cards.push(card);
}

fn setup(state: &mut State) {
    // Shuffle player cards

    // Deal player cards
    let n = 6 - state.players.len();
    for player in &mut state.players {
        for _ in 0..n {
            deal(&mut state.player_cards, &mut player.hand);
        }
    }

    // Infect three cities with three cubes, three cities with two cubes and three cities
    // with one cube
    for i in 0..3 {
        for _ in 0..3 {
            let infection_card = state.infection_cards.draw();
            for _ in 0..(3 - i) {
                infect(state, infection_card);
            }
        }
    }
}

fn search(state: &State) {
    
}

fn main() {
    let mut state = create(3);
    setup(&mut state);
}
