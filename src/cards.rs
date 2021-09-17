// Stack - push is_empty, etc
// Deck - draw
// Hand - iterate, discard to deck

pub trait Stack<T>: Sized {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn push(&mut self, card: T);
}

pub trait Deck<T>: Stack<T> {
    fn draw(&mut self) -> Option<T>;
    fn draw_bottom(&mut self) -> Option<T>;
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
    pub fn split(stack: &mut FlatStack<T>, n: usize) -> Vec<FlatStack<T>> {
        let mut stacks = vec!();
        let chunk_size = stack.cards.len() / n;
        while !stack.cards.is_empty() {
            stacks.push(FlatStack { cards: stack.cards.drain(0..chunk_size).collect() });
        }
        return stacks;
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

// ComboStack
// Consist of several stacks stacked together. Allows for correct random draws of stacked
// stacks
pub struct ComboStack<T> {
  stacks: Vec<FlatStack<T>>,
}
impl<T> ComboStack<T> {
  pub fn new(stacks: Vec<FlatStack<T>>) -> ComboStack<T> {
      ComboStack { stacks: stacks }
  }
}

impl<T: PartialEq> Stack<T> for ComboStack<T> {
  fn is_empty(&self) -> bool {
      match self.stacks.last() {
          Some(stack) => stack.is_empty(),
          None => true
      }
  }
  fn len(&self) -> usize {
      self.stacks.iter().map(|stack| stack.len()).sum()
  }
  fn push(&mut self, card: T) {
      match self.stacks.last_mut() {
          Some(stack) => stack.push(card),
          None => self.stacks.push(FlatStack::new(vec![card])),
      }
  }
}

impl<T: PartialEq> Deck<T> for ComboStack<T> {
  fn draw(&mut self) -> Option<T> {
      match self.stacks.last_mut() {
          Some(stack) => stack.draw(),
          None => None
      }
  }
  fn draw_bottom(&mut self) -> Option<T> {
      if self.stacks.is_empty() {
          return None
      }
      self.stacks[0].draw_bottom()
  }
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