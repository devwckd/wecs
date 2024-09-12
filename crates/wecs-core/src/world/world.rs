use std::ops::Index;

use crate::{
    archetype::Archetypes,
    component::{Component, ComponentList},
    entity::{Entities, EntityId, EntityRef, EntityRefMut},
    resource::{Resource, ResourceId, Resources},
    storage::{Table, Tables},
};

pub struct World {
    archetypes: Archetypes,
    entities: Entities,
    resources: Resources,
    tables: Tables,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            archetypes: Archetypes::new(),
            entities: Entities::new(),
            resources: Resources::new(),
            tables: Tables::new(),
        }
    }

    pub(crate) fn archetypes(&self) -> &Archetypes {
        &self.archetypes
    }

    pub(crate) fn entities(&self) -> &Entities {
        &self.entities
    }

    pub(crate) fn resources(&self) -> &Resources {
        &self.resources
    }

    pub(crate) fn tables(&self) -> &Tables {
        &self.tables
    }

    pub(crate) fn tables_mut(&mut self) -> &mut Tables {
        &mut self.tables
    }

    #[inline]
    pub fn init_resource<R>(&mut self) -> ResourceId
    where
        R: Resource + Default + 'static,
    {
        self.resources.init::<R>()
    }

    #[inline]
    pub fn insert_resource<R>(&mut self, resource: R) -> ResourceId
    where
        R: Resource + 'static,
    {
        self.resources.insert::<R>(resource)
    }

    #[inline]
    pub fn remove_resource<R>(&mut self) -> R
    where
        R: Resource + 'static,
    {
        self.resources.remove::<R>()
    }

    #[inline]
    pub fn get_resource<R>(&self) -> Option<&R>
    where
        R: Resource + 'static,
    {
        self.resources.get::<R>()
    }

    #[inline]
    pub fn get_or_init_resource<R>(&mut self) -> Option<&R>
    where
        R: Resource + Default + 'static,
    {
        self.resources.get_or_init::<R>()
    }

    #[inline]
    pub fn get_resource_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource + 'static,
    {
        self.resources.get_mut::<R>()
    }

    #[inline]
    pub fn get_or_init_resource_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource + Default + 'static,
    {
        self.resources.get_or_init_mut::<R>()
    }

    #[inline]
    pub fn spawn_entity<CL>(&mut self, components: CL) -> EntityId
    where
        CL: ComponentList,
    {
        let entity_id = self.entities.spawn_empty();
        let archetype_id = self.archetypes.get_or_init::<CL>();

        let mut table_id = self.archetypes.get_table_id(&archetype_id);
        if !table_id.is_init() {
            table_id = self.tables.init::<CL>();
            self.archetypes.update_table_id(&archetype_id, &table_id);
        }

        let table = self.tables.get_mut(&table_id);
        table.insert_components(entity_id, CL::get_ids(), components.get_component_values());

        self.entities.modify(entity_id, archetype_id);
        entity_id
    }

    #[inline]
    pub fn query<CL>(&self) -> Vec<EntityId>
    where
        CL: ComponentList,
    {
        let entities = self.entities();
        let archetypes = self.archetypes();
        entities
            .data
            .iter()
            .enumerate()
            .filter(|(index, data)| {
                !entities.free_ids.contains(index) && data.archetype_id.is_init() && {
                    let component_ids = &archetypes.get_data(&data.archetype_id).components_ids;

                    CL::get_ids().iter().all(|id| component_ids.contains(id))
                }
            })
            .map(|(index, data)| EntityId {
                index,
                gen: data.gen,
            })
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_entity(&self, entity_id: EntityId) -> Option<EntityRef> {
        let Some(archetype_id) = self.entities.get(&entity_id) else {
            return None;
        };

        if !archetype_id.is_init() {
            return None;
        }

        let table_id = self.archetypes.get_table_id(&archetype_id);

        Some(EntityRef::new(self, entity_id, table_id))
    }

    #[inline]
    pub(crate) fn get_entity_component_unchecked<C>(&self, entity_id: EntityId) -> &C
    where
        C: Component + 'static,
    {
        let Some(archetype_id) = self.entities.get(&entity_id) else {
            panic!()
        };

        if !archetype_id.is_init() {
            panic!()
        }

        let table_id = self.archetypes.get_table_id(&archetype_id);
        let table = self.tables.get(&table_id);
        let index = table.get_entity_index(&entity_id);
        return table.get_component_at::<C>(index);
    }

    #[inline]
    pub(crate) fn get_entity_component_mut_unchecked<C>(&mut self, entity_id: EntityId) -> &mut C
    where
        C: Component + 'static,
    {
        let Some(archetype_id) = self.entities.get(&entity_id) else {
            panic!()
        };

        if !archetype_id.is_init() {
            panic!()
        }

        let table_id = self.archetypes.get_table_id(&archetype_id);
        let table = self.tables.get_mut(&table_id);
        let index = table.get_entity_index(&entity_id);
        return table.get_component_at_mut::<C>(index);
    }

    #[inline]
    pub fn get_entity_mut(&mut self, entity_id: EntityId) -> Option<EntityRefMut> {
        let Some(archetype_id) = self.entities.get(&entity_id) else {
            return None;
        };

        if !archetype_id.is_init() {
            return None;
        }

        let table_id = self.archetypes.get_table_id(&archetype_id);

        Some(EntityRefMut::new(self, entity_id, table_id))
    }
}
