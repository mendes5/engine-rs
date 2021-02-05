mod vertex;
mod utils;

use crate::internal::FPSControls;
use crate::modules::textured::Sprite;
use crate::time::TimeContext;
use bulletrs::*;
use cgmath::{Vector3, Vector4};
use ecs::{Entity, EntityShapeBuilder, ResourceRegistry, RunSystemPhase, Service, System, ECS};
use renderer::{ShaderVariable, MeshFlags, RenderComponent, RendererDevice, Texture,TextureWrapping, TextureFiltering, TextureStorage};
use std::path::Path;
use math::{Vec3, Vec4};
use vertex::{Vertex};
use utils::build_block_mesh;

extern crate bulletrs;
extern crate cgmath;

struct PhysicsWorld {
    world: DynamicsWorld,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let configuration = CollisionConfiguration::new_default();

        let mut world = DynamicsWorld::new_discrete_world(
            CollisionDispatcher::new(&configuration),
            Broadphase::new(BroadphaseInterface::DbvtBroadphase),
            ConstraintSolver::new(),
            configuration,
        );
        world.set_gravity(Vector3::new(0.0, -10.0, 0.0));

        Self { world }
    }

    pub fn new_box(
        &mut self,
        size: Vector3<f64>,
        mass: f64,
        translation: Vector3<f64>,
        orientation: Vector4<f64>,
        restitution: f64,
    ) -> PhysicsBody {
        PhysicsBody::box_from(self, size, mass, translation, orientation, restitution)
    }

    pub fn new_plane(
        &mut self,
        size: Vector3<f64>,
        plane_const: f64,
        mass: f64,
        translation: Vector3<f64>,
        orientation: Vector4<f64>,
        restitution: f64,
    ) -> PhysicsBody {
        PhysicsBody::plane_from(
            self,
            size,
            plane_const,
            mass,
            translation,
            orientation,
            restitution,
        )
    }
}

struct PhysicsBody {
    body: RigidBodyHandle,
}

impl PhysicsBody {
    pub fn box_from(
        world: &mut PhysicsWorld,
        size: Vector3<f64>,
        mass: f64,
        translation: Vector3<f64>,
        orientation: Vector4<f64>,
        restitution: f64,
    ) -> Self {
        let shape = Shape::new_box(size);
        let mut body = world.world.add_rigid_body(RigidBody::new(
            mass,
            shape.calculate_local_inertia(mass),
            shape,
            translation,
            orientation,
        ));
        body.set_restitution(restitution);

        Self { body }
    }

    pub fn plane_from(
        world: &mut PhysicsWorld,
        size: Vector3<f64>,
        plane_const: f64,
        mass: f64,
        translation: Vector3<f64>,
        orientation: Vector4<f64>,
        restitution: f64,
    ) -> Self {
        let shape = Shape::new_plane(size, plane_const);
        let mut body = world.world.add_rigid_body(RigidBody::new(
            mass,
            shape.calculate_local_inertia(mass),
            shape,
            translation,
            orientation,
        ));
        body.set_restitution(restitution);

        Self { body }
    }
}

fn before_frame(resources: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let time_context = resources.get_mut::<TimeContext>().unwrap();
    let world = resources.get_mut::<PhysicsWorld>().unwrap();

    world
        .world
        .step_simulation(time_context.last_delta(), 0, 0.0);
}

fn render_hit_point_physics(
    entity: &mut Entity,
    resources: &mut ResourceRegistry,
    _value: &RunSystemPhase,
) {
    let active_controls = resources.get_mut::<FPSControls>().unwrap();

    let mut world_pos = Vec3::from(&active_controls.camera.pointing);
    world_pos.multiply_scalar(5.0);
    world_pos.add(&active_controls.camera.position);

    let p = entity.get_mut::<PhysicsBody>().unwrap();

    p.body.reset_position_and_orientation(
        [world_pos.x as f64, world_pos.y as f64, world_pos.z as f64],
        [0.0, 0.0, 0.0, 1.0],
    );
}

fn render_hit_point(
    entity: &mut Entity,
    resources: &mut ResourceRegistry,
    _value: &RunSystemPhase,
) {
    let active_controls = resources.get_mut::<FPSControls>().unwrap();
    let world = resources.get_mut::<PhysicsWorld>().unwrap();

    let mut world_pos = Vec3::from(&active_controls.camera.pointing);
    world_pos.multiply_scalar(100.0);
    world_pos.add(&active_controls.camera.position);

    let result = world.world.raytest(ClosestRayResultCallback::new(
        [
            active_controls.camera.position.x as f64,
            active_controls.camera.position.y as f64,
            active_controls.camera.position.z as f64,
        ],
        [world_pos.x as f64, world_pos.y as f64, world_pos.z as f64],
    ));

    let render = entity.get_mut::<RenderComponent>().unwrap();

    for i in result.intersections() {
        let u_position = render.material.get_variable::<Vec3>("u_position");
        let u_orientation = render.material.get_variable::<Vec4>("u_orientation");
        let u_size = render.material.get_variable::<Vec3>("u_size");

        u_position.set(&Vec3::from_components(
            i.point.x as f32,
            i.point.y as f32,
            i.point.z as f32,
        ));
        u_orientation.set(&Vec4::from_components(0.0, 0.0, 0.0, 0.0));
        u_size.set(&Vec3::from_scalar(0.2));
    }
}

fn render_static(entity: &mut Entity, _resources: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let render = entity.get_mut::<RenderComponent>().unwrap();
    let u_position = render.material.get_variable::<Vec3>("u_position");
    let u_orientation = render.material.get_variable::<Vec4>("u_orientation");
    let u_size = render.material.get_variable::<Vec3>("u_size");

    u_position.set(&Vec3::from_components(0.0, 0.0, 0.0));
    u_orientation.set(&Vec4::from_components(0.0, 0.0, 0.0, 1.0));
    u_size.set(&Vec3::from_scalar(1.0));
}

fn render_dynamic(entity: &mut Entity, _: &mut ResourceRegistry, _value: &RunSystemPhase) {
    let thing = entity.get_mut::<PhysicsBody>().unwrap();
    let render = entity.get_mut::<RenderComponent>().unwrap();

    // Todo:: call defautls on Vec3 and set just copy
    let u_position = render.material.get_variable::<Vec3>("u_position");
    let u_orientation = render.material.get_variable::<Vec4>("u_orientation");
    let u_size = render.material.get_variable::<Vec3>("u_size");

    let (position, orientation) = thing.body.get_world_position_and_orientation();

    u_position.set(&Vec3::from_components(
        position.x as f32,
        position.y as f32,
        position.z as f32,
    ));
    u_orientation.set(&Vec4::from_components(
        orientation.x as f32,
        orientation.y as f32,
        orientation.z as f32,
        orientation.w as f32,
    ));
    u_size.set(&Vec3::from_scalar(1.0));
}

const GRID_SIZE: i32 = 100;
const GRID_STEP: f32 = 2.0;
const GRID_HEIGHT: f32 = -2.0;

struct IsDynamicBodyRendererTag;
struct IsHitPointTag;
struct IsStaticBodyRendererTag;
struct IsPickerCursorTag;


pub fn load(ecs: &mut ECS) {
    ecs.resources.set(PhysicsWorld::new());
    let world = ecs.resources.get_mut::<PhysicsWorld>().unwrap();
    let context = ecs.resources.get_mut::<RendererDevice>().unwrap();

    let ground_shape = world.new_plane(
        Vector3::new(0.0, 1.0, 0.0),
        -2.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
        0.0,
    );
    
    let text = context.register_texture(Texture::new_initialized(
        TextureWrapping::ClampToEdge,
        TextureFiltering::Pixelated,
        TextureStorage::from_image(&Path::new("textures/out.png")),
    ));
    
    let context = ecs.resources.get_mut::<RendererDevice>().unwrap();
    let sprites = ecs.resources.get_mut::<Vec<Sprite>>().unwrap();

    for i in 0..10 {
        if let Some((new_data, new_index)) = build_block_mesh(sprites, Vec3::from_components(0.0, 0.0, 0.0), 0) {
            ecs.add_entity(
                Entity::new()
                    .with(world.new_box(
                        Vector3::new(1.0, 1.0, 1.0),
                        20.1,
                        Vector3::new(0.0, i as f64 * 2.5, 0.0),
                        Vector4::new(0.4 * i as f64, 1.0, 0.22 * (i as f64) / 2.0, 1.0),
                        1.0,
                    ))
                    .with(IsDynamicBodyRendererTag)
                    .with(context.new_mesh(
                        &Path::new("shaders/textured_body_render.glsl"),
                        new_data,
                        Some(new_index),
                        vec![text],
                        None,
                    )),
            );
        }
    }

    if let Some((new_data, new_index)) = build_block_mesh(sprites, Vec3::from_components(0.0, 0.0, 0.0), 10) {


        if let Some((new_data_2, new_index_2)) = build_block_mesh(sprites, Vec3::from_components(0.0, 0.0, 0.0), 12) {



            let body_a = world.new_box(
                Vector3::new(0.5, 0.5, 0.5),
                0.0,
                Vector3::new(0.0, 5.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
                0.0,
            );

            let body_b = world.new_box(
                Vector3::new(1.0, 1.0, 1.0),
                20.0,
                Vector3::new(0.0, 7.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
                1.0,
            );

            let hinge = HingeConstraint::new(
                &body_b.body,
                &body_a.body,
                Vector3::from([3.0, 3.0, 3.0]),
                Vector3::from([0.0, 0.0, 0.0]),
                Vector3::from([1.0, 0.0, 0.0]),
                Vector3::from([1.0, 0.0, 0.0]),
                false
            );
                    world.world.add_constraint(hinge, true);

            ecs.add_entity(
                Entity::new()
                    .with(body_a)
                    .with(IsDynamicBodyRendererTag)
                    .with(IsPickerCursorTag)
                    .with(context.new_mesh(
                        &Path::new("shaders/textured_body_render.glsl"),
                        new_data_2,
                        Some(new_index_2),
                        vec![text],
                        None,
                    )),
            );

            ecs.add_entity(
                Entity::new()
                    .with(body_b)
                    .with(IsDynamicBodyRendererTag)
                    .with(context.new_mesh(
                        &Path::new("shaders/textured_body_render.glsl"),
                        new_data,
                        Some(new_index),
                        vec![text],
                        None,
                    )),
            );
        }
    }





    let mut vertex_data: Vec<Vertex> = Vec::new();

    let h_size = (GRID_SIZE as f32 * GRID_STEP) / 2.0;
    let mut cursor = -h_size;

    for _ in 0..(GRID_SIZE + 1) {
        vertex_data.push(Vertex::new(cursor, GRID_HEIGHT, -h_size, 1.0, 0.0, 0.0));
        vertex_data.push(Vertex::new(cursor, GRID_HEIGHT, h_size, 0.0, 1.0, 0.0));
        vertex_data.push(Vertex::new(-h_size, GRID_HEIGHT, cursor, 0.0, 0.0, 1.0));
        vertex_data.push(Vertex::new(h_size, GRID_HEIGHT, cursor, 1.0, 1.0, 0.0));
        cursor += GRID_STEP;
    }

    ecs.add_entity(
        Entity::new()
            .with(ground_shape)
            .with(IsStaticBodyRendererTag)
            .with(context.new_mesh(&Path::new("shaders/body_render.glsl"), vertex_data, None, vec![], MeshFlags::new().lines_mode().opt())),
    );

    ecs.add_entity(Entity::new().with(IsHitPointTag).with(context.new_mesh(
        &Path::new("shaders/body_render.glsl"),
        vec![
            Vertex::new(-1.0, 1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, 1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, -1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, -1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, -1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, -1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, -1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, -1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, -1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, 1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, 1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, -1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, -1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, -1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, 1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(-1.0, 1.0, 1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, -1.0, -1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, -1.0, 1.0, 1.0, 0.0, 0.0),
        ],
        None,
        vec![],
        MeshFlags::new().lines_mode().opt(),
    )));

    ecs.add_system(System::at_render(
        EntityShapeBuilder::new()
            .with::<RenderComponent>()
            .with::<IsHitPointTag>()
            .build(),
        render_hit_point,
    ));

    ecs.add_system(System::at_render(
        EntityShapeBuilder::new()
            .with::<RenderComponent>()
            .with::<IsPickerCursorTag>()
            .build(),
        render_hit_point_physics,
    ));

    ecs.add_system(System::at_render(
        EntityShapeBuilder::new()
            .with::<PhysicsBody>()
            .with::<RenderComponent>()
            .with::<IsStaticBodyRendererTag>()
            .build(),
        render_static,
    ));

    ecs.add_system(System::at_render(
        EntityShapeBuilder::new()
            .with::<PhysicsBody>()
            .with::<RenderComponent>()
            .with::<IsDynamicBodyRendererTag>()
            .build(),
        render_dynamic,
    ));

    ecs.add_before_service(Service::at_render(before_frame));
}
