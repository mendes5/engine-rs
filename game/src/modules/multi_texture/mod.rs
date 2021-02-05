mod vertex;
use ecs::{Entity, ECS};
use renderer::{
    RendererDevice, Texture, TextureFiltering, TextureStorage, TextureWrapping,
};

use std::path::Path;
use vertex::Vertex;

pub fn load(ecs: &mut ECS) {
    let context = ecs.resources.get_mut::<RendererDevice>().unwrap();

    let idx1 = context.register_texture(Texture::new_initialized(
        TextureWrapping::ClampToEdge,
        TextureFiltering::Pixelated,
        TextureStorage::from_image(&Path::new("textures/normal.jpg")),
    ));
    let idx2 = context.register_texture(Texture::new_initialized(
        TextureWrapping::ClampToEdge,
        TextureFiltering::Pixelated,
        TextureStorage::from_image(&Path::new("textures/box.png")),
    ));
    let idx3 = context.register_texture(Texture::new_initialized(
        TextureWrapping::ClampToEdge,
        TextureFiltering::Pixelated,
        TextureStorage::from_image(&Path::new("textures/wood.jpg")),
    ));
    let idx4 = context.register_texture(Texture::new_initialized(
        TextureWrapping::ClampToEdge,
        TextureFiltering::Pixelated,
        TextureStorage::from_image(&Path::new("textures/metal.jpg")),
    ));

    ecs.add_entity(
        Entity::new()
            .with(context.new_mesh(
                &Path::new("shaders/multi-texture.glsl"),
                vec![
                    Vertex::new(0.5, 2.0 + 0.5, 0.0, 1.0, 1.0),
                    Vertex::new(-0.5, 2.0 + -0.5, 0.0, 0.0, 0.0),
                    Vertex::new(0.5, 2.0 + -0.5, 0.0, 1.0, 0.0),
                    Vertex::new(-0.5, 2.0 + 0.5, 0.0, 0.0, 1.0),
                ],
                Some(vec![0, 1, 2, 0, 3, 1]),
                vec![
                    idx1, idx2
                ],
                None,
            )),
    );

    ecs.add_entity(
        Entity::new()
            .with(context.new_mesh(
                &Path::new("shaders/multi-texture.glsl"),
                vec![
                    Vertex::new(0.5, 3.0 + 0.5, 0.0, 1.0, 1.0),
                    Vertex::new(-0.5, 3.0 + -0.5, 0.0, 0.0, 0.0),
                    Vertex::new(0.5, 3.0 + -0.5, 0.0, 1.0, 0.0),
                    Vertex::new(-0.5, 3.0 + 0.5, 0.0, 0.0, 1.0),
                ],
                Some(vec![0, 1, 2, 0, 3, 1]),
                vec![
                    idx3,
                    idx4
                ],
                None,
            )),
    );
}
