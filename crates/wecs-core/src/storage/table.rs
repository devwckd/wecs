use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Index,
    pin::Pin,
};

use crate::{
    component::{Component, ComponentId, ComponentList},
    entity::EntityId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableId(usize);

impl TableId {
    pub const UNINIT: TableId = TableId(usize::MAX);

    pub fn is_init(&self) -> bool {
        self.0 != TableId::UNINIT.0
    }
}

#[derive(Debug)]
pub struct Tables {
    tables: Vec<Table>,
}

impl Default for Tables {
    fn default() -> Self {
        Self::new()
    }
}

impl Tables {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }

    pub fn init<CL>(&mut self) -> TableId
    where
        CL: ComponentList,
    {
        let index = self.tables.len();
        let mut table = Table::new();
        table.init_archetype_columns(CL::get_ids());
        self.tables.insert(index, table);

        TableId(index)
    }

    pub fn get(&self, table_id: &TableId) -> &Table {
        self.tables.get(table_id.0).expect("")
    }

    pub fn get_mut(&mut self, table_id: &TableId) -> &mut Table {
        self.tables.get_mut(table_id.0).expect("")
    }
}

#[derive(Debug)]
pub struct Table {
    columns: HashMap<ComponentId, Column>,
    entities: Vec<EntityId>,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            entities: Vec::new(),
        }
    }

    fn init_archetype_columns(&mut self, component_ids: Vec<ComponentId>) {
        for ele in component_ids {
            self.columns.insert(ele, Column::new());
        }
    }

    pub fn insert_components(
        &mut self,
        entity_id: EntityId,
        component_ids: Vec<ComponentId>,
        component_values: Vec<Box<dyn Any>>,
    ) {
        let index = self.entities.len();
        self.entities.insert(index, entity_id);
        for (component_id, component_value) in component_ids.iter().zip(component_values) {
            self.columns
                .get_mut(component_id)
                .expect("")
                .data
                .insert(index, component_value);
        }
    }

    pub(crate) fn get_entity_index(&self, entity_id: &EntityId) -> usize {
        self.entities
            .iter()
            .enumerate()
            .find(|(_, id)| *id == entity_id)
            .expect("")
            .0
    }

    pub(crate) fn get_component_at<C>(&self, index: usize) -> &C
    where
        C: Component + 'static,
    {
        self.columns
            .get(&TypeId::of::<C>())
            .expect("")
            .data
            .get(index)
            .expect("")
            .downcast_ref::<C>()
            .expect("")
    }

    pub(crate) fn get_component_at_mut<C>(&mut self, index: usize) -> &mut C
    where
        C: Component + 'static,
    {
        self.columns
            .get_mut(&TypeId::of::<C>())
            .expect("")
            .data
            .get_mut(index)
            .expect("")
            .downcast_mut::<C>()
            .expect("")
    }
}

#[derive(Debug)]
pub struct Column {
    data: Vec<Box<dyn Any>>,
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

impl Column {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}
