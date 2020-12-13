mod vertex;
mod gfx_engine;
use gfx_engine::Gfx;
mod controller;
mod physics_engine;
use physics_engine::Physics;

mod game;
use game::Game;

fn init_gfx_data(gfx: &mut Gfx) {
    // TEXTURES
    gfx.add_texture(&"assets/living_ball2.png");
    gfx.add_texture(&"assets/living_ball.png");
    // ANIMATION SETS
    let mut animation = gfx_engine::Animation::new();
    animation.auto_split_4();
    gfx.add_animation_set(vec!(animation));
}

fn init_game_data(game: &mut Game) {
    game.create_object(0, 0, (1., 1.));
    for _i in 0..500 {
        game.create_object(1, 0, (0., 0.));
    }
}

fn main() {
    let mut gfx = Gfx::new();
    init_gfx_data(&mut gfx);
    let physics = Physics::new();
    let mut game = Game::new(gfx, physics);
    init_game_data(&mut game);
    game.run();
}

// TODO MAYBE pass all textures once to GPU + an indice in the vertices

