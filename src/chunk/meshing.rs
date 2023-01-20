use bevy::{
    prelude::Mesh,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};
use block_mesh::{GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
use ndshape::ConstShape;

use crate::material::MAT_COLORS;

use super::{
    voxel::{Voxel, VOID},
    Chunk, ChunkShape, X_SIZE,
};

impl Chunk {
    pub fn get_mesh(&mut self) -> Mesh {
        if self.mesh.is_none() {
            self.mesh = Some(self.mesh());
        }

        self.mesh.clone().unwrap()
    }

    pub fn mesh(&self) -> Mesh {
        let mut voxels = [VOID; ChunkShape::SIZE as usize];

        for i in 0..ChunkShape::SIZE {
            let [x, y, z] = ChunkShape::delinearize(i);
            let id = self.get_block([x, y, z]);

            voxels[i as usize] = Voxel { id };
        }

        let mut buffer = GreedyQuadsBuffer::new(voxels.len());
        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

        block_mesh::greedy_quads(
            &voxels,
            &ChunkShape {},
            [0; 3],
            [(X_SIZE - 1) as u32; 3],
            &faces,
            &mut buffer,
        );

        let num_indices = buffer.quads.num_quads() * 6;
        let num_vertices = buffer.quads.num_quads() * 4;

        let mut indices = Vec::with_capacity(num_indices);
        let mut positions = Vec::with_capacity(num_vertices);
        let mut normals = Vec::with_capacity(num_vertices);
        let mut colors = Vec::<[f32; 4]>::with_capacity(num_vertices);

        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
                positions.extend_from_slice(&face.quad_mesh_positions(&quad.into(), 1.0));
                normals.extend_from_slice(&face.quad_mesh_normals());

                let [x, y, z] = quad.minimum;
                let id = self.get_block([x, y, z]);
                let color = MAT_COLORS[id as usize];

                colors.extend_from_slice(&[color; 4]);
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        for (key, value) in [
            (
                Mesh::ATTRIBUTE_POSITION,
                VertexAttributeValues::Float32x3(positions),
            ),
            (
                Mesh::ATTRIBUTE_NORMAL,
                VertexAttributeValues::Float32x3(normals),
            ),
            (
                Mesh::ATTRIBUTE_COLOR,
                VertexAttributeValues::Float32x4(colors),
            ),
        ] {
            mesh.insert_attribute(key, value);
        }

        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
