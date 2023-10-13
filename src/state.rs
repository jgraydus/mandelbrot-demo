pub const CANVAS_HEIGHT: u32 = 600;
pub const CANVAS_WIDTH: u32 = 600;

#[derive(Debug)]
pub enum EventType { Click, Move, Reset }

#[derive(Clone,Copy)]
pub enum State { NotSelecting, Selecting }

#[derive(Clone,Debug)]
pub struct Rect<T> {
  pub top: T,
  pub bottom: T,
  pub left: T,
  pub right: T, 
}

