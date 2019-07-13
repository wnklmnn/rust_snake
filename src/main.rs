#![allow(unused)]
#![warn(rust_2018_idioms)]

use piston_window::clear;

struct GameState {
    snake: Snake,
    field_size: (u8, u8),
    food_pos: (u8, u8),
}
impl GameState {
    fn new() -> Self {
        let field_size = (20, 20);
        let snake = Snake {
            moving_direction: Direction::Right,
            head_pos: (0, 0),
            tail: Vec::new(),
        };
        GameState {
            snake,
            field_size,
            food_pos: (5, 5),
        }
    }
    fn tick(&mut self) {
        let next_head_pos: Option<(u8, u8)> = match self.snake.moving_direction {
            Direction::Down => {
                let ret;
                let y = self.snake.head_pos.1.checked_add(1);
                if let Some(y) = y {
                    if y <= self.field_size.1 {
                        ret = Some((self.snake.head_pos.0, y));
                    } else {
                        ret = None;
                    }
                } else {
                    ret = None
                }
                ret
            }
            Direction::Up => {
                let ret;
                let y = self.snake.head_pos.1.checked_sub(1);
                if let Some(y) = y {
                    ret = Some((self.snake.head_pos.0, y));
                } else {
                    ret = None
                }
                ret
            }
            Direction::Left => {
                let ret;
                let x = self.snake.head_pos.0.checked_sub(1);
                if let Some(x) = x {
                    ret = Some((x, self.snake.head_pos.1));
                } else {
                    ret = None
                }
                ret
            }
            Direction::Right => {
                let ret;
                let x = self.snake.head_pos.0.checked_add(1);
                if let Some(x) = x {
                    if x <= self.field_size.0 {
                        ret = Some((x, self.snake.head_pos.1));
                    } else {
                        ret = None;
                    }
                } else {
                    ret = None
                }
                ret
            }
        };
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    pub moving_direction: Direction,
    pub head_pos: (u8, u8),
    pub tail: Vec<(Direction, u8)>,
}

fn main() {
    let game = GameState::new();
}
