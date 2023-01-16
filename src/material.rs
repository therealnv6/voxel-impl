use std::any::TypeId;

use bevy::{
    prelude::{AlphaMode, Assets, Color, Handle, Plugin, ResMut, Resource, StandardMaterial},
    utils::HashMap,
};

#[derive(Clone)]
pub struct Material {
    pub bevy_material: Handle<StandardMaterial>,
}

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

pub struct MaterialPlugin;

impl MaterialPlugin {
    pub fn init_materials(
        mut materials: ResMut<Materials>,
        mut bevy_materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mut id_map = HashMap::<u8, Material>::new();
        let mut type_map = HashMap::<TypeId, Material>::new();

        for (id, ty, material) in [
            (
                0,
                TypeId::of::<Void>(),
                Material {
                    bevy_material: bevy_materials.add(StandardMaterial {
                        base_color: Color::rgba(0.0, 137.0 / 255.0, 32.0 / 255.0, 0.0),
                        alpha_mode: AlphaMode::Mask(0.5),
                        ..Default::default()
                    }),
                },
            ),
            (
                1,
                TypeId::of::<Grass>(),
                Material {
                    bevy_material: bevy_materials.add(StandardMaterial {
                        base_color: Color::rgb(0.0, 137.0 / 255.0, 32.0 / 255.0),
                        ..Default::default()
                    }),
                },
            ),
        ] {
            id_map.insert(id, material.clone());
            type_map.insert(ty, material.clone());
        }

        materials.id_map = id_map;
        materials.type_map = type_map;
    }
}

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
