use ecs::ECS;

pub mod active_controls;
pub mod cube;
pub mod cursor_grab;
pub mod debug_info;
pub mod debug_ui;
pub mod grid;
pub mod input;
pub mod on_resize;
pub mod render_all;
pub mod textured;
pub mod time;
pub mod ui;
pub mod physics;
pub mod multi_texture;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MODULE_LOADER: Vec<fn(&mut ECS)> = vec![
        debug_info::load,
        on_resize::load,
        debug_ui::load,
        render_all::load,
        ui::load,
        cube::load,
        grid::load,
        time::load,
        input::load,
        active_controls::load,
        cursor_grab::load,
        textured::load,
        physics::load,
        multi_texture::load,
    ];
}
