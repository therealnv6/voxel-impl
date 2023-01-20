use std::any::TypeId;

use bevy::{
    prelude::{Plugin, ResMut, Resource},
    utils::HashMap,
};

#[derive(Clone)]
pub struct Material {}

#[derive(Resource)]
pub struct Materials {
    id_map: HashMap<u8, Material>,
    type_map: HashMap<TypeId, Material>,
}

impl Materials {
    pub fn get_from_id(&self, id: &u8) -> Option<&Material> {
        self.id_map.get(id)
    }

    pub fn get<T: 'static>(&self) -> Option<&Material> {
        self.type_map.get(&TypeId::of::<T>())
    }
}

pub const MAT_COLORS: [[f32; 4]; 5] = [
    [0.0, 137.0 / 255.0, 32.0 / 255.0, 0.0],            // void
    [0.0, 137.0 / 255.0, 32.0 / 255.0, 1.0],            // grass
    [145.0 / 255.0, 142.0 / 255.0, 133.0 / 255.0, 1.0], // stone,
    [0.0, 0.0, 137.0 / 255.0, 0.63],                    // water
    [255.0 / 255.0, 229.0 / 255.0, 153.0 / 255.0, 1.0], // sand
];

impl MaterialPlugin {
    pub fn init_materials(mut materials: ResMut<Materials>) {
        let mut id_map = HashMap::<u8, Material>::new();
        let mut type_map = HashMap::<TypeId, Material>::new();

        for (id, ty, material) in [
            (0, TypeId::of::<Void>(), Material {}),
            (1, TypeId::of::<Grass>(), Material {}),
            (2, TypeId::of::<Stone>(), Material {}),
            (3, TypeId::of::<Water>(), Material {}),
            (4, TypeId::of::<Sand>(), Material {}),
        ] {
            id_map.insert(id, material.clone());
            type_map.insert(ty, material.clone());
        }

        materials.id_map = id_map;
        materials.type_map = type_map;
    }
}

pub struct MaterialPlugin;

impl Plugin for MaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(Self::init_materials)
            .insert_resource(Materials {
                id_map: HashMap::new(),
                type_map: HashMap::new(),
            });
    }
}

// type structs
pub struct Grass;
pub struct Void;
pub struct Stone;
pub struct Water;
pub struct Sand;
