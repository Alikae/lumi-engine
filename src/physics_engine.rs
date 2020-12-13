pub struct Physics {
    pub objects:        Vec<PhysicObject>,
}

pub struct PhysicObject {
    pub pos:            (f32, f32),
}

impl Physics {
    pub fn new()-> Physics {
        Physics {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, pos: (f32, f32))-> usize {
        self.objects.push(PhysicObject::new(pos));
        self.objects.len() - 1
    }
}

impl PhysicObject {
    pub fn new(pos: (f32, f32))-> PhysicObject {
        PhysicObject {
            pos,
        }
    }

    //pub fn update() {
    //}

    //pub fn collide_with()

//    fn update_gfx(&self) {
        // take vertices_mut
        // set them to actual pos
//    }
}

