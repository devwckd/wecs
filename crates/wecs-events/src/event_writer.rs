use crate::EventManager;
use wecs_core::system::SystemParam;
use wecs_core::world::World;

pub struct EventWriter<'e, E>
where
    E: 'static,
{
    event_manager: &'e mut EventManager<E>,
}

impl<'e, E> EventDispatcher<E> for EventWriter<'e, E>
where
    E: 'static,
{
    fn dispatch_one(&mut self, event: E) {
        self.event_manager.events.push(event);
    }

    fn dispatch_many(&mut self, events: impl IntoIterator<Item=E>) {
        self.event_manager.events.extend(events);
    }
}

impl<E> EventDispatcher<E> for &mut World
where
    E: 'static,
{
    fn dispatch_one(&mut self, event: E) {
        if let Some(manager) = self.get_resource_mut::<EventManager<E>>() {
            manager.events.push(event)
        }
    }

    fn dispatch_many(&mut self, events: impl IntoIterator<Item=E>) {
        if let Some(manager) = self.get_resource_mut::<EventManager<E>>() {
            manager.events.extend(events)
        }
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

pub trait EventDispatcher<E> {
    fn dispatch_one(&mut self, event: E);

    fn dispatch_many(&mut self, events: impl IntoIterator<Item=E>);
}