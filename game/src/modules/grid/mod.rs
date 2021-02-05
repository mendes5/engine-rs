mod vertex;

use ecs::{Entity, ECS};
use renderer::{RendererDevice, MeshFlags};
use std::path::Path;
use vertex::Vertex;

const GRID_SIZE: i32 = 10;
const GRID_STEP: f32 = 4.0;
const GRID_HEIGHT: f32 = -20.0;

pub fn load(ecs: &mut ECS) {
    let context = ecs.resources.get_mut::<RendererDevice>().unwrap();

    let mut vertex_data: Vec<Vertex> = Vec::new();

    let h_size = (GRID_SIZE as f32 * GRID_STEP) / 2.0;
    let mut cursor = -h_size;

    for _ in 0..(GRID_SIZE + 1) {
        vertex_data.push(Vertex::new(cursor, GRID_HEIGHT, -h_size, 1.0, 0.0, 0.0));
        vertex_data.push(Vertex::new(cursor, GRID_HEIGHT, h_size, 0.0, 1.0, 0.0));
        vertex_data.push(Vertex::new(-h_size, GRID_HEIGHT, cursor, 0.0, 0.0, 1.0));
        vertex_data.push(Vertex::new(h_size, GRID_HEIGHT, cursor, 1.0, 1.0, 0.0));

        vertex_data.push(Vertex::new(cursor, -GRID_HEIGHT, -h_size, 1.0, 0.0, 0.0));
        vertex_data.push(Vertex::new(cursor, -GRID_HEIGHT, h_size, 0.0, 1.0, 0.0));
        vertex_data.push(Vertex::new(-h_size, -GRID_HEIGHT, cursor, 0.0, 0.0, 1.0));
        vertex_data.push(Vertex::new(h_size, -GRID_HEIGHT, cursor, 1.0, 1.0, 0.0));

        cursor += GRID_STEP;
    }

    ecs.add_entity(Entity::new().with(context.new_mesh(
        &Path::new("shaders/color.glsl"),
        vertex_data,
        None,
        vec![],
        MeshFlags::new().lines_mode().opt(),
    )));
}
