use std::vec::Vec;
use std::path::Path;

use glfw::Context as _; // For use swap_buffers (Context Trait)

use luminance::UniformInterface;
use luminance::blending::{Blending, Equation, Factor};
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance::pipeline::TextureBinding;
use luminance::pixel::{NormRGBA8UI, NormUnsigned};
use luminance::render_state::RenderState;
use luminance::shader::Program;
use luminance::shader::Uniform;
use luminance::tess::Mode;
use luminance::texture::{Dim2, Texture};
use luminance_gl::GL33;
use luminance_glfw::GlfwSurface;

use super::vertex;
use super::fixed_vec::FixedVec;
use vertex::{VERTICES, VertexSemantics};

mod utils;
use utils::{create_window, read_image, load_from_disk};

mod animation;
use animation::{AnimatedSprite, AnimationSet};
pub use animation::{Animation};

// TMP HERE
#[derive(UniformInterface)]
#[allow(dead_code)]
struct ShaderInterface {
    image:                  Uniform<TextureBinding<Dim2, NormUnsigned>>,
    camera:                 Uniform<[f32; 3]>,
}
// END TMP

#[allow(dead_code)]
pub struct Gfx {
    pub surface:            GlfwSurface,
    back_buffer:            luminance::framebuffer::Framebuffer
                                <luminance_gl::gl33::GL33, luminance::texture::Dim2, (), ()>,
    program:                Program<GL33, VertexSemantics, (), ShaderInterface>,

    pub animated_sprites:   FixedVec<AnimatedSprite>,
    animation_sets:         Vec<AnimationSet>,
    textures:               Vec<luminance::texture::Texture
                                <GL33, luminance::texture::Dim2, NormRGBA8UI>>,
}

const VS_STR: &str = include_str!("../shaders/vs.glsl");
const FS_STR: &str = include_str!("../shaders/fs.glsl");

#[allow(dead_code)]
impl Gfx {
    pub fn new()-> Gfx {
        let mut surface = create_window();
        let back_buffer = surface.back_buffer().unwrap();
        let program = surface
            .new_shader_program::<VertexSemantics, (), ShaderInterface>()
            .from_strings(VS_STR, None, None, FS_STR)
            .unwrap()
            .ignore_warnings();
        let animated_sprites: FixedVec<AnimatedSprite> = FixedVec::new();
        let animation_sets: Vec<AnimationSet> = Vec::new();
        let textures: Vec<Texture
                <GL33, Dim2, NormRGBA8UI>> = Vec::new();
        Gfx {
            surface,
            back_buffer,
            program,
            animated_sprites,
            animation_sets,
            textures,
        }
    }

    pub fn add_texture(&mut self, path: &str) {
        let img = read_image(Path::new(path))
                .expect("error while reading image on disk");
        let tex = load_from_disk(&mut self.surface, img);
        self.textures.push(tex);
    }

    pub fn add_animation_set(&mut self, animations: Vec<Animation>) {
        self.animation_sets.push(AnimationSet {
            animations,
        });
    }

    pub fn add_animated_sprite(&mut self, texture_index: usize, animation_set_index: usize)-> usize {
        let tess = self.surface.new_tess()
            .set_vertices(VERTICES)
            .set_mode(Mode::TriangleFan)
            .build()
            .unwrap();
        self.animated_sprites.add(AnimatedSprite::new(
            tess,
            texture_index,
            animation_set_index,
        ))
    }

// TODO TODO
    pub fn update_sprites(&mut self, frame_time: f32) {
        // MOVE TO ANIMATION
        let sprites = &mut self.animated_sprites;
        let animation_sets = &mut self.animation_sets;
        sprites.iter(|s: &mut AnimatedSprite| {
            s.frame_timer += frame_time;
            if s.frame_timer > 1. / 12. {
                s.frame_timer -= 1. / 12.;
                let frames_uv = &animation_sets[
                    s.animation_set_index
                ].animations[
                    s.selected_animation
                ].frames_uv;
                s.animation_frame = (s.animation_frame + 1) % frames_uv.len();
                let f = &frames_uv[s.animation_frame];
                s.update_tess_texpos(f);
            }
        })
    }

// TODO TODO TODO
pub fn render_frame(&mut self, camera: [f32; 3]) {
    let gfx = self;
    let program = &mut gfx.program;
    let sprites = &mut gfx.animated_sprites;
    let surface = &mut gfx.surface;
    let back_buffer = &mut gfx.back_buffer;
    let textures = &mut gfx.textures;

    let render = surface.new_pipeline_gate().pipeline(
        &back_buffer,
        &PipelineState::default().set_clear_color([0.7, 0.7, 0.7, 0.]),
        |pipeline, mut shd_gate| {
            sprites.iter(|s: &mut AnimatedSprite| {
                // Bind the texture to GPU
                let texture = &mut textures[s.texture_index];
                let bound_tex = pipeline.bind_texture(texture).unwrap();
                ||->Result<(), ()> {// To silent the type error from shd_gate.shade
                    shd_gate.shade(program, |mut interface, uni, mut rdr_gate| {
                        interface.set(&uni.camera, camera);
                        interface.set(&uni.image, bound_tex.binding());

//pass render state via gfx
                        let render_state = RenderState::default()
                            .set_depth_test(None)
                            .set_blending(Blending {
                                equation: Equation::Additive,
                                src: Factor::SrcAlpha,
                                dst: Factor::SrcAlphaComplement,
                            });
                        rdr_gate.render(&render_state, |mut tess_gate| {
                            tess_gate.render(&s.tess)
                        })
                    })?;
                    Ok(())
                }().unwrap();
            });
            Ok(())
        }
    ).assume();
    if !render.is_ok() {
        println!("RENDER NOT OK");
        std::process::exit(44);
    }
    gfx.surface.window.swap_buffers();
}
// TODO END TODO END TODO
}

