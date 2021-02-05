use crate::window_context::WindowContext;
use ecs::{Entity, EntityShapeBuilder, ResourceRegistry, RunSystemPhase, Service, System, ECS};
use glutin::event::{Event, WindowEvent};
use renderer::{RenderComponent, RendererDevice, TextureStorage};

pub struct ViewportSizedUITag;

fn resize_uis(entity: &mut Entity, resources: &mut ResourceRegistry, value: &RunSystemPhase) {
    match value {
        RunSystemPhase::Event(event) => match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    let device = resources.get_mut::<RendererDevice>().unwrap();
                    let render = entity.get_mut::<RenderComponent>().unwrap();
                    if let Some(texture_index) = render.textures.get_mut(0) {

                        let mut texture = device.get_texture_mut(*texture_index).unwrap();

                        texture.needs_update = true;


                        match &mut texture.storage {
                            TextureStorage::Canvas2D(canvas) => {
                                canvas.resize(
                                    physical_size.width as i32,
                                    physical_size.height as i32,
                                );
                            }
                            _ => (),
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

fn emit_events(resources: &mut ResourceRegistry, value: &RunSystemPhase) {
    let game_context = resources.get_mut::<WindowContext>().unwrap();
    let renderer_device = resources.get_mut::<RendererDevice>().unwrap();

    match value {
        RunSystemPhase::Event(event) => match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    game_context.resize(*physical_size);
                    renderer_device.resize(physical_size.width, physical_size.height);
                }
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}

pub fn load(ecs: &mut ECS) {
    ecs.add_system(System::at_event(
        EntityShapeBuilder::new()
            .with::<RenderComponent>()
            .with::<ViewportSizedUITag>()
            .build(),
        resize_uis,
    ));

    ecs.add_before_service(Service::at_event(emit_events));
}
