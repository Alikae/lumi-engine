use std::time::Instant;

use super::physics_engine::{Physics};
use super::controller::{self, Controller};
use super::gfx_engine::Gfx;
use super::fixed_vec::FixedVec;

pub struct Game {
    pub gfx:            Gfx,
    pub physics:        Physics,
    pub controller:     Controller,

    pub should_stop:    bool,
    pub start_time:     Instant,
    pub objects:        FixedVec<GameObject>,
    pub camera:         [f32; 3],
}

pub struct GameObject {
    // not pub
    pub sprite_index:   usize,
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
            objects:        FixedVec::new(),
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
            self.update_engines(frame_time);
            self.connect_engines();
            self.gfx.render_frame(self.camera);
        }
    }

    pub fn create_object(&mut self, texture_index: usize, animation_set_index: usize, pos: (f32, f32), size: f32)-> usize {
        let sprite_index = self.gfx.add_animated_sprite(texture_index, animation_set_index);
        let physic_index = self.physics.add_object(pos);
        self.objects.add(GameObject {
            sprite_index,
            physic_index,
            size,
        })
    }

    fn update_engines(&mut self, frame_time: f32) {
        self.gfx.update_sprites(frame_time);// animations, tmp ?
        // self.physics.update_world()
        // erase all objects whose lifetime's over
        //for _i in 0..self.objects.len() {
            //if (self.objects[i].)
        //}
    }

    fn connect_engines(&mut self) {
        self.physics_to_gfx();
    }

    fn physics_to_gfx(&mut self) {
        let gfx = &mut self.gfx;
        let physics = &mut self.physics;
        self.objects.iter(|obj| {
            gfx.animated_sprites.get_mut(obj.sprite_index)
                .update_tess_pos(
                    physics.objects.get_mut(obj.physic_index).pos,
                    obj.size);
        });
    }
}

// Utils

fn get_framerate_color(frame_time: f32)-> &'static str {
    if      frame_time > 1. / 60.   {"\x1B[31m"}
    else if frame_time > 1. / 80.   {"\x1B[35m"}
    else if frame_time < 1. / 120.  {"\x1B[32m"}
    else                            {"\x1B[34m"}
}

// TODO TODO Remove objects (Game, gfx, physic)
// Physic update
//      solid tiles
//      player collide with them
//      gravity
//      jump
// Ui debug
