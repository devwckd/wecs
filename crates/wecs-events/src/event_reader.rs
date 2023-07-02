use wecs_core::system::SystemParam;

use crate::EventManager;

pub struct EventReader<'e, E>
where
    E: 'static,
{
    events: &'e Vec<E>,
    index: usize,
}

impl<'e, E> Iterator for EventReader<'e, E>
where
    E: 'static,
{
    type Item = &'e E;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.events.get(self.index) {
            self.index += 1;
            return Some(event);
        }

        return None;
    }
}

impl<'e, E> SystemParam for EventReader<'e, E>
where
    E: 'static,
{
    type Item<'world> = EventReader<'world, E>;

    fn get_param<'world>(world: wecs_core::world::UnsafeWorldCell<'world>) -> Self::Item<'world> {
        EventReader {
            events: &world
                .world()
                .get_resource::<EventManager<E>>()
                .unwrap()
                .events,
            index: 0,
        }
    }
}
