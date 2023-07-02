use std::collections::HashMap;

use crate::{
    component::{ComponentId, ComponentList},
    storage::TableId,
};

#[derive(Clone, Copy, Debug)]
pub struct ArchetypeId(usize);

impl ArchetypeId {
    pub(crate) const UNINIT: ArchetypeId = ArchetypeId(usize::MAX);

    pub(crate) fn is_init(&self) -> bool {
        self.0 != ArchetypeId::UNINIT.0
    }
}

type ArchetypeComponentIds = Vec<ComponentId>;

#[derive(Clone)]
pub(crate) struct ArchetypeData {
    pub(crate) components_ids: Vec<ComponentId>,
    table_id: TableId,
}

pub struct Archetypes {
    data: Vec<ArchetypeData>,
    id_mappings: HashMap<ArchetypeComponentIds, usize>,
}

impl Default for Archetypes {
    fn default() -> Self {
        Self::new()
    }
}

impl Archetypes {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            id_mappings: HashMap::new(),
        }
    }

    pub fn get_or_init<CL>(&mut self) -> ArchetypeId
    where
        CL: ComponentList,
    {
        let mut sorted_component_ids = CL::get_ids();
        sorted_component_ids.sort();

        if let Some(index) = self.id_mappings.get(&sorted_component_ids) {
            return ArchetypeId(*index);
        } else {
            let index = self.data.len();
            self.id_mappings.insert(sorted_component_ids.clone(), index);
            self.data.insert(
                index,
                ArchetypeData {
                    components_ids: sorted_component_ids,
                    table_id: TableId::UNINIT,
                },
            );

            return ArchetypeId(index);
        }
    }

    pub(crate) fn get_data(&self, archetype_id: &ArchetypeId) -> &ArchetypeData {
        self.data.get(archetype_id.0).expect("")
    }

    pub fn get_table_id(&self, archetype_id: &ArchetypeId) -> TableId {
        self.data.get(archetype_id.0).clone().expect("").table_id
    }

    pub fn update_table_id(&mut self, archetype_id: &ArchetypeId, table_id: &TableId) {
        let data = self.data.get_mut(archetype_id.0).unwrap();
        data.table_id = *table_id;
    }
}
