use std::marker::PhantomData;

use crate::{entity::EntityId, system::SystemParam, world::UnsafeWorldCell};

use super::QueryComponent;

pub struct Query<'world, Q>
where
    Q: QueryComponent,
{
    unsafe_world_cell: UnsafeWorldCell<'world>,
    entity_ids: Vec<EntityId>,
    _phantom_data: PhantomData<Q>,
}

impl<'world, Q> Iterator for Query<'world, Q>
where
    Q: QueryComponent,
{
    type Item = Q::Item<'world>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity_id) = self.entity_ids.pop() {
            let item = Q::get_component(self.unsafe_world_cell, entity_id);
            return Some(item);
        }

        None
    }
}

impl<'w, Q> SystemParam for Query<'w, Q>
where
    Q: QueryComponent,
{
    type Item<'world> = Query<'world, Q>;

    fn get_param<'world>(unsafe_world_cell: UnsafeWorldCell<'world>) -> Self::Item<'world> {
        let entity_ids = {
            let world = unsafe_world_cell.world();
            let entities = world.entities();
            let archetypes = world.archetypes();
            entities
                .data
                .iter()
                .enumerate()
                .filter(|(index, data)| {
                    !entities.free_ids.contains(index) && data.archetype_id.is_init() && {
                        let component_ids = &archetypes.get_data(&data.archetype_id).components_ids;

                        Q::get_component_ids()
                            .iter()
                            .all(|id| component_ids.contains(id))
                    }
                })
                .map(|(index, data)| EntityId {
                    index,
                    gen: data.gen,
                })
                .collect::<Vec<_>>()
        };
        Query {
            unsafe_world_cell,
            entity_ids,
            _phantom_data: PhantomData,
        }
    }
}
