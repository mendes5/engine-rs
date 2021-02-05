mod events;
mod input;
mod internal;
mod modules;
mod time;
mod window_context;

use ecs::{RunSystemPhase, ECS};
use events::{EventChannel, EventSystem};
use modules::MODULE_LOADER;
use renderer::RendererDevice;

use window_context::WindowContext;
extern crate bulletrs;
extern crate cgmath;

fn main() {
  let mut ecs = ECS::new();
  let mut event_system = EventSystem::new();

  let window_context = WindowContext::from_event_loop(&event_system);
  let renderer = RendererDevice::from_window(&window_context.window_context);
  let event_channel = EventChannel::new();

  ecs.resources.set(renderer);
  ecs.resources.set(window_context);
  ecs.resources.set(event_channel);

  for loader in MODULE_LOADER.iter() {
    loader(&mut ecs);
  }

  while event_system.running {
    event_system.update_events(|event| ecs.run_systems(RunSystemPhase::Event(event)));

    ecs
      .resources
      .get_mut::<EventChannel>()
      .unwrap()
      .update_timers(|event| ecs.run_systems(RunSystemPhase::Update(event)));

    ecs.run_systems(RunSystemPhase::Render);
  }
}
