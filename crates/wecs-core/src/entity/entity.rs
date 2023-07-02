use crate::archetype::ArchetypeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EntityId {
    pub(crate) index: usize,
    pub(crate) gen: usize,
}

pub(crate) struct EntityData {
    pub(crate) gen: usize,
    pub(crate) archetype_id: ArchetypeId,
}

pub struct Entities {
    pub(crate) data: Vec<EntityData>,
    pub(crate) free_ids: Vec<usize>,
}

impl Default for Entities {
    fn default() -> Self {
        Self::new()
    }
}

impl Entities {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            free_ids: Vec::new(),
        }
    }

    pub fn spawn_empty(&mut self) -> EntityId {
        if let Some(index) = self.free_ids.pop() {
            let data = self.data.get_mut(index).expect("");
            data.gen += 1;
            data.archetype_id = ArchetypeId::UNINIT; // TODO: set not init

            return EntityId {
                index,
                gen: data.gen,
            };
        }

        let index = self.data.len();
        self.data.insert(
            index,
            EntityData {
                gen: 0,
                archetype_id: ArchetypeId::UNINIT, // TODO: set not init
            },
        );

        EntityId { index, gen: 0 }
    }

    pub fn spawn(&mut self, archetype_id: ArchetypeId) -> EntityId {
        if let Some(index) = self.free_ids.pop() {
            let data = self.data.get_mut(index).expect("");
            data.gen += 1;
            data.archetype_id = archetype_id;

            return EntityId {
                index,
                gen: data.gen,
            };
        }

        let index = self.data.len();
        self.data.insert(
            index,
            EntityData {
                gen: 0,
                archetype_id: archetype_id,
            },
        );

        EntityId { index, gen: 0 }
    }

    pub fn modify(&mut self, entity_id: EntityId, archetype_id: ArchetypeId) {
        if let Some(data) = self.data.get_mut(entity_id.index) {
            if data.gen != entity_id.gen {
                panic!()
            }

            data.archetype_id = archetype_id;
        }
    }

    pub fn get(&self, entity_id: &EntityId) -> Option<ArchetypeId> {
        if let Some(data) = self.data.get(entity_id.index) {
            if data.gen != entity_id.gen {
                return None;
            }

            return Some(data.archetype_id);
        }

        None
    }
}
