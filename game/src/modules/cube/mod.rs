mod vertex;

use ecs::{Entity, ECS};
use renderer::{RendererDevice, MeshFlags};
use std::path::Path;
use vertex::Vertex;

pub fn load(ecs: &mut ECS) {
    let context = ecs.resources.get_mut::<RendererDevice>().unwrap();

    ecs.add_entity(Entity::new().with(context.new_mesh(
        &Path::new("shaders/color.glsl"),
        vec![
            Vertex::new(-0.5, 0.5, 0.5, 0.44, 0.77, 0.63),
            Vertex::new(0.5, 0.5, 0.5, 0.82, 0.71, 0.18),
            Vertex::new(-0.5, 0.5, 0.5, 0.35, 0.08, 0.5),
            Vertex::new(-0.5, -0.5, 0.5, 0.32, 0.59, 0.89),
            Vertex::new(0.5, -0.5, 0.5, 0.13, 0.72, 0.78),
            Vertex::new(0.5, 0.5, 0.5, 0.81, 0.96, 0.38),
            Vertex::new(0.5, -0.5, 0.5, 0.84, 0.34, 0.72),
            Vertex::new(-0.5, -0.5, 0.5, 0.4, 0.88, 0.78),
            Vertex::new(0.5, -0.5, -0.5, 0.65, 0.87, 0.03),
            Vertex::new(0.5, 0.5, -0.5, 0.6, 0.82, 0.65),
            Vertex::new(0.5, -0.5, -0.5, 0.27, 0.51, 0.81),
            Vertex::new(-0.5, -0.5, -0.5, 0.09, 0.36, 0.19),
            Vertex::new(-0.5, 0.5, -0.5, 0.57, 0.93, 0.65),
            Vertex::new(0.5, 0.5, -0.5, 0.24, 0.22, 1.0),
            Vertex::new(-0.5, 0.5, -0.5, 0.16, 0.79, 0.63),
            Vertex::new(-0.5, -0.5, -0.5, 0.09, 0.62, 0.19),
            Vertex::new(0.5, 0.5, -0.5, 0.41, 0.02, 0.74),
            Vertex::new(0.5, 0.5, 0.5, 0.51, 0.12, 0.37),
            Vertex::new(-0.5, -0.5, 0.5, 0.25, 0.2, 0.44),
            Vertex::new(-0.5, -0.5, -0.5, 0.61, 0.55, 0.93),
            Vertex::new(-0.5, 0.5, -0.5, 0.68, 0.8, 0.81),
            Vertex::new(-0.5, 0.5, 0.5, 0.53, 0.27, 0.72),
            Vertex::new(0.5, -0.5, -0.5, 0.12, 0.03, 0.15),
            Vertex::new(0.5, -0.5, 0.5, 0.16, 0.36, 0.28),
        ],
        None,
        vec![],
        MeshFlags::new().lines_mode().opt(),
    )));
}
