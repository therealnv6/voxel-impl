use bevy::{
    prelude::Mesh,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};

use super::{
    voxel::{CUBE_INDICES, CUBE_VERTICES},
    Chunk, X_SIZE, Y_SIZE, Z_SIZE,
};

impl Chunk {
    pub fn mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut colors = Vec::new();

        let mut vertex_offset = 0; // Keep track of current vertex offset

        for x in 0..X_SIZE {
            for y in 0..Y_SIZE {
                for z in 0..Z_SIZE {
                    let id = self.blocks[x as usize][y as usize][z as usize];
                    if id > 0 {
                        let scale = 255.0 / X_SIZE as f32;
                        let color = [
                            (x as f32 * scale) / 255.0,
                            (y as f32 * scale) / 255.0,
                            (z as f32 * scale) / 255.0,
                            0.2,
                        ];
                        // Check for visible faces
                        if x == 0 || self.blocks[x as usize - 1][y as usize][z as usize] == 0 {
                            for i in 0..8 {
                                let i = i * 3;
                                colors.push(color);
                                vertices.push([
                                    CUBE_VERTICES[i] + x as f32,
                                    CUBE_VERTICES[i + 1] + y as f32,
                                    CUBE_VERTICES[i + 2] + z as f32,
                                ]);
                            }
                        }
                        if x == X_SIZE - 1
                            || self.blocks[x as usize + 1][y as usize][z as usize] == 0
                        {
                            for i in 0..8 {
                                let i = i * 3;
                                colors.push(color);
                                vertices.push([
                                    CUBE_VERTICES[i] + x as f32,
                                    CUBE_VERTICES[i + 1] + y as f32,
                                    CUBE_VERTICES[i + 2] + z as f32,
                                ]);
                            }
                        }
                        if y == 0 || self.blocks[x as usize][y as usize - 1][z as usize] == 0 {
                            for i in 0..8 {
                                let i = i * 3;
                                colors.push(color);
                                vertices.push([
                                    CUBE_VERTICES[i] + x as f32,
                                    CUBE_VERTICES[i + 1] + y as f32,
                                    CUBE_VERTICES[i + 2] + z as f32,
                                ]);
                            }
                        }
                        if y == Y_SIZE - 1
                            || self.blocks[x as usize][y as usize + 1][z as usize] == 0
                        {
                            for i in 0..8 {
                                let i = i * 3;
                                colors.push(color);
                                vertices.push([
                                    CUBE_VERTICES[i] + x as f32,
                                    CUBE_VERTICES[i + 1] + y as f32,
                                    CUBE_VERTICES[i + 2] + z as f32,
                                ]);
                            }
                        }
                        if z == 0 || self.blocks[x as usize][y as usize][z as usize - 1] == 0 {
                            for i in 0..8 {
                                let i = i * 3;
                                colors.push(color);
                                vertices.push([
                                    CUBE_VERTICES[i] + x as f32,
                                    CUBE_VERTICES[i + 1] + y as f32,
                                    CUBE_VERTICES[i + 2] + z as f32,
                                ]);
                            }
                        }
                        if z == Z_SIZE - 1
                            || self.blocks[x as usize][y as usize][z as usize + 1] == 0
                        {
                            for i in 0..8 {
                                let i = i * 3;
                                colors.push(color);
                                vertices.push([
                                    CUBE_VERTICES[i] + x as f32,
                                    CUBE_VERTICES[i + 1] + y as f32,
                                    CUBE_VERTICES[i + 2] + z as f32,
                                ]);
                            }
                        }
                    }
                }
            }
        }
        // Add the indices for all cubes
        for i in 0..vertices.len() / 24 {
            let offset = i * 24;
            for j in 0..36 {
                indices.push(CUBE_INDICES[j] + offset as u32);
            }
        }

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vertices),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            VertexAttributeValues::Float32x4(colors),
        );
        mesh.set_indices(Some(Indices::U32(indices)));

        mesh
    }
}
