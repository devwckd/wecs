use std::any::{Any, TypeId};

use crate::query::QueryComponent;

pub trait Component {}

pub type ComponentId = TypeId;

pub trait ComponentList {
    fn get_ids() -> Vec<ComponentId>;
    fn get_component_values(self) -> Vec<Box<dyn Any>>;
}

impl<T1> ComponentList for T1
where
    T1: Component + 'static,
{
    fn get_ids() -> Vec<ComponentId> {
        vec![TypeId::of::<T1>()]
    }

    fn get_component_values(self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self)]
    }
}

macro_rules! impl_cl_for {
    ( $($n:ident),* ) => {
        #[allow(non_snake_case)]
        impl<$($n,)*> ComponentList for ($($n,)*)
        where
        $($n: Component + 'static,)*
        {
            fn get_ids() -> Vec<ComponentId> {
                vec![$(TypeId::of::<$n>(),)*]
            }

            fn get_component_values(self) -> Vec<Box<dyn Any>> {
                let ($($n,)*) = self;
                vec![$(Box::new($n)),*]
            }
        }
    };
}

impl_cl_for!(T1);
impl_cl_for!(T1, T2);
impl_cl_for!(T1, T2, T3);
impl_cl_for!(T1, T2, T3, T4);
impl_cl_for!(T1, T2, T3, T4, T5);
impl_cl_for!(T1, T2, T3, T4, T5, T6);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_cl_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);

impl<C> QueryComponent for &C
where
    C: Component + 'static,
{
    type Item<'world> = &'world C;

    fn get_component_ids() -> Vec<ComponentId> {
        vec![TypeId::of::<C>()]
    }

    #[inline]
    fn get_component<'world>(
        world: crate::world::UnsafeWorldCell<'world>,
        entity_id: crate::entity::EntityId,
    ) -> Self::Item<'world> {
        world.world().get_entity_component_unchecked::<C>(entity_id)
    }
}

impl<C> QueryComponent for &mut C
where
    C: Component + 'static,
{
    type Item<'world> = &'world mut C;

    fn get_component_ids() -> Vec<ComponentId> {
        vec![TypeId::of::<C>()]
    }

    #[inline]
    fn get_component<'world>(
        world: crate::world::UnsafeWorldCell<'world>,
        entity_id: crate::entity::EntityId,
    ) -> Self::Item<'world> {
        world
            .world_mut()
            .get_entity_component_mut_unchecked::<C>(entity_id)
    }
}
