use crate::events::EventChannel;
use crate::modules::debug_info::{DebugInfo, DebugKey, FPSUpdateEvent};
use ecs::{Entity, EntityShapeBuilder, ResourceRegistry, RunSystemPhase, System, ECS};
use renderer::{RenderComponent, TextureStorage, RendererDevice};
use skia_safe::Color;
use std::time::Duration;

pub struct IsDebugUITag;

fn render_ui(entity: &mut Entity, resources: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let dbg_info = resources.get_mut::<DebugInfo>().unwrap();
    let render = entity.get_mut::<RenderComponent>().unwrap();
    let device = resources.get_mut::<RendererDevice>().unwrap();

    if let Some(texture_idx) = render.textures.get_mut(0) {
        let mut texture = device.get_texture_mut(*texture_idx).unwrap();
        texture.needs_update = true;
        match &mut texture.storage {
            TextureStorage::Canvas2D(canvas) => {
                canvas.clear(Color::TRANSPARENT);

                if let Some(fps) = dbg_info.get(DebugKey::CurrentFPS) {
                    canvas.text(15.0, 15.0, &format!("FPS: {:?}", fps).to_owned());
                }

                if let Some(camera) = dbg_info.get(DebugKey::CameraPosition) {
                    canvas.text(15.0, 35.0, &format!("Position: {:?}", camera).to_owned());
                }

                if let Some(block) = dbg_info.get(DebugKey::CurrentBlock) {
                    canvas.text(15.0, 55.0, &format!("Block: {:?}", block).to_owned());
                }

                if let Some(block_pos) = dbg_info.get(DebugKey::BlockPos) {
                    canvas.text(15.0, 75.0, &format!("Position: {:?}", block_pos).to_owned());
                }
            }
            _ => (),
        }
    }
}

pub fn load(ecs: &mut ECS) {
    ecs.add_system(System::at_update(
        FPSUpdateEvent,
        EntityShapeBuilder::new()
            .with::<IsDebugUITag>()
            .with::<RenderComponent>()
            .build(),
        render_ui,
    ));

    let event_channel = ecs.resources.get_mut::<EventChannel>().unwrap();

    event_channel.schedule(FPSUpdateEvent, Duration::from_millis(1000), true);
}
