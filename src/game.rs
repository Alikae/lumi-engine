use std::time::Instant;

use super::physics_engine::{Physics};
use super::controller::{self, Controller};
use super::gfx_engine::Gfx;

pub struct Game {
    pub should_stop:    bool,
    pub start_time:     Instant,
    pub gfx:            Gfx,
    pub physics:        Physics,
    pub objects:        Vec<GameObject>,
    pub controller:     Controller,
    pub camera:         [f32; 3],
}

pub struct GameObject {
    sprite_index:   usize,
    physic_index:  usize,
}

impl Game {
    pub fn new(gfx: Gfx, physics: Physics)-> Game {
        Game {
            should_stop:    false,
            start_time:     Instant::now(),
            gfx,
            physics,
            objects:        Vec::new(),
            controller:     Controller::new(),
            camera:         [1., 1., 0.5],
        }
    }

    pub fn run(mut self) {
        let mut last_frame_time = self.start_time;
        while !self.should_stop {
            let frame_time = last_frame_time.elapsed().as_nanos() as f32 * 1e-9;
            println!("{:.4}", frame_time);
            last_frame_time = Instant::now();
            controller::handle_input(&mut self);
            self.connect_engines();
            self.gfx.update_sprites(frame_time);// animations, tmp ?
            self.gfx._render_frame(self.camera);
        }
    }

    pub fn create_object(&mut self, texture_index: usize, animation_set_index: usize, pos: (f32, f32))-> usize {
        let sprite_index = self.gfx.add_animated_sprite(texture_index, animation_set_index);
        let physic_index = self.physics.add_object(pos);
        self.objects.push(GameObject {
            sprite_index,
            physic_index,
        });
        self.objects.len() - 1
    }

    fn physics_to_gfx(&mut self) {
        for i in 0..self.objects.len() {
            self.gfx.animated_sprites[self.objects[i].sprite_index].update_tess_pos(self.physics.objects[self.objects[i].physic_index].pos);
        }
    }

    fn connect_engines(&mut self) {
        self.physics_to_gfx();
    }
}

