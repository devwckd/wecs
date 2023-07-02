use crate::{component::Component, storage::TableId, world::World};

use super::EntityId;

pub struct EntityRef<'w> {
    world: &'w World,
    entity_id: EntityId,
    table_id: TableId,
}

impl<'w> EntityRef<'w> {
    pub fn new(world: &'w World, entity_id: EntityId, table_id: TableId) -> Self {
        Self {
            world,
            entity_id,
            table_id,
        }
    }

    pub fn get<C>(&'w self) -> &'w C
    where
        C: Component + 'static,
    {
        let table = self.world.tables().get(&self.table_id);
        let index = table.get_entity_index(&self.entity_id);
        table.get_component_at::<C>(index)
    }
}

pub struct EntityRefMut<'w> {
    world: &'w mut World,
    entity_id: EntityId,
    table_id: TableId,
}

impl<'w> EntityRefMut<'w> {
    pub fn new(world: &'w mut World, entity_id: EntityId, table_id: TableId) -> Self {
        Self {
            world,
            entity_id,
            table_id,
        }
    }

    pub fn get<C>(&'w self) -> &'w C
    where
        C: Component + 'static,
    {
        let table = self.world.tables().get(&self.table_id);
        let index = table.get_entity_index(&self.entity_id);
        table.get_component_at::<C>(index)
    }

    pub fn get_mut<C>(&'w mut self) -> &'w mut C
    where
        C: Component + 'static,
    {
        let table = self.world.tables_mut().get_mut(&self.table_id);
        let index = table.get_entity_index(&self.entity_id);
        table.get_component_at_mut::<C>(index)
    }
}
