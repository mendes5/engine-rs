use super::entity::Entity;
use super::resource_registry::ResourceRegistry;
use super::shape::EntityShape;
use anymap::AnyMap;
use glutin::event::Event;
use std::any::TypeId;

pub type Resources = AnyMap;

type SystemFn = fn(&mut Entity, resourceRegistry: &mut ResourceRegistry, value: &RunSystemPhase);
type ServiceFn = fn(resourceRegistry: &mut ResourceRegistry, value: &RunSystemPhase);

pub enum ServicePhase {
    Before,
    After,
}

pub enum SystemPhase {
    Update,
    Event,
    Tick,
    Render,
}

#[derive(Clone)]
pub enum RunSystemPhase {
    Update(TypeId),
    Event(Event<'static, ()>),
    Tick,
    Render,
}

pub struct Service {
    pub(crate) phase: SystemPhase,
    pub(crate) calls: ServiceFn,
    pub(crate) update_key: Option<TypeId>,
}

impl Service {
    pub fn at_update<T: 'static>(_update_key: T, calls: ServiceFn) -> Self {
        Self {
            phase: SystemPhase::Update,
            calls,
            update_key: Some(TypeId::of::<T>()),
        }
    }
    pub fn at_event(calls: ServiceFn) -> Self {
        Self {
            phase: SystemPhase::Event,
            calls,
            update_key: None,
        }
    }
    pub fn at_tick(calls: ServiceFn) -> Self {
        Self {
            phase: SystemPhase::Tick,
            calls,
            update_key: None,
        }
    }
    pub fn at_render(calls: ServiceFn) -> Self {
        Self {
            phase: SystemPhase::Render,
            calls,
            update_key: None,
        }
    }
}

pub struct System {
    pub(crate) phase: SystemPhase,
    pub(crate) query: EntityShape,
    pub(crate) calls: SystemFn,
    pub(crate) update_key: Option<TypeId>,
}

impl System {
    pub fn at_update<T: 'static>(_event_key: T, query: EntityShape, calls: SystemFn) -> Self {
        Self {
            phase: SystemPhase::Update,
            query,
            calls,
            update_key: Some(TypeId::of::<T>()),
        }
    }
    pub fn at_event(query: EntityShape, calls: SystemFn) -> Self {
        Self {
            phase: SystemPhase::Event,
            query,
            calls,
            update_key: None,
        }
    }
    pub fn at_tick(query: EntityShape, calls: SystemFn) -> Self {
        Self {
            phase: SystemPhase::Tick,
            query,
            calls,
            update_key: None,
        }
    }
    pub fn at_render(query: EntityShape, calls: SystemFn) -> Self {
        Self {
            phase: SystemPhase::Render,
            query,
            calls,
            update_key: None,
        }
    }
}

pub struct SystemRunner {
    pub(crate) update: Vec<System>,
    pub(crate) before_update: Vec<Service>,
    pub(crate) after_update: Vec<Service>,
    pub(crate) tick: Vec<System>,
    pub(crate) before_tick: Vec<Service>,
    pub(crate) after_tick: Vec<Service>,
    pub(crate) render: Vec<System>,
    pub(crate) before_render: Vec<Service>,
    pub(crate) after_render: Vec<Service>,
    pub(crate) event: Vec<System>,
    pub(crate) before_event: Vec<Service>,
    pub(crate) after_event: Vec<Service>,
}

impl SystemRunner {
    pub fn new() -> Self {
        Self {
            update: Vec::new(),
            before_update: Vec::new(),
            after_update: Vec::new(),
            tick: Vec::new(),
            before_tick: Vec::new(),
            after_tick: Vec::new(),
            render: Vec::new(),
            before_render: Vec::new(),
            after_render: Vec::new(),
            event: Vec::new(),
            before_event: Vec::new(),
            after_event: Vec::new(),
        }
    }

    pub fn add(&mut self, system: System) {
        match system.phase {
            SystemPhase::Update => self.update.push(system),
            SystemPhase::Event => self.event.push(system),
            SystemPhase::Tick => self.tick.push(system),
            SystemPhase::Render => self.render.push(system),
        }
    }

    pub fn add_service(&mut self, service: Service, phase: ServicePhase) {
        match phase {
            ServicePhase::Before => match service.phase {
                SystemPhase::Update => self.before_update.push(service),
                SystemPhase::Event => self.before_event.push(service),
                SystemPhase::Tick => self.before_tick.push(service),
                SystemPhase::Render => self.before_render.push(service),
            },
            ServicePhase::After => match service.phase {
                SystemPhase::Update => self.after_update.push(service),
                SystemPhase::Event => self.after_event.push(service),
                SystemPhase::Tick => self.after_tick.push(service),
                SystemPhase::Render => self.after_render.push(service),
            },
        }
    }
}
