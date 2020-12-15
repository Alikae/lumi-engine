use luminance_gl::GL33;

use super::super::vertex;
use vertex::{VertexPosition, VertexTexpos};

#[allow(dead_code)]
pub struct AnimatedSprite {
    pub tess:                   luminance::tess::Tess<GL33, vertex::Vertex>,
    pub texture_index:          usize,
    pub animation_set_index:    usize,
    pub selected_animation:     usize,
    pub animation_frame:            usize,
    pub frame_timer:                f32,
}

#[allow(dead_code)]
pub struct AnimationSet {
    pub animations:             Vec<Animation>,
}

#[allow(dead_code)]
pub struct Animation {
    pub frames_uv:              Vec<FrameUV>,
}

#[allow(dead_code)]
pub struct FrameUV {
    xmin:                   f32,
    xmax:                   f32,
    ymin:                   f32,
    ymax:                   f32,
}

#[allow(dead_code)]
impl AnimatedSprite {
    pub fn new(tess: luminance::tess::Tess<GL33, vertex::Vertex>, texture_index: usize, animation_set_index: usize)-> AnimatedSprite {
        AnimatedSprite {
            tess,
            texture_index,
            animation_set_index,
            selected_animation: 0,
            animation_frame: 0,
            frame_timer: 0.,
        }
    }

    pub fn update_tess_pos(&mut self, pos: (f32, f32), size: f32) {
        // can i set instead of allocate new ??? TODO
        // Maybe store mut vertices ?
        let mut vertices = self.tess.vertices_mut().unwrap();
        vertices[0].position = VertexPosition::new([pos.0, pos.1]);
        vertices[1].position = VertexPosition::new([pos.0 + size, pos.1]);
        vertices[2].position = VertexPosition::new([pos.0 + size, pos.1 + size]);
        vertices[3].position = VertexPosition::new([pos.0, pos.1 + size]);
    }
    
    pub fn update_tess_texpos(&mut self, f: &FrameUV) {
        let mut vertices = self.tess.vertices_mut().unwrap();
        vertices[0].texpos = VertexTexpos::new([f.xmin, f.ymin]);
        vertices[1].texpos = VertexTexpos::new([f.xmax, f.ymin]);
        vertices[2].texpos = VertexTexpos::new([f.xmax, f.ymax]);
        vertices[3].texpos = VertexTexpos::new([f.xmin, f.ymax]);
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
