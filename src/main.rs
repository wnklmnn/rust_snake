#![allow(unused)]
#![warn(rust_2018_idioms)]

use piston_window::clear;

#[derive(Debug)]
struct GameStateRunning {
    snake: Snake,
    field_size: (u8, u8),
    food_pos: (u8, u8),
}
impl GameStateRunning {
    fn new() -> Self {
        let field_size = (3, 20);
        let snake = Snake {
            moving_direction: Direction::Right,
            head_pos: (0, 0),
            tail: std::collections::VecDeque::new(),
        };
        Self {
            snake,
            field_size,
            food_pos: (2, 0),
        }
    }
    fn change_direction(&mut self, direction: Direction) {
        self.snake.moving_direction = direction;
    }
    fn tick(mut self) -> Option<GameStateRunning> {
        let mut next_head_pos: Option<(u8, u8)> = match self.snake.moving_direction {
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
        if let Some(pos) = next_head_pos {
            // Der Kopf befeindet sich noch im Spielfeld
            if self.snake.tail.contains(&pos) {
                // Der Kopf hat eine Teil des Körpers berührt
                return None;
            }
            self.snake.tail.push_front(self.snake.head_pos);
            self.snake.head_pos = pos;
        } else {
            return None;
        }
        if self.snake.head_pos == self.food_pos {
            self.food_pos = (0, 0);
        } else {
            self.snake.tail.pop_back();
        }

        Some(self)
    }
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Snake {
    pub moving_direction: Direction,
    pub head_pos: (u8, u8),
    pub tail: std::collections::VecDeque<(u8, u8)>,
}

fn main() {
    let mut game = GameStateRunning::new();
    while let Some(gs) = game.tick() {
        dbg!(&gs);
        eprintln!("Tick");
        game = gs;
    }
    eprintln!("Done");
}
