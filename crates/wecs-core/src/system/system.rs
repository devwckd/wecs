use crate::world::World;

pub type BoxedSystem = Box<dyn System>;

pub trait System {
    fn run(&mut self, world: &mut World);
}

pub trait IntoSystem<Marker> {
    type System: System;

    fn into_system(self) -> Self::System;
}
