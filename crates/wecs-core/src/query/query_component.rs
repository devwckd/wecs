use crate::{
    component::{Component, ComponentId, ComponentList},
    entity::EntityId,
    world::UnsafeWorldCell,
};

pub trait QueryComponent {
    type Item<'world>: QueryComponent;

    fn get_component_ids() -> Vec<ComponentId>;

    fn get_component<'world>(
        world: UnsafeWorldCell<'world>,
        entity_id: EntityId,
    ) -> Self::Item<'world>;
}

macro_rules! impl_qc_for {
    ( $($n:ident),* ) => {
        #[allow(non_snake_case)]
        impl<$($n,)*> QueryComponent for ($($n,)*)
        where $($n: QueryComponent,)* {
            type Item<'world> = ($($n::Item<'world>,)*);

            fn get_component_ids() -> Vec<ComponentId> {
                let mut vec = Vec::new();

                $(vec.extend($n::get_component_ids());)*

                vec
            }

            fn get_component<'world>(
                world: UnsafeWorldCell<'world>,
                entity_id: EntityId,
            ) -> Self::Item<'world> {
                $(let $n = $n::get_component(world, entity_id);)*

                ($($n,)*)
            }
        }
    };
}

impl_qc_for!(T1);
