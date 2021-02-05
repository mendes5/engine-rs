mod vertex;
use crate::events::EventChannel;
use crate::internal::FPSControls;
use crate::modules::debug_info::{DebugInfo, DebugKey, FPSUpdateEvent};
use ecs::{Entity, EntityShapeBuilder, ResourceRegistry, RunSystemPhase, Service, System, ECS};
use glutin::event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent};
use renderer::{
    RenderComponent, RendererDevice, Texture, TextureFiltering, TextureStorage, TextureWrapping,
};
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use math::Vec3;
use vertex::Vertex;



#[derive(Debug, Deserialize)]
pub struct SpriteSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Deserialize)]
pub struct SpriteFrame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Deserialize)]
pub struct Sprite {
    pub filename: String,
    pub frame: SpriteFrame,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: SpriteFrame,
    pub sourceSize: SpriteSize,
}

struct IsChunkRenderer;

#[derive(Debug)]
enum BlockType {
    Full,
    Crossed,
}

#[derive(Debug)]
pub struct Block {
    pos: Vec3,
    id: usize,
    mode: BlockType,
}

pub struct Chunk {
    blocks: Vec<Block>,
}



fn render_ui(entity: &mut Entity, resources: &mut ResourceRegistry, valuex: &RunSystemPhase) {
    match valuex {
        RunSystemPhase::Event(event) => match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    if *button == MouseButton::Left && *state == ElementState::Pressed {
                        let context = resources.get_mut::<RendererDevice>().unwrap();
                        let sprites = resources.get_mut::<Vec<Sprite>>().unwrap();
                        let active_controls = resources.get_mut::<FPSControls>().unwrap();
                        let chunk = resources.get_mut::<Chunk>().unwrap();
                        let block_idx = resources.get_mut::<BlockIndex>().unwrap();

                        let render = entity.get_mut::<RenderComponent>().unwrap();

                        let mut new_data = Vec::<Vertex>::new();
                        let mut new_index = Vec::<i32>::new();

                        let px = active_controls.camera.position.x.round();
                        let py = active_controls.camera.position.y.round();
                        let pz = active_controls.camera.position.z.round();
                        let (width, height) = (626.0, 1782.0);

                        chunk.blocks.push(Block {
                            pos: Vec3::from_components(px, py, pz),
                            id: block_idx.0,
                            mode: BlockType::Full,
                        });

                        let mut index = 0;
                        for block in &chunk.blocks {
                            let value = if let Some(value) = sprites.get(block.id) {
                                value
                            } else {
                                return;
                            };

                            let x = &value.frame.x;
                            let y = &value.frame.y;
                            let xw = x + &value.frame.w;
                            let yh = y + &value.frame.h;

                            let (xx, yy, xw, yh) = (x / width, y / height, xw / width, yh / height);

                            println!("Real   {} {} {} {}", x, y, xw, yh);
                            println!("Normal {} {} {} {}", xx, yy, xw, yh);

                            let pbs = 0.5;
                            let nbs = -0.5;


                            // Top
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + pbs, block.pos.z + nbs, xx, yy));
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + pbs, block.pos.z + pbs, xx, yh));
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + pbs, block.pos.z + pbs, xw, yh));
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + pbs, block.pos.z + nbs, xw, yy));

                            // Top
                            new_index.push(((index * 24) + 0) as i32);
                            new_index.push(((index * 24) + 1) as i32);
                            new_index.push(((index * 24) + 2) as i32);
                            new_index.push(((index * 24) + 0) as i32);
                            new_index.push(((index * 24) + 2) as i32);
                            new_index.push(((index * 24) + 3) as i32);


                            // Left
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + pbs, block.pos.z + pbs, xx, yy));
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + nbs, block.pos.z + pbs, xw, yy));
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + nbs, block.pos.z + nbs, xw, yh));
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + pbs, block.pos.z + nbs, xx, yh));

                            // Left
                            new_index.push(((index * 24) + 5) as i32);
                            new_index.push(((index * 24) + 4) as i32);
                            new_index.push(((index * 24) + 6) as i32);
                            new_index.push(((index * 24) + 6) as i32);
                            new_index.push(((index * 24) + 4) as i32);
                            new_index.push(((index * 24) + 7) as i32);


                            // Right
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + pbs, block.pos.z + pbs, xw, yh));
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + nbs, block.pos.z + pbs, xx, yh));
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + nbs, block.pos.z + nbs, xx, yy));
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + pbs, block.pos.z + nbs, xw, yy));

                            // Right
                            new_index.push(((index * 24) + 8) as i32);
                            new_index.push(((index * 24) + 9) as i32);
                            new_index.push(((index * 24) + 10) as i32);
                            new_index.push(((index * 24) + 8) as i32);
                            new_index.push(((index * 24) + 10) as i32);
                            new_index.push(((index * 24) + 11) as i32);



                            // Front
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + pbs, block.pos.z + pbs, xw, yh));
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + nbs, block.pos.z + pbs, xw, yy));
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + nbs, block.pos.z + pbs, xx, yy));
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + pbs, block.pos.z + pbs, xx, yh));

                            // Front
                            new_index.push(((index * 24) + 13) as i32);
                            new_index.push(((index * 24) + 12) as i32);
                            new_index.push(((index * 24) + 14) as i32);
                            new_index.push(((index * 24) + 15) as i32);
                            new_index.push(((index * 24) + 14) as i32);
                            new_index.push(((index * 24) + 12) as i32);



                            // Back
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + pbs, block.pos.z + nbs, xx, yy));
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + nbs, block.pos.z + nbs, xx, yh));
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + nbs, block.pos.z + nbs, xw, yh));
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + pbs, block.pos.z + nbs, xw, yy));

                            new_index.push(((index * 24) + 16) as i32);
                            new_index.push(((index * 24) + 17) as i32);
                            new_index.push(((index * 24) + 18) as i32);
                            new_index.push(((index * 24) + 16) as i32);
                            new_index.push(((index * 24) + 18) as i32);
                            new_index.push(((index * 24) + 19) as i32);


                            // Bottom
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + nbs, block.pos.z + nbs, xw, yh));
                            new_data.push(Vertex::new(block.pos.x + nbs, block.pos.y + nbs, block.pos.z + pbs, xw, yy));
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + nbs, block.pos.z + pbs, xx, yy));
                            new_data.push(Vertex::new(block.pos.x + pbs, block.pos.y + nbs, block.pos.z + nbs, xx, yh));

                            new_index.push(((index * 24) + 21) as i32);
                            new_index.push(((index * 24) + 20) as i32);
                            new_index.push(((index * 24) + 22) as i32);
                            new_index.push(((index * 24) + 22) as i32);
                            new_index.push(((index * 24) + 20) as i32);
                            new_index.push(((index * 24) + 23) as i32);

                            index += 1;
                        }
                        context.set_vertex_data(render, &new_data);
                        context.set_index_data(render, &new_index);
                    }
                }
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}

struct BlockIndex(usize);

fn emit_events(resources: &mut ResourceRegistry, value: &RunSystemPhase) {
    match value {
        RunSystemPhase::Event(event) => match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input,
                    is_synthetic: false,
                    ..
                } => {
                    if input.state == ElementState::Pressed {
                        if let Some(key) = input.virtual_keycode {
                            let block_idx = resources.get_mut::<BlockIndex>().unwrap();
                            let sprites = resources.get_mut::<Vec<Sprite>>().unwrap();
                            let dbg_info = resources.get_mut::<DebugInfo>().unwrap();
                            let event_channel = resources.get_mut::<EventChannel>().unwrap();

                            let original = block_idx.0;

                            if key == VirtualKeyCode::Add {
                                block_idx.0 += 1;
                            }

                            if key == VirtualKeyCode::Subtract {
                                let block_idx = resources.get_mut::<BlockIndex>().unwrap();
                                if block_idx.0 < 1 {
                                    block_idx.0 -= 1;
                                }
                            }

                            if let Some(sprite) = &sprites.get(block_idx.0) {
                                dbg_info.set(DebugKey::CurrentBlock, sprite.filename.clone());
                                event_channel.issue(FPSUpdateEvent);
                            } else {
                                block_idx.0 = original;
                            }
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
    let context = ecs.resources.get_mut::<RendererDevice>().unwrap();

    let mut file = File::open("textures/out.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let sprites: Vec<Sprite> = serde_json::from_str(&data).expect("JSON was not well-formatted");

    let (width, height) = (626.0, 1782.0);
    let value = if let Some(value) = sprites.get(2) {
        value
    } else {
        return;
    };
    let x = &value.frame.x;
    let y = &value.frame.y;
    let xw = x + &value.frame.w;
    let yh = y + &value.frame.h;
    let (x, y, xw, yh) = (x / width, y / height, xw / width, yh / height);

    let text = context.register_texture(Texture::new_initialized(
        TextureWrapping::ClampToEdge,
        TextureFiltering::Pixelated,
        TextureStorage::from_image(&Path::new("textures/out.png")),
    ));

    ecs.add_entity(
        Entity::new()
            .with(IsChunkRenderer)
            .with(context.new_mesh(
                &Path::new("shaders/single-texture.glsl"),
                vec![
                    Vertex::new(0.5, 2.0 + 0.5, 0.0, xw, yh),
                    Vertex::new(-0.5, 2.0 + -0.5, 0.0, x, y),
                    Vertex::new(0.5, 2.0 + -0.5, 0.0, xw, y),
                    Vertex::new(-0.5, 2.0 + 0.5, 0.0, x, yh),
                ],
                Some(vec![0, 1, 2, 0, 3, 1]),
                vec![text],
                None,
            )),
    );

    ecs.resources.set(BlockIndex(0));
    ecs.resources.set::<Vec<Sprite>>(sprites);
    ecs.resources.set(Chunk { blocks: Vec::new() });
    ecs.add_before_service(Service::at_event(emit_events));

    ecs.add_system(System::at_event(
        EntityShapeBuilder::new()
            .with::<IsChunkRenderer>()
            .with::<RenderComponent>()
            .build(),
        render_ui,
    ));
}
