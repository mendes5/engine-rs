use anymap::AnyMap;
use std::marker::PhantomData;

pub struct ResourceRegistry {
    resources: AnyMap,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            resources: AnyMap::new(),
        }
    }

    pub fn tagged_set<K: 'static, T: 'static>(&mut self, resource: T) {
        let marker: std::marker::PhantomData<K> = std::marker::PhantomData;
        self.resources
            .insert::<(PhantomData<K>, T)>((marker, resource));
    }

    pub fn tagged_get_mut<K: 'static, T: 'static>(&mut self) -> Option<&'static mut T> {
        unsafe {
            if let Some(tuple) = self.resources.get_mut::<(PhantomData<K>, T)>() {
                let (_, b) = tuple;
                return Some(std::mem::transmute(b));
            }
            None
        }
    }

    pub fn tagged_get<K: 'static, T: 'static>(&self) -> Option<&T> {
        if let Some(tuple) = self.resources.get::<(PhantomData<K>, T)>() {
            return Some(&tuple.1);
        }
        None
    }

    pub fn set<T: 'static>(&mut self, resource: T) {
        self.resources.insert::<T>(resource);
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&'static mut T> {
        unsafe { std::mem::transmute(self.resources.get_mut::<T>()) }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }
}
