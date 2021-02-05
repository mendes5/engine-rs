use crate::input::keyboard::KeyboardState;
use crate::internal::FPSControls;
use crate::modules::debug_info::{DebugInfo, DebugKey};
use crate::modules::render_all::ViewportUBO;
use crate::time::TimeContext;
use ecs::{ResourceRegistry, RunSystemPhase, Service, ECS};
use glutin::event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent};

fn emit_events(resources: &mut ResourceRegistry, value: &RunSystemPhase) {
    let active_controls = resources.get_mut::<FPSControls>().unwrap();
    let time_context = resources.get_mut::<TimeContext>().unwrap();
    let keyboard_state = resources.get_mut::<KeyboardState>().unwrap();
    let dbg_info = resources.get_mut::<DebugInfo>().unwrap();

    match value {
        RunSystemPhase::Event(event) => match event.clone() {
            Event::MainEventsCleared => {
                active_controls
                    .read_last_keyboard(keyboard_state, time_context.last_delta() as f32);

                dbg_info.set(
                    DebugKey::CameraPosition,
                    format!(
                        "X: {:.2} Y: {:.2} Z: {:.2}",
                        active_controls.camera.position.x,
                        active_controls.camera.position.y,
                        active_controls.camera.position.z
                    )
                    .to_owned(),
                );
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Focused(is_focused) => {
                    if !is_focused {
                        active_controls.camera.set_enabled(false);
                    }
                }
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => {
                    active_controls.camera.set_enabled(true);
                }
                WindowEvent::KeyboardInput {
                    input,
                    is_synthetic: false,
                    ..
                } => {
                    if let Some(x) = input.virtual_keycode {
                        if x == VirtualKeyCode::Escape {
                            active_controls.camera.set_enabled(false);
                        }
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    active_controls.camera.on_resize(physical_size);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let h_width = active_controls.camera.f_width / 2.0;
                    let h_height = active_controls.camera.f_height / 2.0;

                    active_controls.camera.on_mouse_step(
                        (position.x - h_width as f64) as f32,
                        (position.y - h_height as f64) as f32,
                    );
                }
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}

fn before_frame(resources: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let active_controls = resources.get_mut::<FPSControls>().unwrap();
    let viewport_ubo = resources.get_mut::<ViewportUBO>().unwrap();

    active_controls.camera.write_matrix(
        &mut viewport_ubo.data[0].view,
        &mut viewport_ubo.data[0].projection,
    );
}

pub fn load(ecs: &mut ECS) {
    ecs.resources.set(FPSControls::new());
    ecs.add_before_service(Service::at_event(emit_events));
    ecs.add_before_service(Service::at_render(before_frame));
}
