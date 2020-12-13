use std::time::Instant;

use super::physics_engine::{Physics};
use super::controller::{self, Controller};
use super::gfx_engine::Gfx;

pub struct Game {
    pub gfx:            Gfx,
    pub physics:        Physics,
    pub controller:     Controller,

    pub should_stop:    bool,
    pub start_time:     Instant,
    pub objects:        Vec<GameObject>,
    pub camera:         [f32; 3],
}

pub struct GameObject {
    sprite_index:   usize,
    physic_index:   usize,
    size:           f32,
}

impl Game {
    pub fn new(gfx: Gfx, physics: Physics)-> Game {
        Game {
            gfx,
            physics,
            controller:     Controller::new(),
            should_stop:    false,
            start_time:     Instant::now(),
            objects:        Vec::new(),
            camera:         [1., 1., 0.5],
        }
    }

    pub fn run(mut self) {
        let mut last_frame_time = self.start_time;
        while !self.should_stop {
            // FrameRate
            let frame_time = last_frame_time.elapsed().as_nanos() as f32 * 1e-9;
            last_frame_time = Instant::now();
            println!("{} {:.4} <{:width$}>", get_framerate_color(frame_time), frame_time, "", width=(frame_time * 1000.) as usize);
            //END
            controller::handle_input(&mut self);
            // self.physics.update()
            self.connect_engines();
            self.gfx.update_sprites(frame_time);// animations, tmp ?
            self.gfx._render_frame(self.camera);
        }
    }

    pub fn create_object(&mut self, texture_index: usize, animation_set_index: usize, pos: (f32, f32), size: f32)-> usize {
        let sprite_index = self.gfx.add_animated_sprite(texture_index, animation_set_index);
        let physic_index = self.physics.add_object(pos);
        self.objects.push(GameObject {
            sprite_index,
            physic_index,
            size,
        });
        self.objects.len() - 1
    }

    fn physics_to_gfx(&mut self) {
        for i in 0..self.objects.len() {
            let obj = &self.objects[i];
            self.gfx.animated_sprites[obj.sprite_index]
                .update_tess_pos(
                    self.physics.objects[obj.physic_index].pos,
                    obj.size);
        }
    }

    fn connect_engines(&mut self) {
        self.physics_to_gfx();
    }
}

// Utils

fn get_framerate_color(frame_time: f32)-> &'static str {
    if      frame_time > 1. / 60.   {"\x1B[31m"}
    else if frame_time > 1. / 80.   {"\x1B[35m"}
    else if frame_time < 1. / 120.  {"\x1B[32m"}
    else                            {"\x1B[34m"}
}

