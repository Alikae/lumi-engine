use std::vec::Vec;
use std::path::Path;

use glfw::Context as _; // For use swap_buffers (Context Trait)

use luminance::UniformInterface;
use luminance::backend::texture::Texture as TextureBackend;
use luminance::blending::{Blending, Equation, Factor};
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance::pipeline::TextureBinding;
use luminance::pixel::{NormRGBA8UI, NormUnsigned};
use luminance::render_state::RenderState;
use luminance::shader::Program;
use luminance::shader::Uniform;
use luminance::tess::Mode;
use luminance::texture::{Dim2, GenMipmaps, Sampler, Texture};
use luminance::texture::{Wrap, MinFilter, MagFilter};
use luminance_gl::GL33;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

use super::vertex;
use super::fixed_vec::FixedVec;
use vertex::{VERTICES, VertexSemantics, VertexTexpos, VertexPosition};


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
    timer:                  f32, // TO REMOVE
}

#[allow(dead_code)]
pub struct AnimatedSprite {
    tess:                   luminance::tess::Tess<GL33, vertex::Vertex>,
    texture_index:          usize,
    animation_set_index:    usize,
    selected_animation:     usize,
    animation_frame:        usize,
    frame_timer:            f32,
}

#[allow(dead_code)]
struct AnimationSet {
    animations:             Vec<Animation>,
}

#[allow(dead_code)]
pub struct Animation {
    frames_uv:              Vec<FrameUV>,
}

#[allow(dead_code)]
pub struct FrameUV {
    xmin:                   f32,
    xmax:                   f32,
    ymin:                   f32,
    ymax:                   f32,
}

const VS_STR: &str = include_str!("../shaders/vs.glsl");
const FS_STR: &str = include_str!("../shaders/fs.glsl");

#[allow(dead_code)]
impl Gfx {
    pub fn new()-> Gfx {
        // Window
        let mut surface = create_window();
        let back_buffer = surface.back_buffer().unwrap();
        // Shader Program
        let program = surface
            .new_shader_program::<VertexSemantics, (), ShaderInterface>()
            .from_strings(VS_STR, None, None, FS_STR)
            .unwrap()
            .ignore_warnings();
        // Sprites
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
            timer: 0.,
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
        // handle remove !!!
        // Array of "freed" ?
        let tess = self.surface.new_tess()
            .set_vertices(VERTICES)
            .set_mode(Mode::TriangleFan)
            .build()
            .unwrap();
        self.animated_sprites.add(AnimatedSprite {
            tess,
            texture_index,
            animation_set_index,
            selected_animation: 0,
            animation_frame: 0,
            frame_timer: 0.,
        })
    }

// TODO TODO
    pub fn update_sprites(&mut self, frame_time: f32) {
        // each sprite should have its own timer ?
        self.timer += frame_time;
        if self.timer > 1. / 12. {
            self.timer -= 1. / 12.;
            let sprites = &mut self.animated_sprites;
            let animation_sets = &mut self.animation_sets;
            sprites.iter(|s: &mut AnimatedSprite| {
                let frames_uv = &animation_sets[
                    s.animation_set_index
                ].animations[
                    s.selected_animation
                ].frames_uv;
                s.animation_frame = (s.animation_frame + 1) % frames_uv.len();
                let f = &frames_uv[s.animation_frame];
                // Update sprite tess
                let mut vertices = s.tess.vertices_mut().unwrap();
                vertices[0].texpos = VertexTexpos::new([f.xmin, f.ymin]);
                vertices[1].texpos = VertexTexpos::new([f.xmax, f.ymin]);
                vertices[2].texpos = VertexTexpos::new([f.xmax, f.ymax]);
                vertices[3].texpos = VertexTexpos::new([f.xmin, f.ymax]);
            })
        }
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

impl AnimatedSprite {
    pub fn update_tess_pos(&mut self, pos: (f32, f32), size: f32) {
        let mut vertices = self.tess.vertices_mut().unwrap();
        vertices[0].position = VertexPosition::new([pos.0, pos.1]);
        vertices[1].position = VertexPosition::new([pos.0 + size, pos.1]);
        vertices[2].position = VertexPosition::new([pos.0 + size, pos.1 + size]);
        vertices[3].position = VertexPosition::new([pos.0, pos.1 + size]);
    }
}

#[allow(dead_code)]
impl Animation {
    pub fn new()-> Animation {
        Animation {
            frames_uv: Vec::new(),
        }
    }

    pub fn add_frame_uv(&mut self, xmin: f32, xmax: f32, ymin: f32, ymax: f32) {
        self.frames_uv.push(FrameUV {
            xmin,
            xmax,
            ymin,
            ymax,
        });
    }
    
    pub fn auto_split_4(&mut self) {
        self.add_frame_uv(0., 0.5, 0.5, 1.);
        self.add_frame_uv(0.5, 1., 0.5, 1.);
        self.add_frame_uv(0., 0.5, 0., 0.5);
        self.add_frame_uv(0.5, 1., 0., 0.5);
    }
}



// UTILS remove pub

fn create_window()-> GlfwSurface {
    let dim = WindowDim::Windowed { width: 1920, height: 1080 };
    GlfwSurface::new_gl33("Luminance, BITCHES!", WindowOpt::default().set_dim(dim)).unwrap()
}

pub fn read_image(path: &Path) -> Option<image::RgbaImage> {
      image::open(path).map(|img| img.flipv().to_rgba8()).ok()
}

pub fn load_from_disk<B>(surface: &mut B, img: image::RgbaImage) -> Texture<B::Backend, Dim2, NormRGBA8UI>
where
  B: GraphicsContext,
  B::Backend: TextureBackend<Dim2, NormRGBA8UI>,
{
      let (width, height) = img.dimensions();
      let texels = img.into_raw();

    // create the luminance texture; the third argument is the number of mipmaps we want (leave it
    // to 0 for now) and the latest is the sampler to use when sampling the texels in the
    // shader (we’ll just use the default one)
    let mut tex = Texture::new(surface, [width, height], 0,
        Sampler {
            wrap_r: Wrap::ClampToEdge,
            wrap_s: Wrap::ClampToEdge,
            wrap_t: Wrap::ClampToEdge,
            min_filter: MinFilter::Nearest,
            mag_filter: MagFilter::Nearest,
            depth_comparison: None,
        })
        .expect("luminance texture creation");

    // the first argument disables mipmap generation (we don’t care so far)
    tex.upload_raw(GenMipmaps::No, &texels).unwrap();

    tex
}

