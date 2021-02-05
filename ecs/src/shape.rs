use std::any::TypeId;

#[derive(Debug)]
pub struct EntityShape(pub(crate) Vec<TypeId>);

impl EntityShape {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn add(&mut self, type_id: TypeId) {
        if !self.0.contains(&type_id) {
            self.0.push(type_id);
        }
    }
}

pub struct EntityShapeBuilder(EntityShape);

impl EntityShapeBuilder {
    pub fn new() -> Self {
        Self(EntityShape::new())
    }

    pub fn with<T: 'static>(mut self) -> Self {
        self.0.add(TypeId::of::<T>());
        self
    }

    pub fn build(self) -> EntityShape {
        self.0
    }
}
