use piston_window::{types::Color, Context, G2d, Key};
use rand::{thread_rng, Rng};

use crate::{
    draw::{draw_block, draw_rectangle},
    snake::{Direction, Snake},
};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAME_OVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 4,
            food_y: 6,
            width,
            height,
            game_over: false,
        }
    }
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        if direction.unwrap() == self.snake.head_direction().opposite() {
            return;
        }
        self.update_snake(direction);
    }
    pub fn draw(&self, context: &Context, graphic_buffer: &mut G2d) {
        self.snake.draw(context, graphic_buffer);
        if self.food_exists {
            draw_block(
                FOOD_COLOR,
                self.food_x,
                self.food_y,
                context,
                graphic_buffer,
            );
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, graphic_buffer);
        draw_rectangle(
            BORDER_COLOR,
            0,
            self.height - 1,
            self.width,
            1,
            context,
            graphic_buffer,
        );
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, graphic_buffer);
        draw_rectangle(
            BORDER_COLOR,
            self.width - 1,
            0,
            1,
            self.height,
            context,
            graphic_buffer,
        );
        if self.game_over {
            draw_rectangle(
                GAME_OVER_COLOR,
                0,
                0,
                self.width,
                self.height,
                context,
                graphic_buffer,
            );
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if !self.food_exists {
            self.add_food();
        }
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }
    pub fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }
    pub fn check_if_snake_alive(&mut self, direction: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(direction);
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        return next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1;
    }
    pub fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }
}
