use crate::time::TimeContext;
use ecs::{ResourceRegistry, RunSystemPhase, Service, ECS};

fn after_frame(resources: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let time_context = resources.get_mut::<TimeContext>().unwrap();
    time_context.tick();
}

pub fn load(ecs: &mut ECS) {
    ecs.resources.set(TimeContext::new());
    ecs.add_after_service(Service::at_render(after_frame));
}
