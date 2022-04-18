use core::time;

use ggez::{Context};
use ggez::event::{EventHandler};
use ggez::graphics;
use ggez::timer;
use ggez::input::keyboard;


use crate::engine;

use snake::Snake;
use food::FoodHandler;
use grid::Grid;

mod grid;
mod grid_drawable;
pub mod settings;
mod snake;
mod food;


const SCREEN_COLOR: [f32; 4] = [0.6, 0.7, 0.6, 1.0];

struct Game {
    timer: time::Duration,
    command_blocked: bool,

    grid: Grid,

    snake: Snake,
    food_handler: FoodHandler,
}

pub struct GameHandler {
    game: Game,
    is_gameover: bool,

    window_dimentions: cgmath::Point2<f32>,
}


impl Game {
    pub fn new() -> Game {

        let grid = Grid::new(settings::SCREEN_SIZE, settings::GRID_SIZE);


        let start_position:cgmath::Point2<i32> = settings::GRID_SIZE.map(|c| (c as f32 / 2.0).floor() as i32);

        let snake = Snake::new(start_position, settings::SNAKE_LENGTH_START);
   
        Game {
            timer: time::Duration::new(0, 0),
            command_blocked: false,
            
            grid,
            snake,
            food_handler: FoodHandler::new(),
        }
    }

    fn update(&mut self, ctx: &mut ggez::Context) -> bool {

        let mut is_gameover = false;

        self.timer += timer::delta(ctx);

        if self.timer.as_secs_f32() >= settings::SNAKE_SPEED_SECONDS {

            is_gameover = self.snake.update();

            let snake = self.snake.get_objects();
            
            
            while self.food_handler.is_low() { self.food_handler.spawn(snake.iter()); }

            let food = self.food_handler.get_objects();

            let (is_collided, food_index) = self.snake.check_food(food.iter());
            
            if is_collided == true {
                for _ in 0..settings::FOOD_GROWTH { self.snake.create_body(); }
                self.food_handler.remove(food_index)
            }

            self.command_blocked = false;
            self.timer = time::Duration::from_secs(0);
        }
        
        is_gameover
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {

        let mut transition = None;
        if settings::SMOOTH_TRANSITION == true { transition = Some(self.timer.as_secs_f32() / settings::SNAKE_SPEED_SECONDS); }

        self.snake.update_objects(&self.grid, transition);
        self.food_handler.update_objects(&self.grid);


        let mut render_buffer: Vec<&engine::Object> = Vec::new();

        render_buffer.push(&self.grid.object);

        let snake = self.snake.get_objects();
        let food = self.food_handler.get_objects();

        for grid_drawable in snake.into_iter().chain(food.into_iter()) {
            render_buffer.push(&grid_drawable.object);
        }
        

        graphics::clear(ctx, graphics::Color::from(SCREEN_COLOR));
        
        engine::renderer::draw(ctx, render_buffer);

        graphics::present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {

        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height)).unwrap();

        self.grid = Grid::new(cgmath::point2(width as i32, height as i32), settings::GRID_SIZE);
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: keyboard::KeyCode, _keymods: keyboard::KeyMods, _repeat: bool){

            if self.command_blocked == true { return; }
        
            let new_direction = match keycode {
                keyboard::KeyCode::Up | keyboard::KeyCode::W => snake::SnakeDirection::Up,
                keyboard::KeyCode::Down | keyboard::KeyCode::S => snake::SnakeDirection::Down,
                keyboard::KeyCode::Left | keyboard::KeyCode::A => snake::SnakeDirection::Left,
                keyboard::KeyCode::Right | keyboard::KeyCode::D => snake::SnakeDirection::Right,
                
                _ => return,
            };

            let status = self.snake.change_direction(new_direction);

            if status == true { self.command_blocked = true; }
    }
}

impl GameHandler {
    pub fn new() -> GameHandler {
        GameHandler {
            game: Game::new(),
            is_gameover: false,

            window_dimentions: cgmath::Point2 { x: settings::SCREEN_SIZE.x as f32, y: settings::SCREEN_SIZE.y as f32 },
        }
    }

    pub fn init(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.resize_event(ctx, width, height);
    }
}

impl EventHandler for GameHandler {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {

        if self.is_gameover == false {
            self.is_gameover = self.game.update(ctx);
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        self.game.draw(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {

        self.window_dimentions = cgmath::point2(width, height);

        self.game.resize_event(ctx, width, height)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: keyboard::KeyCode, _keymods: keyboard::KeyMods, _repeat: bool){

        if self.is_gameover == true {
            if let keyboard::KeyCode::Space = keycode { 
            
                self.game = Game::new(); 
                self.game.resize_event(ctx, self.window_dimentions.x, self.window_dimentions.y);

                self.is_gameover = false;
            }
         }

        if let keyboard::KeyCode::Escape = keycode { ggez::event::quit(ctx); }

        self.game.key_down_event(ctx, keycode, _keymods, _repeat)
    }
}