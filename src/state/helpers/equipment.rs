use crate::components::equipment::{EquipmentModel, EquipmentType, FireType, WeaponProperties};

// EQUIPMENT TYPES
pub fn get_equipment() -> (
    EquipmentType,
    EquipmentType,
    EquipmentModel,
    EquipmentModel,
    EquipmentModel,
    EquipmentModel,
) {
    let small_arms: EquipmentType = EquipmentType::new(
        String::from("Small Arms"),
        vec![FireType::Direct],
        WeaponProperties::new(1.0, 300.0),
    );
    let artillery: EquipmentType = EquipmentType::new(
        String::from("Artillery"),
        vec![FireType::Indirect],
        WeaponProperties::new(5.0, 1000.0),
    );

    // SMALL ARMS
    let pukka_1: EquipmentModel = EquipmentModel::new(
        String::from("Pukka Mk. 1"),
        small_arms.clone(),
        WeaponProperties::new(1.0, 1.0),
    );
    let pukka_2: EquipmentModel = EquipmentModel::new(
        String::from("Pukka Mk. 4"),
        small_arms.clone(),
        WeaponProperties::new(2.0, 1.5),
    );

    // artillery
    let thana_1: EquipmentModel = EquipmentModel::new(
        String::from("Thana Mk. 1"),
        artillery.clone(),
        WeaponProperties::new(1.0, 1.0),
    );
    let thana_2: EquipmentModel = EquipmentModel::new(
        String::from("Thana Mk. 2"),
        artillery.clone(),
        WeaponProperties::new(1.0, 2.0),
    );

    return (small_arms, artillery, pukka_1, pukka_2, thana_1, thana_2);
}
