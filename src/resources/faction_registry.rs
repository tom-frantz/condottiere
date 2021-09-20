use crate::components::equipment::EquipmentModel;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum FactionType {
    Player(Faction),
    NonPlayer(Faction),
}

#[derive(Debug, Clone)]
pub struct Faction {
    equipment_models: HashSet<EquipmentModel>,
}

#[derive(Debug, Clone)]
pub struct FactionRegistry {
    factions: Vec<FactionType>,
}

impl FactionRegistry {
    fn new(factions: Vec<FactionType>) -> Self {
        FactionRegistry { factions }
    }
}
