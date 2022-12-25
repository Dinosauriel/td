use std::iter::Iterator;

pub struct Size3DIterator {
    size: Size3D,
    i: usize
}

impl Iterator for Size3DIterator {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.size.volume() {
            return None
        }
        let z = self.i % self.size.z;
        let y = (self.i / self.size.z) % self.size.y;
        let x = (self.i / (self.size.z * self.size.y)) % self.size.x;

        self.i += 1;

        return Some((x, y, z));
    }
}

#[derive(Clone, Copy)]
pub struct Size3D {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl IntoIterator for Size3D {
    type Item = (usize, usize, usize);
    type IntoIter = Size3DIterator;

    fn into_iter(self) -> Self::IntoIter {
        Size3DIterator { size: self, i: 0 }
    }
}

impl Size3D {
    pub fn volume(&self) -> usize {
        return self.x * self.y * self.z;
    }

    pub fn num_vertices(&self) -> usize {
        return (self.x + 1) * (self.y + 1) * (self.z + 1);
    }

    pub fn coordinates_1_d(&self, x: usize, y: usize, z: usize) -> usize {
        return self.x * self.z * y + self.z * x + z;
    }

    pub fn vertex_coordinates_1_d(&self, x: usize, y: usize, z: usize) -> usize {
        return (self.x + 1) * (self.z + 1) * y + (self.z + 1) * x + z;
    }
}

// size of sub_segment space
pub const L1_SIZE: Size3D = Size3D { x: 8, y: 8, z: 8 };
pub const L2_SIZE: Size3D = Size3D { x: 8, y: 8, z: 8 };
pub const L3_SIZE: Size3D = Size3D { x: 8, y: 8, z: 8 };
pub const L4_SIZE: Size3D = Size3D { x: 8, y: 8, z: 8 };

// size in blocks
pub const L1_SIZE_BL: Size3D = L1_SIZE;
pub const L2_SIZE_BL: Size3D = Size3D { x: L1_SIZE_BL.x * L2_SIZE.x, y: L1_SIZE_BL.y * L2_SIZE.y, z: L1_SIZE_BL.z * L2_SIZE.z };
pub const L3_SIZE_BL: Size3D = Size3D { x: L2_SIZE_BL.x * L3_SIZE.x, y: L2_SIZE_BL.y * L3_SIZE.y, z: L2_SIZE_BL.z * L3_SIZE.z };
pub const L4_SIZE_BL: Size3D = Size3D { x: L3_SIZE_BL.x * L4_SIZE.x, y: L3_SIZE_BL.y * L4_SIZE.y, z: L3_SIZE_BL.z * L4_SIZE.z };
