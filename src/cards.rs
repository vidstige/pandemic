// Stack - push is_empty, etc
// Deck - draw, split
// Hand - iterate, discard to deck

pub trait Stack<T>: Sized {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn push(&mut self, card: T);
}

pub trait Deck<T>: Stack<T> {
    fn draw(&mut self) -> Option<T>;
    fn draw_bottom(&mut self) -> Option<T>;
    fn split(&mut self, n: usize) -> Vec<Self>;
}

pub trait Hand<T>: Stack<T> {
    fn cards(&self) -> &Vec<T>;
    fn discard_at(&mut self, index: usize, discard: &mut Self);
    fn discard(&mut self, card: &T, discard: &mut Self);
}

// Flatstack

#[derive(Clone)]
pub struct FlatStack<T> {
    cards: Vec<T>,
}
impl<T> FlatStack<T> {
    pub fn new(cards: Vec<T>) -> FlatStack<T> {
        FlatStack { cards: cards }
    }
}

impl<T: PartialEq> Stack<T> for FlatStack<T> {
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    fn len(&self) -> usize {
        self.cards.len()
    }
    fn push(&mut self, card: T) {
        self.cards.push(card);
    }
}

impl<T: PartialEq> Deck<T> for FlatStack<T> {
    fn draw(&mut self) -> Option<T> {
        let last = self.cards.len().saturating_sub(1);
        return self.cards.drain(last..).next();
    }
    fn draw_bottom(&mut self) -> Option<T> {
        return self.cards.drain(0..1).next();
    }
    fn split(&mut self, n: usize) -> Vec<Self> {
        let mut stacks = vec!();
        let chunk_size = self.cards.len() / n;
        while !self.cards.is_empty() {
            stacks.push(Self { cards: self.cards.drain(0..chunk_size).collect() });
        }
        return stacks;
    }
}

impl<T: PartialEq> Hand<T> for FlatStack<T> {
    fn cards(&self) -> &Vec<T> {
        &self.cards
    }
    fn discard_at(&mut self, index: usize, discard: &mut Self) {
        discard.cards.push(self.cards.remove(index));
    }
    fn discard(&mut self, card: &T, discard: &mut Self) {
        let index = self.cards.iter().position(|c| c == card).unwrap();
        self.discard_at(index, discard);
    }
}

pub fn empty_stack<T>() -> FlatStack<T> {
    return FlatStack{ cards: vec!() };
}

// Generic functions
pub fn deal<T, S: Deck<T>>(deck: &mut S, hand: &mut S) {
    match deck.draw() {
        Some(card) => hand.push(card),
        None => (),
    }
}

// Stacks the source on top of target. Source will be empty after
pub fn stack<T, S: Deck<T>>(target: &mut S, source: &mut S) {
    while !source.is_empty() {
        target.push(source.draw().unwrap());
    }
}

pub fn combine<T>(stacks: &mut Vec<FlatStack<T>>) -> FlatStack<T> {
    let mut combined = FlatStack { cards: vec!() };
    for stack in stacks {
        combined.cards.append(&mut stack.cards);
    }
    return combined;
}