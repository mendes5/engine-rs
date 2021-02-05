use super::{
    Entity, EntityShape, ResourceRegistry, RunSystemPhase, Service, ServicePhase, System,
    SystemRunner,
};
use generational_arena::Arena;
use std::fmt;

pub struct ECS {
    pub(crate) entities: Arena<Entity>,
    systems: SystemRunner,
    pub resources: ResourceRegistry,
}

impl fmt::Debug for ECS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<ECS>")
    }
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entities: Arena::new(),
            systems: SystemRunner::new(),
            resources: ResourceRegistry::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    pub fn add_system(&mut self, system: System) {
        self.systems.add(system);
    }

    pub fn add_before_service(&mut self, system: Service) {
        self.systems.add_service(system, ServicePhase::Before);
    }

    pub fn add_after_service(&mut self, system: Service) {
        self.systems.add_service(system, ServicePhase::After);
    }

    pub fn run_systems(&mut self, phase: RunSystemPhase) {
        match phase.clone() {
            RunSystemPhase::Update(type_id) => {
                for service in self.systems.before_update.iter_mut() {
                    if let Some(update_key) = service.update_key {
                        if update_key == type_id {
                            (service.calls)(&mut self.resources, &phase);
                        }
                    }
                }
                for (_, entity) in self.entities.iter_mut() {
                    for system in self.systems.update.iter_mut() {
                        if let Some(update_key) = system.update_key {
                            if update_key == type_id {
                                if entity.components.raw_contains_all(&system.query.0) {
                                    (system.calls)(entity, &mut self.resources, &phase);
                                }
                            }
                        }
                    }
                }
                for service in self.systems.after_update.iter_mut() {
                    if let Some(update_key) = service.update_key {
                        if update_key == type_id {
                            (service.calls)(&mut self.resources, &phase);
                        }
                    }
                }
            }
            RunSystemPhase::Event(_event) => {
                for system in self.systems.before_event.iter_mut() {
                    (system.calls)(&mut self.resources, &phase);
                }
                for (_, entity) in self.entities.iter_mut() {
                    for system in self.systems.event.iter_mut() {
                        if entity.components.raw_contains_all(&system.query.0) {
                            (system.calls)(entity, &mut self.resources, &phase);
                        }
                    }
                }
                for system in self.systems.after_event.iter_mut() {
                    (system.calls)(&mut self.resources, &phase);
                }
            }
            RunSystemPhase::Tick => {
                for system in self.systems.before_tick.iter_mut() {
                    (system.calls)(&mut self.resources, &phase);
                }
                for (_, entity) in self.entities.iter_mut() {
                    for system in self.systems.tick.iter_mut() {
                        if entity.components.raw_contains_all(&system.query.0) {
                            (system.calls)(entity, &mut self.resources, &phase);
                        }
                    }
                }
                for system in self.systems.after_tick.iter_mut() {
                    (system.calls)(&mut self.resources, &phase);
                }
            }
            RunSystemPhase::Render => {
                for system in self.systems.before_render.iter_mut() {
                    (system.calls)(&mut self.resources, &phase);
                }
                for (_, entity) in self.entities.iter_mut() {
                    for system in self.systems.render.iter_mut() {
                        if entity.components.raw_contains_all(&system.query.0) {
                            (system.calls)(entity, &mut self.resources, &phase);
                        }
                    }
                }
                for system in self.systems.after_render.iter_mut() {
                    (system.calls)(&mut self.resources, &phase);
                }
            }
        }
    }

    pub fn query_exact(&mut self, shape: &EntityShape) -> Vec<&mut Entity> {
        let mut matching: Vec<&mut Entity> = Vec::new();

        for (_, entity) in self.entities.iter_mut() {
            if entity.components.raw_contains_all(&shape.0) {
                matching.push(entity);
            }
        }

        matching
    }
}
