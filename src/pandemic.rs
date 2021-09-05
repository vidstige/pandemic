const DISEASES: usize = 4;
// blue, yellow, black, red

#[derive(Clone)]
enum PlayerCard {
    City(usize),
}

pub type InfectionCard = usize; // index to cities array

const CITY_DISEASES: [usize; 48] = [
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    1,
    1,
    1,
    1,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
];

const CITY_NAMES: &[&str] = &[
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
pub struct Stack<T> {
    cards: Vec<T>,
}

trait Drawable<T> {
    fn draw(&mut self) -> Option<T>;
}

impl Drawable<InfectionCard> for Stack<InfectionCard> {
    fn draw(&mut self) -> Option<InfectionCard> {
        return self.cards.drain((self.cards.len() - 1)..).next();
    }
}

impl Drawable<PlayerCard> for Stack<PlayerCard> {
    fn draw(&mut self) -> Option<PlayerCard> {
        return self.cards.drain((self.cards.len() - 1)..).next();
    }
}

fn empty<T>() -> Stack<T> {
    return Stack { cards: vec![] };
}

fn full() -> Stack<InfectionCard> {
    let cards: Vec<InfectionCard> = (0..CITY_DISEASES.len()).collect();
    return Stack { cards: cards };
}

fn player_cards() -> Stack<PlayerCard> {
    let cards: Vec<PlayerCard> = (0..CITY_DISEASES.len()).map(|i| PlayerCard::City(i)).collect();
    return Stack { cards: cards }; 
}

#[derive(Clone)]
pub struct Player {
    hand: Stack<PlayerCard>,
    location: usize, // city index
}

#[derive(Clone)]
pub struct State {
    turn: usize,
    actions_taken: usize,
    players: Vec<Player>,
    player_cards: Stack<PlayerCard>,
    player_discard: Stack<PlayerCard>,
    infection_cards: Stack<InfectionCard>,
    infection_discard: Stack<InfectionCard>,
    infection_rate: usize,
    cured: [bool; DISEASES],
    cubes: [[usize; CITY_DISEASES.len()]; DISEASES], // cubes per disease
    outbreaks: usize,
}

fn city_by_name(name: &str) -> Option<usize> {
    return CITY_NAMES.iter().position(|&city| city == name);
}

// Create an empty game state with unshuffled decks, etc.
pub fn create(players: usize) -> State  {
    let atlanta = city_by_name("Atlanta").unwrap();
    return State {
        turn: 0,
        actions_taken: 0,
        players: vec![Player { hand: empty(), location: atlanta }; players],
        player_cards: player_cards(),
        player_discard: empty(),
        infection_cards: full(),
        infection_discard: empty(),
        infection_rate: 0,
        cured: [false; DISEASES],
        cubes: [[0; CITY_DISEASES.len()]; DISEASES],
        outbreaks: 0,
     };
}

fn infect(state: &mut State, infection_card: InfectionCard) {
    let disease = CITY_DISEASES[infection_card];
    state.cubes[disease][infection_card] += 1;
}

fn deal(deck: &mut Stack<PlayerCard>, hand: &mut Stack<PlayerCard>) {
    match deck.draw() {
        Some(card) => hand.cards.push(card),
        None => (),
    }
}

pub fn setup(state: &mut State) {
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
            let infection_card = state.infection_cards.draw().unwrap();
            for _ in 0..(3 - i) {
                infect(state, infection_card);
            }
        }
    }
}

fn cubes_of(state: &State, disease: usize) -> usize {
    return state.cubes[disease].iter().sum();
}

// Returns none if the game is not over, otherwise true for win and false for loose
pub fn is_win(state: &State) -> Option<bool> {
    // Players loose if there are more than 7 outbreaks
    if state.outbreaks > 7 {
        return Some(false);
    }
    // Players loose if any cube pile runs out
    for disease in 0..4 {
        if cubes_of(state, disease) > 24 {
            return Some(false);
        }
    }
    // TODO: Players loose if player card is empty at end of turn

    // Players win if all diseases are cured
    if state.cured.iter().all(|&b| b) {
        return Some(true);
    }

    return None;
}

pub enum Ply {
    Drive(usize),
    DirectFlight(usize),
    CharteredFlight(usize),
    Treat(usize),
}

pub fn perform(state: &mut State, ply: &Ply) {
}

fn player_index(state: &State) -> usize {
    return state.turn % state.players.len();
}

pub fn valid_plys(state: &State) -> Vec<Ply> {
    return vec![];
}