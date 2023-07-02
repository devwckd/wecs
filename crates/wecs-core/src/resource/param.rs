use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::system::SystemParam;

use super::Resource;

pub struct Res<'w, R>
where
    R: Resource + 'static,
{
    resource: &'w R,
    // _phantom_data: PhantomData<&'w R>,
}

impl<'w, R> Deref for Res<'w, R>
where
    R: Resource + 'static,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<'w, R> SystemParam for Res<'w, R>
where
    R: Resource + 'static,
{
    type Item<'world> = Res<'world, R>;

    fn get_param<'world>(world: crate::world::UnsafeWorldCell<'world>) -> Self::Item<'world> {
        Res {
            resource: world.world().get_resource::<R>().unwrap(),
            // _phantom_data: PhantomData,
        }
    }
}

pub struct ResMut<'w, R>
where
    R: Resource + 'static,
{
    resource: &'w mut R,
    // _phantom_data: PhantomData<&'w R>,
}

impl<'w, R> Deref for ResMut<'w, R>
where
    R: Resource + 'static,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<'w, R> DerefMut for ResMut<'w, R>
where
    R: Resource + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource
    }
}

impl<'w, R> SystemParam for ResMut<'w, R>
where
    R: Resource + 'static,
{
    type Item<'world> = ResMut<'world, R>;

    fn get_param<'world>(world: crate::world::UnsafeWorldCell<'world>) -> Self::Item<'world> {
        ResMut {
            resource: world.world_mut().get_resource_mut::<R>().unwrap(),
            // _phantom_data: PhantomData,
        }
    }
}
