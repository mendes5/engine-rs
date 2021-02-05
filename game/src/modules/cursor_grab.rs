use crate::window_context::WindowContext;
use ecs::{ResourceRegistry, RunSystemPhase, Service, ECS};

use glutin::event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent};

fn emit_events(resources: &mut ResourceRegistry, value: &RunSystemPhase) {
    let window_context = resources.get_mut::<WindowContext>().unwrap();

    match value {
        RunSystemPhase::Event(event) => match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Focused(is_focused) => {
                    if !is_focused {
                        window_context.set_cursor_grab(false);
                    }
                }
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => {
                    window_context.set_cursor_grab(true);
                }
                WindowEvent::KeyboardInput {
                    input,
                    is_synthetic: false,
                    ..
                } => {
                    if let Some(key) = input.virtual_keycode {
                        if key == VirtualKeyCode::Escape {
                            window_context.set_cursor_grab(false);
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}

pub fn load(ecs: &mut ECS) {
    ecs.add_before_service(Service::at_event(emit_events));
}
