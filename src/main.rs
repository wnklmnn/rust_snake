#![allow(unused)]
#![warn(rust_2018_idioms)]

use rand::Rng;
use piston_window::{
    clear, rectangle, EventLoop, Key, PressEvent, RenderArgs, RenderEvent, UpdateEvent,
};

#[derive(Debug)]
struct GameStateRunning {
    rng: rand::rngs::ThreadRng,
    snake: Snake,
    field_size: (u8, u8),
    food_pos: (u8, u8),
}
impl GameStateRunning{
    fn new() -> Self {
        let field_size = (20, 20);
        let snake = Snake {
            moving_direction: Direction::Right,
            head_pos: (0, 0),
            tail: std::collections::VecDeque::new(),
        };
        let mut rng = rand::thread_rng();
        let food_pos = (rng.gen_range(0, field_size.0 +1), rng.gen_range(0, field_size.1 +1));
        Self {
            rng,
            snake,
            field_size,
            food_pos,
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
                eprintln!("Kopf hat den Körper berührt");
                dbg!(&self);
                return None;
            }
            self.snake.tail.push_front(self.snake.head_pos);
            self.snake.head_pos = pos;
        } else {
            eprintln!("Kopf befindet sich außerhalb des Spielfelds");
            dbg!(&self);
            return None;
        }
        if self.snake.head_pos == self.food_pos {
            let new_food_pos = (self.rng.gen_range(0, self.field_size.0 +1), self.rng.gen_range(0, self.field_size.1 +1));
            self.food_pos = new_food_pos;
        } else {
            self.snake.tail.pop_back();
        }

        Some(self)
    }
}
#[derive(Debug, PartialEq)]
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

fn create_rect(point: (u8, u8), size: (f64, f64)) -> piston_window::types::Rectangle {
    rectangle::rectangle_by_corners(
        (f64::from(point.0) * size.0),
        (f64::from(point.1) * size.1),
        ((f64::from(point.0) * size.0) + size.0),
        ((f64::from(point.1) * size.1) + size.1),
    )
}

fn main() {
    let mut game = Some(GameStateRunning::new());
    let mut win_settings = piston_window::WindowSettings::new("MySnake", (200, 200));
    win_settings.set_resizable(false);
    let mut window: piston_window::PistonWindow = win_settings.build().unwrap();
    window.set_ups(5);
    while let Some(event) = window.next() {
        if let Some(piston_window::Button::Keyboard(key)) = event.press_args() {
            if let Some(g) = &mut game {
                match key {
                    Key::Up if g.snake.moving_direction != Direction::Down => {
                        g.change_direction(Direction::Up);
                    }
                    Key::Down if g.snake.moving_direction != Direction::Up=> {
                        g.change_direction(Direction::Down);
                    }
                    Key::Left if g.snake.moving_direction != Direction::Right => {
                        g.change_direction(Direction::Left);
                    }
                    Key::Right if g.snake.moving_direction != Direction::Left => {
                        g.change_direction(Direction::Right);
                    }
                    _ => {}
                }
            }
        }

        if let Some(ren_args) = event.render_args() {
            if let Some(g) = &game {
                window.draw_2d(&event, |_context, _g2d, _device| {
                    let vp = ren_args.viewport();
                    let box_width: f64 =
                        (f64::from(vp.draw_size[0]) / f64::from(g.field_size.0 + 1));
                    let box_height: f64 =
                        (f64::from(vp.draw_size[1]) / f64::from(g.field_size.1 + 1));
                    clear([1., 1., 1., 1.], _g2d);

                    let food_rect = create_rect(
                        (g.food_pos.0, g.food_pos.1),
                        (box_width, box_height),
                    );
                    piston_window::rectangle([0., 1., 0., 1.], food_rect, _context.transform, _g2d);

                    let head_rect = create_rect(
                        (g.snake.head_pos.0, g.snake.head_pos.1),
                        (box_width, box_height),
                    );
                    piston_window::rectangle([1., 0., 0., 1.], head_rect, _context.transform, _g2d);
                    for t_pos in g.snake.tail.iter() {
                        let head_rect = create_rect((t_pos.0, t_pos.1), (box_width, box_height));
                        piston_window::rectangle(
                            [0., 0., 0., 1.],
                            head_rect,
                            _context.transform,
                            _g2d,
                        );
                    }
                });
            }
        }
        if let Some(up_args) = event.update_args() {
            if let Some(g) = game {
                dbg!(&g.snake.moving_direction);
                game = g.tick();
            }
        }
    }
    /*while let Some(gs) = game.tick() {
        dbg!(&gs);
        eprintln!("Tick");
        game = gs;
    }
    eprintln!("Done");*/
}
