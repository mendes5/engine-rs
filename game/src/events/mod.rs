// https://github.com/alacritty/alacritty
// 539855cdb92c44dc0f755e6ea6debcf13a77f049
// alacritty/alacritty/src/scheduler.rs

pub use glutin::event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent};
use std::any::TypeId;
use std::time::{Duration, Instant};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::desktop::EventLoopExtDesktop,
};

pub struct Timer {
    pub deadline: Instant,
    pub interval: Duration,
    pub event: TypeId,
    pub repeat: bool,
}

pub struct EventSystem {
    pub event_loop: EventLoop<()>,
    pub running: bool,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            running: true,
        }
    }

    pub fn update_events<F>(&mut self, mut handler: F)
    where
        F: FnMut(Event<'static, ()>),
    {
        let running_address = &mut self.running;

        self.event_loop.run_return(|event, _, control_flow| {
            let event: Event<'static, ()> = unsafe { std::mem::transmute(event) };

            match event.clone() {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *running_address = false,
                    _ => (),
                },
                Event::LoopDestroyed => return,
                Event::MainEventsCleared => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    *control_flow = ControlFlow::Poll;
                }
            }

            handler(event);
        });
    }
}

pub struct EventChannel {
    timers: Vec<Timer>,
}

impl EventChannel {
    pub fn new() -> Self {
        Self { timers: Vec::new() }
    }

    pub fn schedule<T: 'static>(&mut self, _event: T, interval: Duration, repeat: bool) {
        self.timers.push(Timer {
            deadline: Instant::now() + interval,
            interval,
            event: TypeId::of::<T>(),
            repeat,
        });
    }

    pub fn issue<T: 'static>(&mut self, _event: T) {
        self.timers.push(Timer {
            deadline: Instant::now(),
            interval: Duration::from_nanos(0),
            event: TypeId::of::<T>(),
            repeat: false,
        });
    }

    pub fn update_timers<F>(&mut self, mut handler: F)
    where
        F: FnMut(TypeId),
    {
        let now = Instant::now();

        let mut invalid = Vec::new();
        let mut index = 0;

        for timer in &mut self.timers {
            if timer.deadline < now {
                handler(timer.event);

                if timer.repeat {
                    timer.deadline += timer.interval;
                } else {
                    invalid.push(index);
                }
            }
            index += 0;
        }

        for index in invalid {
            self.timers.remove(index);
        }
    }
}
