use block_mesh::{MergeVoxel, VoxelVisibility};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Voxel {
    pub id: u8,
}

pub const VOID: Voxel = Voxel { id: 0 };
pub const CUBE_VERTICES: [f32; 24] = [
    -0.5, 0.5, 0.5, // front top left
    0.5, 0.5, 0.5, // front top right
    -0.5, -0.5, 0.5, // front bottom left
    0.5, -0.5, 0.5, // front bottom right
    -0.5, 0.5, -0.5, // back top left
    0.5, 0.5, -0.5, // back top right
    -0.5, -0.5, -0.5, // back bottom left
    0.5, -0.5, -0.5, // back bottom right
];

pub const CUBE_INDICES: [u32; 36] = [
    0, 2, 1, // FRONT
    1, 2, 3, 0, 1, 4, // TOP
    4, 1, 5, 1, 3, 5, // RIGHT
    5, 3, 7, 2, 7, 3, // BOTTOM
    7, 2, 6, 6, 2, 0, // LEFT
    0, 4, 6, 6, 4, 5, // BACK
    5, 7, 6,
];

pub const CUBE_NORMALS: [f32; 24] = [
    0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0,
    0.0, 0.0, -1.0, 0.0, 0.0, -1.0,
];

impl block_mesh::Voxel for Voxel {
    fn get_visibility(&self) -> block_mesh::VoxelVisibility {
        if self.id == 0 {
            VoxelVisibility::Translucent
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for Voxel {
    type MergeValue = u8;

    fn merge_value(&self) -> Self::MergeValue {
        self.id
    }
}
