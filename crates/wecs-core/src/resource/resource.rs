use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait Resource {}

pub struct ResourceId(usize);

struct ResourceData {
    resource: Box<dyn Any>,
}

pub struct Resources {
    data: Vec<ResourceData>,
    id_mappings: HashMap<TypeId, usize>,
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

impl Resources {
    pub fn new() -> Self {
        Self {
            data: Vec::default(),
            id_mappings: HashMap::default(),
        }
    }

    pub fn init<R>(&mut self) -> ResourceId
    where
        R: Resource + Default + 'static,
    {
        self.insert(R::default())
    }

    pub fn insert<R>(&mut self, resource: R) -> ResourceId
    where
        R: Resource + 'static,
    {
        let type_id = TypeId::of::<R>();

        let index = self.id_mappings.entry(type_id).or_insert(self.data.len());
        self.data.insert(
            *index,
            ResourceData {
                resource: Box::new(resource),
            },
        );

        ResourceId(*index)
    }

    pub fn remove<R>(&mut self)
    where
        R: Resource + 'static,
    {
        let type_id = TypeId::of::<R>();

        if let Some(index) = self.id_mappings.remove(&type_id) {
            self.data.remove(index);
        };
    }

    pub fn get<R>(&self) -> Option<&R>
    where
        R: Resource + 'static,
    {
        return if let Some(index) = self.id_mappings.get(&TypeId::of::<R>()) {
            self.data.get(*index).map(|data| {
                data.resource
                    .downcast_ref::<R>()
                    .expect("could not cast resource")
            })
        } else {
            None
        };
    }

    pub fn get_or_init<R>(&mut self) -> Option<&R>
    where
        R: Resource + Default + 'static,
    {
        return if let Some(index) = self.id_mappings.get(&TypeId::of::<R>()) {
            self.data.get(*index).map(|data| {
                data.resource
                    .downcast_ref::<R>()
                    .expect("could not cast resource")
            })
        } else {
            self.init::<R>();
            self.get::<R>()
        };
    }

    pub fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource + 'static,
    {
        return if let Some(index) = self.id_mappings.get(&TypeId::of::<R>()) {
            self.data.get_mut(*index).map(|data| {
                data.resource
                    .downcast_mut::<R>()
                    .expect("could not cast resource")
            })
        } else {
            None
        };
    }

    pub fn get_or_init_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource + Default + 'static,
    {
        return if let Some(index) = self.id_mappings.get(&TypeId::of::<R>()) {
            self.data.get_mut(*index).map(|data| {
                data.resource
                    .downcast_mut::<R>()
                    .expect("could not cast resource")
            })
        } else {
            self.init::<R>();
            self.get_mut::<R>()
        };
    }
}
