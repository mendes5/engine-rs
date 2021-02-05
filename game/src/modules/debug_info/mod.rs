use crate::time::TimeContext;
use ecs::{ResourceRegistry, RunSystemPhase, Service, ECS};
use std::collections::hash_map::HashMap;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash)]
pub enum DebugKey {
    CurrentFPS,
    CameraPosition,
    CurrentBlock,
    BlockPos,
}

pub struct DebugInfo {
    map: HashMap<DebugKey, String>,
}

pub struct FPSUpdateEvent;

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: DebugKey, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: DebugKey) -> Option<&String> {
        self.map.get(&key)
    }
}

fn before_frame(resources: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let time_context = resources.get_mut::<TimeContext>().unwrap();
    let dbg_info = resources.get_mut::<DebugInfo>().unwrap();
    dbg_info.set(DebugKey::CurrentFPS, time_context.get_fps().to_string());
}

pub fn load(ecs: &mut ECS) {
    ecs.resources.set(DebugInfo::new());
    ecs.add_before_service(Service::at_render(before_frame));
}
