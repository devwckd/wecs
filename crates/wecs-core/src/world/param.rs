use crate::system::SystemParam;

use super::World;

impl<'w> SystemParam for &'w World {
    type Item<'world> = &'world World;

    fn get_param<'world>(world: super::UnsafeWorldCell<'world>) -> Self::Item<'world> {
        world.world()
    }
}

impl<'w> SystemParam for &'w mut World {
    type Item<'world> = &'world mut World;

    fn get_param<'world>(mut world: super::UnsafeWorldCell<'world>) -> Self::Item<'world> {
        world.world_mut()
    }
}
