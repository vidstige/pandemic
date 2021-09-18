use std::collections::HashSet;
use itertools::Itertools;

use super::cards::{Deck, Hand, Stack, ComboStack, FlatStack};
use super::cards::{deal, empty_stack};
use super::cards;

const DISEASES: usize = 4;
// blue, yellow, black, red

pub type CityIndex = usize;
pub type DiseaseIndex = usize;

#[derive(Clone, PartialEq, Debug)]
pub enum PlayerCard {
    City(CityIndex),
    Epidemic,
}

const INFECTION_RATES: [usize; 7] = [2, 2, 2, 3, 3, 4, 4];

pub type InfectionCard = CityIndex; // index to cities array

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

pub fn city_name(city: CityIndex) -> &'static str {
    return CITY_NAMES[city];
}

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

fn infection_cards() -> ComboStack<InfectionCard> {
    let cards: Vec<InfectionCard> = (0..CITY_DISEASES.len()).collect();
    return ComboStack::new(cards);
}

fn player_cards() -> FlatStack<PlayerCard> {
    let cards: Vec<PlayerCard> = (0..CITY_DISEASES.len()).map(|i| PlayerCard::City(i)).collect();
    return FlatStack::new(cards);
}

#[derive(Clone)]
pub struct Player {
    hand: FlatStack<PlayerCard>,
    location: usize, // city index
}

#[derive(Clone)]
pub struct State {
    turn: usize,
    actions_taken: usize,
    players: Vec<Player>,
    player_cards: ComboStack<PlayerCard>,
    player_discard: FlatStack<PlayerCard>,
    infection_cards: ComboStack<InfectionCard>,
    infection_discard: FlatStack<InfectionCard>,
    infection_rate: usize,
    stations: HashSet<usize>,
    cured: [bool; DISEASES],
    cubes: [[usize; CITY_DISEASES.len()]; DISEASES], // cubes per disease
    outbreaks: usize,
}

fn city_by_name(name: &str) -> Option<CityIndex> {
    return CITY_NAMES.iter().position(|&city| city == name);
}

// Create an empty game state with unshuffled decks, etc.
pub fn create(players: usize) -> State  {
    let atlanta = city_by_name("Atlanta").unwrap();
    return State {
        turn: 0,
        actions_taken: 0,
        players: vec![Player { hand: empty_stack(), location: atlanta }; players],
        player_cards: ComboStack::from_flat(player_cards()),
        player_discard: empty_stack(),
        infection_cards: infection_cards(),
        infection_discard: empty_stack(),
        infection_rate: 0,
        stations: HashSet::new(),
        cured: [false; DISEASES],
        cubes: [[0; CITY_DISEASES.len()]; DISEASES],
        outbreaks: 0,
     };
}

// Recursively handles outbreaks
fn _infect(state: &mut State, disease: DiseaseIndex, city: CityIndex, outbreaked: &mut HashSet<CityIndex>) {
    if state.cubes[disease][city] < 3 {
        state.cubes[disease][city] += 1;
    } else {
        // If there already was an outbreak here, do nothing
        if outbreaked.contains(&city) {
            return;
        }
        outbreaked.insert(city);
        // Outbreak!
        println!("Outbreak!");
        
        // 1. Infect city with three cubes
        state.cubes[disease][city] = 3;
        // 2. Infect all neighbours with one cube
        for neighbour in neighbours(&map(), city) {
            _infect(state, disease, neighbour, outbreaked);
        }
        // 3. Increase outbreak counter
        state.outbreaks += 1;
    }
}

fn infect(state: &mut State, infection_card: InfectionCard, n: usize) {
    let disease = CITY_DISEASES[infection_card];
    let mut outbreaked = HashSet::new();
    for _ in 0..n {
        _infect(state, disease, infection_card, &mut outbreaked);
    }
}

pub fn setup(state: &mut State, epidemic_cards: usize) {
    // Deal player cards
    let n = 6 - state.players.len();
    for player in &mut state.players {
        for _ in 0..n {
            deal(&mut state.player_cards, &mut player.hand);
        }
    }

    let mut epidemic_stack = FlatStack::new(vec![PlayerCard::Epidemic; epidemic_cards]);
    // Insert epidemic cards
    let mut stacks = FlatStack::split(&mut state.player_cards.flatten(), epidemic_stack.len());
    for stack in &mut stacks {
        deal(&mut epidemic_stack, stack);
    }
    state.player_cards = cards::combine(stacks);

    // Infect three cities with three cubes, three cities with two cubes and three cities
    // with one cube
    for i in 0..3 {
        for _ in 0..3 {
            let infection_card = state.infection_cards.draw().unwrap();
            infect(state, infection_card, 3 - i);
        }
    }
}

fn cubes_of(state: &State, disease: DiseaseIndex) -> usize {
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
    
    // TODO: buggy - add end turn ply instead of using 3 here
    if state.actions_taken >= 3 {  // end of turn
        if state.player_cards.is_empty() {
            return Some(false);
        }        
    }

    // Players win if all diseases are cured
    if state.cured.iter().all(|&b| b) {
        return Some(true);
    }

    return None;
}

pub enum Ply {
    Discard(usize, usize), // player index & card index
    Drive(CityIndex),
    DirectFlight(CityIndex),
    CharteredFlight(CityIndex),
    Treat(DiseaseIndex, CityIndex),
    Cure(DiseaseIndex, [PlayerCard; 5]),
    Construct(CityIndex),
}

fn epidemic(state: &mut State) {
    println!("Epidemic!");

    // 1. Draw bottom card
    // TODO: What to do if infection deck is empty?
    let card = state.infection_cards.draw_bottom().unwrap();

    // 2. Infect with three cubes
    infect(state, card, 3);

    // 3. Discard
    state.infection_discard.push(card);

    // 4. Add infection discard pile to infection deck
    state.infection_cards.stack(&mut state.infection_discard);
}

pub fn perform(state: &mut State, ply: &Ply) {
    let player_index = current_player_index(state);
    match ply {
        Ply::Discard(player_index, card_index) => {
            state.players[*player_index].hand.discard_at(*card_index, &mut state.player_discard);
        }
        Ply::Drive(city) => state.players[player_index].location = *city,
        Ply::Treat(disease, city) => {
            state.cubes[*disease][*city] -= 1;
        }
        Ply::Construct(city) => {
            let hand = &mut state.players[player_index].hand;
            hand.discard(&PlayerCard::City(*city), &mut state.player_discard);
            state.stations.insert(*city);
        }
        Ply::Cure(disease, cards) => {
            // Discard five cards
            let hand = &mut state.players[player_index].hand;
            for card in cards {
                hand.discard(card, &mut state.player_discard);
            }
            state.cured[*disease] = true;
        }
        _ => (),
    }
    state.actions_taken += 1;
    
    if state.actions_taken >= 4 {
        // 1. Draw two player cards
        for _ in 0..2 {
            let hand = &mut state.players[player_index].hand;
            match state.player_cards.draw() {
                Some(card) => 
                    match card {
                        PlayerCard::City(_) => hand.push(card),
                        PlayerCard::Epidemic => epidemic(state),
                    },
                None => (),
            }
        }
        
        // 2. Draw infection cards
        for _ in 0..INFECTION_RATES[state.infection_rate] {
            // TODO: What if we run out of infection cards?
            let infection_card = state.infection_cards.draw().unwrap(); 
            println!("Infection in {:?}", CITY_NAMES[infection_card]);
            infect(state, infection_card, 1);
            state.infection_discard.push(infection_card);
        }

        // 3. Get ready for next turn
        state.turn += 1;
        state.actions_taken = 0;
    }
}

fn current_player_index(state: &State) -> usize {
    return state.turn % state.players.len();
}

fn neighbours(tm: &TravelMatrix, city: CityIndex) -> Vec<CityIndex> {
    let mut c = vec![];
    for i in 0..CITY_DISEASES.len() {
        if tm[city][i] {
            c.push(i);
        }
    }
    return c;
}

fn matches_disease(player_card: &PlayerCard, disease: DiseaseIndex) -> bool {
    match player_card {
        PlayerCard::City(city) => CITY_DISEASES[*city] == disease,
        PlayerCard::Epidemic => false,
    }
}

pub fn valid_plys(state: &State) -> Vec<Ply> {
    let mut plys = vec![];
    // if ANY player have more than 7 cards - they need to discard
    for (player_index, player) in state.players.iter().enumerate() {
        let n = player.hand.len();
        if n > 7 {
            return (0..n).map(|i| Ply::Discard(player_index, i)).collect();
        }        
    }
    
    // movement
    let map = map();
    let player = &state.players[current_player_index(state)];
    for neighbour in neighbours(&map, player.location) {
        plys.push(Ply::Drive(neighbour));
    }

    // treat
    for disease in 0..DISEASES {
        if state.cubes[disease][player.location] > 0 {
            plys.push(Ply::Treat(disease, player.location));
        }
    }

    // build station
    for card in player.hand.cards() {
        if let PlayerCard::City(city) = card {
            if player.location == *city {
                plys.push(Ply::Construct(*city));
            }
        }
    }

    // cure disease, if at station
    if state.stations.contains(&player.location) {
        for disease in 0..DISEASES {
            if !state.cured[disease] {
                // Iterate through all possible ways to select five cards
                // from of the same color from the hand
                let combinations = player.hand.cards()
                    .iter()
                    .filter(|&card| matches_disease(card, disease))
                    .combinations(5);
                for combination in combinations {
                    let cards: [PlayerCard; 5] = [
                        combination[0].clone(),
                        combination[1].clone(),
                        combination[2].clone(),
                        combination[3].clone(),
                        combination[4].clone(),
                    ];
                    plys.push(Ply::Cure(disease, cards));
                }
            }
        }
    }
    return plys;
}
