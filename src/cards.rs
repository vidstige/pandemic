#[derive(Clone)]
pub struct Stack<T> {
    pub cards: Vec<T>,
}

pub trait Drawable<T> {
    fn draw(&mut self) -> Option<T>;
    fn draw_bottom(&mut self) -> Option<T>;
}

impl<T> Drawable<T> for Stack<T> {
    fn draw(&mut self) -> Option<T> {
        let last = self.cards.len().saturating_sub(1);
        return self.cards.drain(last..).next();
    }
    fn draw_bottom(&mut self) -> Option<T> {
        return self.cards.drain(0..1).next();
    }
}

pub fn deal<T>(deck: &mut Stack<T>, hand: &mut Stack<T>) {
    match deck.draw() {
        Some(card) => hand.cards.push(card),
        None => (),
    }
}

pub fn discard<T: std::cmp::PartialEq>(deck: &mut Stack<T>, card: &T, discard: &mut Stack<T>) {
    let index = deck.cards.iter().position(|c| c == card).unwrap();
    discard.cards.push(deck.cards.remove(index));
}

// Stacks the source on top of target. Source will be empty after
pub fn stack<T>(target: &mut Stack<T>, source: &mut Stack<T>) {
    target.cards.append(&mut source.cards);
}

pub fn split<T>(stack: &mut Stack<T>, n: usize) -> Vec<Stack<T>> {
    let mut stacks = vec!();
    let chunk_size = stack.cards.len() / n;
    while !stack.cards.is_empty() {
        stacks.push(Stack { cards: stack.cards.drain(0..chunk_size).collect() });
    }
    return stacks;
}

pub fn combine<T>(stacks: &mut Vec<Stack<T>>) -> Stack<T> {
    let mut combined = Stack { cards: vec!() };
    for stack in stacks {
        combined.cards.append(&mut stack.cards);
    }
    return combined;
}