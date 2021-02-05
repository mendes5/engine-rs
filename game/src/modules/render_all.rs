use crate::window_context::WindowContext;
use ecs::{Entity, EntityShapeBuilder, ResourceRegistry, RunSystemPhase, Service, System, ECS};
use renderer::gl_vertex_format::get_attribute_format;
use renderer::{
    offset_of, renderer::gl, Float, GLBuffer, GLUniformBlockIndex, RenderComponent, RendererDevice,
    VertexFormat,
};
use std::ffi::c_void;
use std::path::Path;
use math::Mat4;

fn render_all(entity: &mut Entity, resources: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let ctx = resources.get_mut::<RendererDevice>().unwrap();

    let thing = entity.get_mut::<RenderComponent>().unwrap();

    ctx.render_component(thing);
}

fn before_frame(resources: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let device = resources.get_mut::<RendererDevice>().unwrap();
    let viewport_ubo = resources.get_mut::<ViewportUBO>().unwrap();

    device.ctx.reset_state();

    device
        .ctx
        .buffer
        .bind_buffer(&viewport_ubo.buffer)
        .set_data(&viewport_ubo.data);
}

fn after_frame(resources: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let game_context = resources.get_mut::<WindowContext>().unwrap();
    game_context.loop_end();
}

pub struct ViewportMatrices {
    pub projection: Mat4,
    pub view: Mat4,
}

pub struct ViewportUBO {
    pub block: GLUniformBlockIndex<ViewportMatrices>,
    pub buffer: GLBuffer,
    pub data: [ViewportMatrices; 1],
}

#[repr(C)]
pub struct Vertex {
    v_scalar: Float,
}

impl Vertex {
    pub fn new(x: f32) -> Self {
        Self {
            v_scalar: Float { x },
        }
    }
}

impl VertexFormat for Vertex {
    fn size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn on_vertex_layout() -> Vec<(i32, u32, u8, *const c_void)> {
        unsafe { vec![get_attribute_format::<Float>(offset_of!(Self, v_scalar))] }
    }
}

pub fn load(ecs: &mut ECS) {
    ecs.add_system(System::at_render(
        EntityShapeBuilder::new().with::<RenderComponent>().build(),
        render_all,
    ));

    let device = ecs.resources.get_mut::<RendererDevice>().unwrap();
    let program = device
        .ctx
        .program
        .create_from_file::<Vertex>(&Path::new("shaders/ubo_init.glsl"));

    let ubo = device
        .ctx
        .buffer
        .create_buffer(gl::UNIFORM_BUFFER, gl::STREAM_DRAW);

    let matrices_ubo_block_binding =
        program.get_uniform_block_index::<ViewportMatrices>("ViewportUBO");

    let binding_point: gl::GLuint = 0;
    program.uniform_block_binding(&matrices_ubo_block_binding, binding_point);

    device.ctx.buffer.bind_buffer_base(binding_point, &ubo);

    let viewport_ubo = ViewportUBO {
        block: matrices_ubo_block_binding,
        buffer: ubo,
        data: [ViewportMatrices {
            projection: Mat4::new(),
            view: Mat4::new(),
        }],
    };

    ecs.resources.set(viewport_ubo);
    ecs.add_before_service(Service::at_render(before_frame));
    ecs.add_after_service(Service::at_render(after_frame));
}
