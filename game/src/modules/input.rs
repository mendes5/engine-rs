use crate::input::keyboard::KeyboardState;
use ecs::{ResourceRegistry, RunSystemPhase, Service, ECS};
use glutin::event::{Event, WindowEvent};

fn emit_events(resources: &mut ResourceRegistry, value: &RunSystemPhase) {
    let keyboard_state = resources.get_mut::<KeyboardState>().unwrap();

    match value {
        RunSystemPhase::Event(event) => match event.clone() {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input,
                    is_synthetic: false,
                    ..
                } => {
                    keyboard_state.handle_keyboard_input(input);
                }
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}

pub fn load(ecs: &mut ECS) {
    ecs.resources.set(KeyboardState::new());
    ecs.add_before_service(Service::at_event(emit_events));
}
