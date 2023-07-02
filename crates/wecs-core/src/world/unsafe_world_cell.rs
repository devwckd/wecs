use std::{cell::UnsafeCell, marker::PhantomData};

use crate::resource::Resource;

use super::World;

#[derive(Clone, Copy)]
pub struct UnsafeWorldCell<'w> {
    world_ptr: *mut World,
    _phantom_data: PhantomData<(&'w World, &'w UnsafeCell<World>)>,
}

impl<'w> UnsafeWorldCell<'w> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world_ptr: world as *mut World,
            _phantom_data: PhantomData,
        }
    }

    pub fn world(self) -> &'w World {
        unsafe { &*self.world_ptr }
    }

    pub fn world_mut(self) -> &'w mut World {
        unsafe { &mut *self.world_ptr }
    }
}
