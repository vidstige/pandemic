#[derive(Clone)]
pub struct FlatStack<T> {
    pub cards: Vec<T>,
}

pub trait Stack<T>: Sized {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn draw(&mut self) -> Option<T>;
    fn draw_bottom(&mut self) -> Option<T>;
    fn push(&mut self, card: T);
    fn discard_at(&mut self, index: usize, discard: &mut Self);
    fn discard(&mut self, card: &T, discard: &mut Self);
    fn split(&mut self, n: usize) -> Vec<Self>;
}

impl<T: PartialEq> Stack<T> for FlatStack<T> {
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    fn len(&self) -> usize {
        self.cards.len()
    }
    fn draw(&mut self) -> Option<T> {
        let last = self.cards.len().saturating_sub(1);
        return self.cards.drain(last..).next();
    }
    fn draw_bottom(&mut self) -> Option<T> {
        return self.cards.drain(0..1).next();
    }
    fn discard_at(&mut self, index: usize, discard: &mut Self) {
        discard.cards.push(self.cards.remove(index));
    }
    fn discard(&mut self, card: &T, discard: &mut Self) {
        let index = self.cards.iter().position(|c| c == card).unwrap();
        self.discard_at(index, discard);
    }
    fn push(&mut self, card: T) {
        self.cards.push(card);
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

pub fn empty_stack<T>() -> FlatStack<T> {
    return FlatStack{ cards: vec!() };
}

pub fn deal<T, S: Stack<T>>(deck: &mut S, hand: &mut S) {
    match deck.draw() {
        Some(card) => hand.push(card),
        None => (),
    }
}

// Stacks the source on top of target. Source will be empty after
pub fn stack<T, S: Stack<T>>(target: &mut S, source: &mut S) {
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