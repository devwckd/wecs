use wecs_core::system::SystemParam;

use crate::EventManager;

pub struct EventWriter<'e, E>
where
    E: 'static,
{
    event_manager: &'e mut EventManager<E>,
}

impl<'e, E> EventWriter<'e, E>
where
    E: 'static,
{
    pub fn dispatch_one(&mut self, event: E) {
        self.event_manager.events.push(event);
    }

    pub fn dispatch_many(&mut self, events: impl IntoIterator<Item = E>) {
        self.event_manager.events.extend(events);
    }
}

impl<'e, E> SystemParam for EventWriter<'e, E>
where
    E: 'static,
{
    type Item<'world> = EventWriter<'world, E>;

    fn get_param<'world>(world: wecs_core::world::UnsafeWorldCell<'world>) -> Self::Item<'world> {
        EventWriter {
            event_manager: world
                .world_mut()
                .get_resource_mut::<EventManager<E>>()
                .unwrap(),
        }
    }
}
