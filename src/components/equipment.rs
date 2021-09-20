use amethyst::core::ecs::*;
use amethyst::prelude::*;

#[derive(Clone, Debug)]
pub enum FireType {
    Direct,
    Indirect,
}

#[derive(Clone, Debug)]
pub struct WeaponProperties {
    damage: f32,
    range: f32,
}

impl WeaponProperties {
    pub(crate) fn new(damage: f32, range: f32) -> Self {
        WeaponProperties { damage, range }
    }
}

#[derive(Clone, Debug)]
pub struct EquipmentType {
    base_properties: WeaponProperties,

    name: String,
    fire_types: Vec<FireType>,
}

impl EquipmentType {
    pub fn new(name: String, fire_types: Vec<FireType>, base_properties: WeaponProperties) -> Self {
        EquipmentType {
            base_properties,
            name,
            fire_types,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EquipmentModel {
    modifiers: WeaponProperties,

    name: String,
    equipment_type: EquipmentType,
}

impl EquipmentModel {
    pub fn new(name: String, equipment_type: EquipmentType, modifiers: WeaponProperties) -> Self {
        EquipmentModel {
            name,
            equipment_type,
            modifiers,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EquipmentComponent {
    equipment: Vec<(EquipmentModel, usize)>, // amount, model
}

impl EquipmentComponent {
    pub fn new(equipment: Vec<(EquipmentModel, usize)>) -> Self {
        EquipmentComponent { equipment }
    }
}

impl Component for EquipmentComponent {
    type Storage = DenseVecStorage<Self>;
}
