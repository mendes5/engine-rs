use anymap::AnyMap;
use std::any::TypeId;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Entity {
    pub(crate) components: AnyMap,
    pub(crate) shape: HashSet<TypeId>,
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: AnyMap::new(),
            shape: HashSet::new(),
        }
    }

    pub fn deregister_component<T: 'static>(&mut self) {
        self.components.remove::<T>();
        self.shape.remove(&TypeId::of::<T>());
    }

    fn register_component<T: 'static>(&mut self, value: T) {
        self.components.insert::<T>(value);
    }

    pub fn remove<T: 'static>(&mut self) {
        self.deregister_component::<T>()
    }

    pub fn with<T: 'static>(mut self, value: T) -> Self {
        self.shape.insert(TypeId::of::<T>());
        self.register_component(value);
        self
    }

    pub fn set<T: 'static>(&mut self, value: T) {
        self.components.insert::<T>(value);
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.components.get::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&'static mut T> {
        unsafe { std::mem::transmute(self.components.get_mut::<T>()) }
    }

    pub fn has<A: 'static>(&self) -> bool {
        self.components.contains::<A>()
    }
}
