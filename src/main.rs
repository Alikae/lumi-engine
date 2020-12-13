use std::time::Instant;

mod vertex;
mod gfx_engine;
use gfx_engine::Gfx;
mod controller;
use controller::Controller;
mod physics_engine;
use physics_engine::{PhysicObject, Physics};

// Game Meta Data
pub struct Game {
    gfx:            Gfx,
    physics:        Physics,
    controller:     Controller,
    should_stop:    bool,
    start_time:     Instant,
    camera:         [f32; 3],
}

impl Game {
    fn physics_to_gfx(&mut self) {
        for i in 0..self.physics.objects.len() {
            self.gfx.animated_sprites[self.physics.objects[i].sprite_index].update_tess_pos(self.physics.objects[i].pos)
        }
    }
}

fn create_object(game: &mut Game, texture_index: usize, animation_set_index: usize, pos: (f32, f32)) {
    let sprite_index = game.gfx.add_animated_sprite(texture_index, animation_set_index);
    game.physics.objects.push(PhysicObject::new(pos, sprite_index));
}

fn init_gfx()-> Gfx {
    let mut gfx = Gfx::new();
    // TEXTURES
    gfx.add_texture(&"assets/living_ball2.png");
    gfx.add_texture(&"assets/living_ball.png");
    // ANIMATION SETS
    let mut animation = gfx_engine::Animation::new();
    animation.auto_split_4();
    gfx.add_animation_set(vec!(animation));
    gfx
}

fn main() {
    let mut game = Game {
        should_stop:    false,
        start_time:     Instant::now(),
        gfx:            init_gfx(),
        physics:        Physics::new(),
        controller:     Controller::new(),
        camera:         [1., 1., 0.5],
    };
    create_object(&mut game, 0, 0, (1., 1.));
    for _i in 0..500 {
        create_object(&mut game, 1, 0, (0., 0.));
    }
    let mut last_frame_time = game.start_time;
    while !game.should_stop {
        let frame_time = last_frame_time.elapsed().as_nanos() as f32 * 1e-9;
        println!("{:.4}", frame_time);
        last_frame_time = Instant::now();
        controller::handle_input(&mut game);
        game.physics_to_gfx();
        game.gfx.update_sprites(frame_time);// animations, tmp ?
        game.gfx._render_frame(game.camera);
    }
}




// TODO MAYBE pass all textures once to GPU + an indice in the vertices
