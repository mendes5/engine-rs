mod vertex;

use crate::modules::{debug_ui::IsDebugUITag, on_resize::ViewportSizedUITag};
use ecs::{Entity, ECS};
use renderer::{
    MeshFlags, RendererDevice, Texture, TextureFiltering, TextureStorage, TextureWrapping,
};
use std::path::Path;
use vertex::Vertex;

pub fn load(ecs: &mut ECS) {
    let context = ecs.resources.get_mut::<RendererDevice>().unwrap();

    let text_id = context.register_texture(Texture::new_initialized(
        TextureWrapping::ClampToEdge,
        TextureFiltering::Pixelated,
        TextureStorage::from_canvas(500, 500),
    ));

    ecs.add_entity(
        Entity::new()
            .with(IsDebugUITag)
            .with(ViewportSizedUITag)
            .with(context.new_mesh(
                &Path::new("shaders/ui.glsl"),
                vec![
                    Vertex::new(1.0, 1.0, 1.0, 0.0),
                    Vertex::new(-1.0, -1.0, 0.0, 1.0),
                    Vertex::new(1.0, -1.0, 1.0, 1.0),
                    Vertex::new(-1.0, 1.0, 0.0, 0.0),
                ],
                Some(vec![0, 1, 2, 0, 3, 1]),
                vec![text_id],
                MeshFlags::new().no_depth().opt(),
            )),
    );
}
