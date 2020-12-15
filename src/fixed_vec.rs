// A simple Vector where elements always keep their index
pub struct FixedVec<T> {
    vec:    Vec<T>,
    used:   Vec<bool>,
    free:   Vec<usize>,
}

#[allow(dead_code)]
impl<T> FixedVec<T> {
    pub fn new()-> FixedVec<T> {
        FixedVec {
            vec:    Vec::new(),
            used:   Vec::new(),
            free:   Vec::new(),
        }
    }

    pub fn add(&mut self, elem: T)-> usize {
        if let Some(i) = self.free.pop() {
            self.vec[i] = elem;
            self.used[i] = true;
            i
        } else {
            self.vec.push(elem);
            self.used.push(true);
            self.vec.len() - 1
        }
    }

    pub fn remove(&mut self, i: usize) {
        self.used[i] = false;
        self.free.push(i);
    }

    pub fn get_mut(&mut self, i: usize)-> &mut T {
        &mut self.vec[i]
    }

    pub fn iter(&mut self, mut func: impl FnMut(&mut T)) {
        let len = self.used.len();
        for i in 0..len {
            if self.used[i] {
                func(&mut self.vec[i]);
            }
        }
    }
}

// Maybe optimize with a hashmap to map indexes ?

