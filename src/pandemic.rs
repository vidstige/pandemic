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

type TravelMatrix = [[bool; CITY_DISEASES.len()]; CITY_DISEASES.len()];

fn connect(tm: &mut TravelMatrix, name0: &str, name1: &str) {
    let city0 = city_by_name(name0).unwrap();
    let city1 = city_by_name(name1).unwrap();
    tm[city0][city1] = true;
    tm[city1][city0] = true;
}

pub fn map() -> TravelMatrix {
    let mut tm = [[false; CITY_DISEASES.len()]; CITY_DISEASES.len()];
    connect(&mut tm, "Atlanta", "Chicago");
    connect(&mut tm, "Atlanta", "Washington");
    connect(&mut tm, "Atlanta", "Miami");
    connect(&mut tm, "Chicago", "San Franscisco");
    connect(&mut tm, "Chicago", "Los Angeles");
    connect(&mut tm, "Chicago", "Mexico City");
    connect(&mut tm, "Chicago", "Montreál");
    connect(&mut tm, "San Franscisco", "Los Angeles");
    connect(&mut tm, "San Franscisco", "Tokyo");
    connect(&mut tm, "San Franscisco", "Manila");
    connect(&mut tm, "Los Angeles", "Sidney");
    connect(&mut tm, "Los Angeles", "Mexico City");
    connect(&mut tm, "Mexico City", "Miami");
    connect(&mut tm, "Mexico City", "Bogotá");
    connect(&mut tm, "Mexico City", "Lima");
    connect(&mut tm, "Montreál", "Washington");
    connect(&mut tm, "Montreál", "New York");
    connect(&mut tm, "Washington", "New York");
    connect(&mut tm, "Washington", "Miami");
    connect(&mut tm, "Miami", "Bogotá");
    connect(&mut tm, "Miami", "Bogotá");
    connect(&mut tm, "Bogotá", "Lima");
    connect(&mut tm, "Bogotá", "Buenos Aires");
    connect(&mut tm, "Bogotá", "São Paulo");
    connect(&mut tm, "Lima", "Santiago");
    connect(&mut tm, "Buenos Aires", "São Paulo");
    connect(&mut tm, "São Paulo", "Madrid");
    connect(&mut tm, "São Paulo", "Lagos");
    connect(&mut tm, "Lagos", "Kinshasa");
    connect(&mut tm, "Lagos", "Khartoum");
    connect(&mut tm, "Kinshasa", "Khartoum");
    connect(&mut tm, "Kinshasa", "Johannesburg");
    connect(&mut tm, "Johannesburg", "Khartoum");
    connect(&mut tm, "New York", "London");
    connect(&mut tm, "New York", "Madrid");
    connect(&mut tm, "London", "Essen");
    connect(&mut tm, "London", "Paris");
    connect(&mut tm, "London", "Madrid");
    connect(&mut tm, "Madrid", "Paris");
    connect(&mut tm, "Madrid", "Algiers");
    connect(&mut tm, "Paris", "Essen");
    connect(&mut tm, "Paris", "Milan");
    connect(&mut tm, "Paris", "Algiers");
    connect(&mut tm, "Essen", "St. Petersburg");
    connect(&mut tm, "Essen", "Milan");
    connect(&mut tm, "Milan", "Istanbul");
    connect(&mut tm, "St. Petersburg", "Istanbul");
    connect(&mut tm, "St. Petersburg", "Moscow");
    connect(&mut tm, "Istanbul", "Moscow");
    connect(&mut tm, "Istanbul", "Algiers");
    connect(&mut tm, "Istanbul", "Cairo");
    connect(&mut tm, "Istanbul", "Baghdad");
    connect(&mut tm, "Algiers", "Cairo");
    connect(&mut tm, "Cairo", "Khartoum");
    connect(&mut tm, "Cairo", "Riyadh");
    connect(&mut tm, "Cairo", "Baghdad");
    connect(&mut tm, "Riyadh", "Baghdad");
    connect(&mut tm, "Riyadh", "Karachi");
    connect(&mut tm, "Moscow", "Tehran");
    connect(&mut tm, "Baghdad", "Tehran");
    connect(&mut tm, "Baghdad", "Karachi");
    connect(&mut tm, "Tehran", "Delhi");
    connect(&mut tm, "Tehran", "Karachi");
    connect(&mut tm, "Karachi", "Delhi");
    connect(&mut tm, "Karachi", "Mumbai");
    connect(&mut tm, "Delhi", "Mumbai");
    connect(&mut tm, "Delhi", "Chennai");
    connect(&mut tm, "Delhi", "Kolkata");
    connect(&mut tm, "Mumbai", "Chennai");
    connect(&mut tm, "Kolkata", "Chennai");
    connect(&mut tm, "Kolkata", "Bankok");
    connect(&mut tm, "Kolkata", "Hong Kong");
    connect(&mut tm, "Chennai", "Bankok");
    connect(&mut tm, "Chennai", "Jakarta");
    connect(&mut tm, "Bankok", "Jakarta");
    connect(&mut tm, "Bankok", "Ho Chi Min City");
    connect(&mut tm, "Bankok", "Hong Kong");
    connect(&mut tm, "Jakarta", "Ho Chi Min City");
    connect(&mut tm, "Jakarta", "Sidney");
    connect(&mut tm, "Ho Chi Min City", "Hong Kong");
    connect(&mut tm, "Ho Chi Min City", "Manila");
    connect(&mut tm, "Hong Kong", "Shanghai");
    connect(&mut tm, "Hong Kong", "Manila");
    connect(&mut tm, "Hong Kong", "Taipei");
    connect(&mut tm, "Manila", "Sidney");
    connect(&mut tm, "Manila", "Taipei");
    connect(&mut tm, "Taipei", "Osaka");
    connect(&mut tm, "Taipei", "Shanghai");
    connect(&mut tm, "Shanghai", "Bejing");
    connect(&mut tm, "Shanghai", "Seoul");
    connect(&mut tm, "Shanghai", "Tokyo");
    connect(&mut tm, "Bejing", "Seoul");
    connect(&mut tm, "Seoul", "Tokyo");
    connect(&mut tm, "Tokyo", "Osaka");

    return tm; 
}

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
    let map = map();
    return vec![];
}
