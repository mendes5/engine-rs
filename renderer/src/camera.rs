use glutin::dpi::PhysicalSize;

use math::{clamp_mut, Mat4, Vec2, Vec3, SAFE_HALF_PI_MAX, SAFE_HALF_PI_MIN};

pub struct Camera {
    pub f_width: f32,
    pub f_height: f32,
    mov: Vec2,
    pub position: Vec3,
    pub pointing: Vec3,
    pub right: Vec3,
    pub forward: Vec3,
    pub look_at: Vec3,
    up: Vec3,
    fov_y: f32,
    z_near: f32,
    z_far: f32,
    cached_view: Mat4,
    cached_projection: Mat4,
    pub speed: f32,
    enabled: bool,
}

impl Camera {
    pub fn new() -> Self {
        let mut state = Self {
            f_width: 500.0,
            f_height: 500.0,
            mov: Vec2::from_components(0.0, 0.0),
            position: Vec3::from_components(0.0, 0.0, 0.0),
            pointing: Vec3::from_components(1.0, 0.0, 0.0),
            right: Vec3::from_components(0.0, 0.0, 0.0),
            forward: Vec3::from_components(0.0, 0.0, 0.0),
            look_at: Vec3::from_components(0.0, 0.0, 0.0),
            up: Vec3::from_components(0.0, 1.0, 0.0),
            fov_y: std::f32::consts::FRAC_PI_2,
            z_near: 0.01,
            z_far: 1000.0,
            cached_view: Mat4::new(),
            cached_projection: Mat4::new(),
            speed: 4.0,
            enabled: false,
        };

        state.make_valid();

        state
    }

    pub fn make_valid(&mut self) {
        self.enabled = true;
        self.on_mouse_step(0.0, 0.0);
        self.enabled = false;

        self.look_at.add_vectors(&self.pointing, &self.position);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn on_resize(&mut self, physical_size: PhysicalSize<u32>) {
        let (width, height) = physical_size.into();

        self.f_width = width;
        self.f_height = height;
    }

    pub fn on_mouse_step(&mut self, x: f32, y: f32) {
        if !self.enabled {
            return;
        }
        self.mov.x += x / 1000.0;
        self.mov.y -= y / 1000.0;

        clamp_mut(&mut self.mov.y, SAFE_HALF_PI_MIN, SAFE_HALF_PI_MAX);

        self.pointing.to_sphere_coord(&self.mov);

        self.right.cross_vectors(&self.up, &self.pointing);
        self.right.normalize();

        self.forward.cross_vectors(&self.up, &self.right);
        self.forward.normalize();
    }

    pub fn get_matrix(&mut self) -> (&Mat4, &Mat4) {
        self.cached_view
            .look_at(&self.position, &self.look_at, &self.up);

        self.cached_projection.to_projection_matrix(
            self.z_near,
            self.z_far,
            self.fov_y,
            self.f_width / self.f_height,
            1.0,
        );
        (&self.cached_view, &self.cached_projection)
    }

    pub fn write_matrix(&self, cached_view: &mut Mat4, cached_projection: &mut Mat4) {
        cached_view.look_at(&self.position, &self.look_at, &self.up);

        cached_projection.to_projection_matrix(
            self.z_near,
            self.z_far,
            self.fov_y,
            self.f_width / self.f_height,
            1.0,
        );
    }
}
