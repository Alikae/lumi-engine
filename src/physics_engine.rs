use super::fixed_vec::FixedVec;

pub struct Physics {
    //pub solid_map:      Vec<Vec<u8>>,
    pub objects:        FixedVec<PhysicObject>,
}

pub struct PhysicObject {
    pub pos:            (f32, f32),
}

impl Physics {
    pub fn new()-> Physics {
        Physics {
            objects: FixedVec::new(),
        }
    }

    pub fn add_object(&mut self, pos: (f32, f32))-> usize {
        self.objects.add(PhysicObject::new(pos))
    }

    pub fn update_world() {

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

// Different object layers:
//      floor           - ALL
//      player          - Floor ennemy{b}
//      player bullets  - Floor ennemy
//      ennemy          - Floor player{b}
//      ennemy bullets  - Floor player

/*
physics.add_layer(PLAYER)
physics.add_object(player_data)
physics.add_layer(E_BULLET)
physics.add_object(bullet_data)
physics.collide_layers(
    E_BULLET,
    PLAYER,
    |bullet, player|
        bullet.disappear, player.lose_life
)

physics.octree.collide() {
    find a/b via octree
    a.collide(&b)
    b.collide(&a)
}
*/

