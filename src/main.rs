#![allow(unused)]
#![warn(rust_2018_idioms)]

use piston_window::{
    clear, rectangle, EventLoop, Key, PressEvent, RenderArgs, RenderEvent, UpdateEvent,
};

mod snake;
use snake::{Direction, GameStateRunning};

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
    win_settings.set_resizable(true);
    win_settings.set_exit_on_esc(true);
    let mut window: piston_window::PistonWindow = win_settings.build().unwrap();
    window.set_ups(10);
    while let Some(event) = window.next() {
        if let Some(piston_window::Button::Keyboard(key)) = event.press_args() {
            if let Some(g) = &mut game {
                match key {
                    Key::Up => {
                        g.change_direction(Direction::Up);
                    }
                    Key::Down => {
                        g.change_direction(Direction::Down);
                    }
                    Key::Left => {
                        g.change_direction(Direction::Left);
                    }
                    Key::Right => {
                        g.change_direction(Direction::Right);
                    }
                    _ => {}
                }
            }
        }

        if let Some(up_args) = event.update_args() {
            if let Some(g) = game {
                game = g.tick();
            }
        }

        if let Some(ren_args) = event.render_args() {
            if let Some(g) = &game {
                window.draw_2d(&event, |_context, _g2d, _device| {
                    let vp = ren_args.viewport();
                    let box_width: f64 =
                        (f64::from(vp.draw_size[0]) / f64::from(g.get_field_size().0 + 1));
                    let box_height: f64 =
                        (f64::from(vp.draw_size[1]) / f64::from(g.get_field_size().1 + 1));
                    clear([0., 0., 1., 1.], _g2d);
                    let bg = rectangle::rectangle_by_corners(
                        1.,
                        1.,
                        f64::from(vp.draw_size[0] - 1),
                        f64::from(vp.draw_size[1] - 1),
                    );
                    piston_window::rectangle([1., 1., 1., 1.], bg, _context.transform, _g2d);

                    let food_rect = create_rect(g.get_food_pos(), (box_width, box_height));
                    piston_window::rectangle([0., 1., 0., 1.], food_rect, _context.transform, _g2d);

                    let head_rect = create_rect(g.get_head_pos(), (box_width, box_height));
                    piston_window::rectangle([1., 0., 0., 1.], head_rect, _context.transform, _g2d);
                    for t_pos in g.get_snake_tail() {
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
    }
}
