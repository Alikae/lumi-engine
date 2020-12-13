pub struct Physics {
    pub objects:        Vec<PhysicObject>,
}

pub struct PhysicObject {
    pub pos:            (f32, f32),
    pub sprite_index:   usize,
}

impl Physics {
    pub fn new()-> Physics {
        Physics {
            objects: Vec::new(),
        }
    }
}

impl PhysicObject {
    pub fn new(pos: (f32, f32), sprite_index: usize)-> PhysicObject {
        PhysicObject {
            pos,
            sprite_index,
        }
    }

    //pub fn update() {

    //}

    //pub fn collide_with_one()

//    fn update_gfx(&self) {
        // take vertices_mut
        // set them to actual pos
//    }
}
