use crate::{
    system::{BoxedSystem, IntoSystem},
    world::World,
};

pub struct Schedule {
    systems: Vec<BoxedSystem>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn with_system<Marker, S>(mut self, system: S) -> Self
    where
        S: IntoSystem<Marker> + 'static,
        Marker: 'static,
    {
        self.systems.push(Box::new(system.into_system()));
        self
    }

    pub fn add_system<Marker, S>(&mut self, system: S)
    where
        S: IntoSystem<Marker> + 'static,
        Marker: 'static,
    {
        self.systems.push(Box::new(system.into_system()));
    }

    pub fn run(&mut self, world: &mut World) {
        for ele in self.systems.iter_mut() {
            ele.run(world);
        }
    }
}
