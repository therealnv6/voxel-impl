use bevy::prelude::Resource;
use bevy::reflect::Reflect;
use bevy_inspector_egui::InspectorOptions;
use ndshape::ConstShape;
use noise::utils::NoiseMap;
use noise::utils::NoiseMapBuilder;
use noise::utils::PlaneMapBuilder;
use noise::Fbm;
use noise::MultiFractal;

use super::TerrainGenerator;
use bevy::ecs::reflect::ReflectResource;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use noise::Perlin;

#[derive(Reflect, Resource, Default, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct NoiseData {
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
}

impl NoiseData {
    pub fn new() -> Self {
        Self {
            octaves: 1,
            persistence: 3.512,
            lacunarity: 3.351,
        }
    }
}

pub fn generate_terrain_3d<T: ConstShape<3, Coord = u32>, N: ConstShape<2, Coord = usize>>(
    noise_data: &NoiseData,
    seed: u32,
    terrain: impl TerrainGenerator,
) -> Vec<u8> {
    let mut ids = Vec::new();
    let noise_map = generate_noise_map::<N>(
        seed,
        noise_data.octaves,
        noise_data.persistence,
        noise_data.lacunarity,
    );

    for z in 0..T::ARRAY[2] {
        for y in 0..T::ARRAY[1] {
            for x in 0..T::ARRAY[0] {
                // scale the noise value to fit the desired range of heights
                let height = (noise_map[(x as usize, y as usize)] as f32) * T::ARRAY[2] as f32;
                if z as f32 <= height {
                    ids.push(terrain.get_block_type(height.into()));
                } else {
                    ids.push(0);
                }
            }
        }
    }

    ids
}

pub fn generate_noise_map<'a, T: ConstShape<2, Coord = usize>>(
    seed: u32,
    octaves: usize,
    persistance: f64,
    lacunarity: f64,
) -> NoiseMap {
    let fbm = Fbm::<Perlin>::new(seed)
        .set_octaves(octaves)
        .set_persistence(persistance)
        .set_lacunarity(lacunarity);

    PlaneMapBuilder::<&Fbm<Perlin>, 4>::new(&fbm)
        .set_size(T::ARRAY[0], T::ARRAY[1])
        .build()
}
