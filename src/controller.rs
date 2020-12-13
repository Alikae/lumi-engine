use luminance_glfw::GlfwSurface;
use glfw::{Action, Key, WindowEvent}; // Context as _,

use super::Game;

pub struct Controller {
    pub quit:   bool,
    pub camleft:   bool,
    pub camright:  bool,
    pub camup:     bool,
    pub camdown:   bool,
    pub left:   bool,
    pub right:  bool,
    pub up:     bool,
    pub down:   bool,
    pub zoom:   bool,
    pub dezoom: bool,
}

// PASS VIA ENUM ?
impl Controller {
    pub fn new()-> Controller {
        Controller {
            quit:       false,
            camleft:    false,
            camright:   false,
            camup:      false,
            camdown:    false,
            left:       false,
            right:      false,
            up:         false,
            down:       false,
            zoom:       false,
            dezoom:     false,
        }
    }

// Pass via callback ?
    pub fn map_inputs(&mut self, surface: &mut GlfwSurface) {
        surface.window.glfw.poll_events();
        for (_, event) in surface.events_rx.try_iter() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => self.quit = true,
                WindowEvent::Key(Key::W, _, Action::Press, _)       => self.up = true,
                WindowEvent::Key(Key::W, _, Action::Release, _)     => self.up = false,
                WindowEvent::Key(Key::S, _, Action::Press, _)       => self.down = true,
                WindowEvent::Key(Key::S, _, Action::Release, _)     => self.down = false,
                WindowEvent::Key(Key::A, _, Action::Press, _)       => self.left = true,
                WindowEvent::Key(Key::A, _, Action::Release, _)     => self.left = false,
                WindowEvent::Key(Key::D, _, Action::Press, _)       => self.right = true,
                WindowEvent::Key(Key::D, _, Action::Release, _)     => self.right = false,

                WindowEvent::Key(Key::Up, _, Action::Press, _)      => self.camup = true,
                WindowEvent::Key(Key::Up, _, Action::Release, _)    => self.camup = false,
                WindowEvent::Key(Key::Down, _, Action::Press, _)    => self.camdown = true,
                WindowEvent::Key(Key::Down, _, Action::Release, _)  => self.camdown = false,
                WindowEvent::Key(Key::Left, _, Action::Press, _)    => self.camleft = true,
                WindowEvent::Key(Key::Left, _, Action::Release, _)  => self.camleft = false,
                WindowEvent::Key(Key::Right, _, Action::Press, _)   => self.camright = true,
                WindowEvent::Key(Key::Right, _, Action::Release, _) => self.camright = false,

                WindowEvent::Key(Key::Q, _, Action::Press, _)       => self.dezoom = true,
                WindowEvent::Key(Key::Q, _, Action::Release, _)     => self.dezoom = false,
                WindowEvent::Key(Key::E, _, Action::Press, _)       => self.zoom = true,
                WindowEvent::Key(Key::E, _, Action::Release, _)     => self.zoom = false,
                _ => (),
            }
        }
    }
}

// Input
pub fn handle_input(mut game: &mut Game) {
    let c = &mut game.controller;
    c.map_inputs(&mut game.gfx.surface);
    if c.quit {
        game.should_stop = true;
    }
    if c.zoom {
        game.camera[2] += 0.01;
    }
    if c.dezoom {
        game.camera[2] -= 0.01;
    }
    if c.camup {
        game.camera[1] += 0.01;
    }
    if c.camdown {
        game.camera[1] -= 0.01;
    }
    if c.camright {
        game.camera[0] += 0.01;
    }
    if c.camleft {
        game.camera[0] -= 0.01;
    }

    if c.up {
        game.physics.objects[0].pos.1 += 0.01;
    }
    if c.down {
        game.physics.objects[2].pos.1 -= 0.01;
    }
    if c.left {
        game.physics.objects[3].pos.0 -= 0.01;
    }
    if c.right {
        game.physics.objects[4].pos.0 += 0.01;
    }
}

