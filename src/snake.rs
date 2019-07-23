use rand::Rng;

#[derive(Debug)]
pub struct GameStateRunning {
    rng: rand::rngs::ThreadRng,
    snake: Snake,
    field_size: (u8, u8),
    food_pos: (u8, u8),
}
impl GameStateRunning {
    pub fn new() -> Self {
        let field_size = (20, 20);
        let snake = Snake {
            moving_direction: Direction::Right,
            head_pos: (0, 0),
            tail: std::collections::VecDeque::new(),
        };
        let mut rng = rand::thread_rng();
        let food_pos = (
            rng.gen_range(0, field_size.0 + 1),
            rng.gen_range(0, field_size.1 + 1),
        );
        Self {
            rng,
            snake,
            field_size,
            food_pos,
        }
    }

    pub fn get_snake_tail(&self) -> impl std::iter::Iterator<Item = &(u8, u8)> {
        self.snake.tail.iter()
    }

    pub fn get_head_pos(&self) -> (u8, u8) {
        self.snake.head_pos
    }

    pub fn get_food_pos(&self) -> (u8, u8) {
        self.food_pos
    }

    pub fn get_field_size(&self) -> (u8, u8) {
        (self.field_size.0, self.field_size.1)
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Down if self.snake.moving_direction != Direction::Up => {
                self.snake.moving_direction = direction;
            }
            Direction::Up if self.snake.moving_direction != Direction::Down => {
                self.snake.moving_direction = direction;
            }
            Direction::Left if self.snake.moving_direction != Direction::Right => {
                self.snake.moving_direction = direction;
            }
            Direction::Right if self.snake.moving_direction != Direction::Left => {
                self.snake.moving_direction = direction;
            }
            _ => {}
        }
    }
    pub fn tick(mut self) -> Option<GameStateRunning> {
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
            // Snakes head is still on the field.
            if self.snake.tail.contains(&pos) {
                // The touched the body
                eprintln!("Head touched the body");
                dbg!(&self);
                return None;
            }
            self.snake.tail.push_front(self.snake.head_pos);
            self.snake.head_pos = pos;
        } else {
            eprintln!("Head is poistioned outside the field");
            dbg!(&self);
            return None;
        }
        if self.snake.head_pos == self.food_pos {
            let new_food_pos = (
                self.rng.gen_range(0, self.field_size.0 + 1),
                self.rng.gen_range(0, self.field_size.1 + 1),
            );
            self.food_pos = new_food_pos;
        } else {
            self.snake.tail.pop_back();
        }

        Some(self)
    }
}
#[derive(Debug, PartialEq)]
pub enum Direction {
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
