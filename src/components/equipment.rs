use amethyst::core::ecs::*;

#[derive(Clone, Debug)]
pub enum FireType {
    Direct,
    Indirect,
}

#[derive(Clone, Debug)]
pub struct WeaponProperties {
    pub damage: f32,
    pub range: f32,
    pub manpower: Option<usize>,
}

impl WeaponProperties {
    pub fn new(damage: f32, range: f32, manpower: Option<usize>) -> Self {
        WeaponProperties {
            damage,
            range,
            manpower,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EquipmentType {
    base_properties: WeaponProperties,

    name: String,
    fire_types: Vec<FireType>,
}

impl EquipmentType {
    pub fn new(
        name: String,
        fire_types: Vec<FireType>,
        damage: f32,
        range: f32,
        manpower: usize,
    ) -> Self {
        EquipmentType {
            base_properties: WeaponProperties::new(damage, range, Some(manpower)),
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
    pub fn new(
        name: String,
        equipment_type: EquipmentType,
        damage_mod: f32,
        range_mod: f32,
    ) -> Self {
        EquipmentModel {
            name,
            equipment_type,
            modifiers: WeaponProperties::new(damage_mod, range_mod, None),
        }
    }

    pub fn stats(&self) -> WeaponProperties {
        let base = &self.equipment_type.base_properties;
        WeaponProperties {
            damage: base.damage * self.modifiers.damage,
            range: base.range * self.modifiers.range,
            manpower: base.manpower,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Equipment {
    pub equipment: EquipmentModel,
    amount: usize, // amount, model
}

impl Equipment {
    pub fn new(equipment: EquipmentModel, amount: usize) -> Self {
        Equipment { equipment, amount }
    }

    pub fn deal_damage(&self) -> f32 {
        self.equipment.stats().damage * self.amount as f32
    }
}

impl Component for Equipment {
    type Storage = DenseVecStorage<Self>;
}
