use wecs_core::resource::ResMut;

pub struct EventManager<E>
where
    E: 'static,
{
    pub(crate) events: Vec<E>,
}

impl<E> wecs_core::resource::Resource for EventManager<E> where E: 'static {}

impl<E> Default for EventManager<E>
where
    E: 'static,
{
    fn default() -> Self {
        Self { events: Vec::new() }
    }
}

impl<E> EventManager<E>
where
    E: 'static,
{
    pub fn clear(mut manager: ResMut<EventManager<E>>) {
        manager.events.clear();
    }
}
