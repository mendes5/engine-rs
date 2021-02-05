use crate::events::EventSystem;
use glutin::dpi::{PhysicalPosition, PhysicalSize};
use std::fmt;
use std::time::Duration;

use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use winit::window::{Window, WindowBuilder};

#[allow(dead_code)]
pub enum TargetFPS {
    RefreshRate,
    Absolute(u64),
    Max,
}

pub struct WindowContext {
    #[allow(dead_code)]
    pub window_context: ContextWrapper<PossiblyCurrent, Window>,
    view_port: (u32, u32),
    target_fps: TargetFPS,
    cursor_locked: bool,
}

impl WindowContext {
    pub fn from_event_loop(event_loop: &EventSystem) -> Self {
        let wb = WindowBuilder::new()
            .with_title("Game Window")
            .with_inner_size(PhysicalSize::new(500, 500));

        let window_context = ContextBuilder::new()
            .build_windowed(wb, &event_loop.event_loop)
            .unwrap();

        let window_context = unsafe { window_context.make_current().unwrap() };

        Self {
            window_context,
            view_port: (500, 500),
            target_fps: TargetFPS::Max,
            cursor_locked: false,
        }
    }

    pub fn half_view_port(&self) -> PhysicalPosition<f64> {
        PhysicalPosition::new(self.view_port.0 as f64 / 2.0, self.view_port.1 as f64 / 2.0)
    }

    #[allow(dead_code)]
    pub fn set_target_fps(&mut self, target_fps: TargetFPS) {
        self.target_fps = target_fps;
    }

    pub fn loop_end(&mut self) {
        self.swap_buffers();

        if self.cursor_locked {
            self.set_cursor_position(self.half_view_port());
        }

        match self.target_fps {
            TargetFPS::RefreshRate => {
                std::thread::yield_now();
                std::thread::sleep(Duration::from_millis(1000 / 75));
            }
            TargetFPS::Absolute(value) => {
                std::thread::yield_now();
                std::thread::sleep(Duration::from_millis(1000 / value));
            }
            TargetFPS::Max => {}
        }
    }

    pub fn set_cursor_grab(&mut self, grab: bool) {
        self.cursor_locked = grab;
        self.set_cursor_visible(!grab);
    }

    pub fn set_cursor_position(&self, size: PhysicalPosition<f64>) {
        self.window_context
            .window()
            .set_cursor_position(size)
            .expect("Cannot set cursor position");
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        self.window_context.window().set_cursor_visible(visible);
    }

    pub fn swap_buffers(&self) {
        self.window_context.swap_buffers().unwrap();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.window_context.resize(size);
        self.view_port.0 = size.width;
        self.view_port.1 = size.height;
    }
}

impl fmt::Debug for WindowContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<WindowContext: {:p}>", self)
    }
}
