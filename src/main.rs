use ggez::conf;
use game::settings;

use game::GameHandler;

pub mod game;
pub mod engine;

fn main() {

    let mut game_handler = GameHandler::new();

    let configuration = conf::Conf::default()
        .window_width(settings::SCREEN_SIZE.x)
        .window_height(settings::SCREEN_SIZE.y)
        .high_dpi(true);
    
    let _result = ggez::start(configuration, |ctx| {
        
        let window_size = ctx.quad_ctx.screen_size(); 
        game_handler.init(ctx, window_size.0, window_size.1); 
        
        Box::new(game_handler) 
    });
}


/*
getrandom = { version = "0.2", features = ["js"] }
wasm-bindgen = "0.2.80"
 */