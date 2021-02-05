use std::cmp;
use std::f64;
use std::time;

#[derive(Debug, Clone)]
struct LogBuffer<T>
where
    T: Clone,
{
    head: usize,
    size: usize,
    samples: usize,
    contents: Vec<T>,
}

impl<T> LogBuffer<T>
where
    T: Clone + Copy,
{
    fn new(size: usize, init_val: T) -> LogBuffer<T> {
        LogBuffer {
            head: 0,
            size,
            contents: vec![init_val; size],
            samples: 1,
        }
    }

    fn push(&mut self, item: T) {
        self.head = (self.head + 1) % self.contents.len();
        self.contents[self.head] = item;
        self.size = cmp::min(self.size + 1, self.contents.len());
        self.samples += 1;
    }

    fn contents(&self) -> &[T] {
        if self.samples > self.size {
            &self.contents
        } else {
            &self.contents[..self.samples]
        }
    }

    fn latest(&self) -> T {
        self.contents[self.head]
    }
}

#[derive(Debug)]
pub struct TimeContext {
    init_instant: time::Instant,
    last_instant: time::Instant,
    frame_durations: LogBuffer<time::Duration>,
    residual_update_dt: time::Duration,
    frame_count: usize,
}

const TIME_LOG_FRAMES: usize = 200;

impl TimeContext {
    pub fn new() -> TimeContext {
        let initial_dt = time::Duration::from_millis(16);
        TimeContext {
            init_instant: time::Instant::now(),
            last_instant: time::Instant::now(),
            frame_durations: LogBuffer::new(TIME_LOG_FRAMES, initial_dt),
            residual_update_dt: time::Duration::from_secs(0),
            frame_count: 0,
        }
    }

    pub fn tick(&mut self) {
        let now = time::Instant::now();
        let time_since_last = now - self.last_instant;
        self.frame_durations.push(time_since_last);
        self.last_instant = now;
        self.frame_count += 1;

        self.residual_update_dt += time_since_last;
    }

    pub fn delta(&self) -> time::Duration {
        self.frame_durations.latest()
    }

    pub fn last_delta(&self) -> f64 {
        duration_to_f64(self.delta())
    }

    pub fn get_fps(&self) -> f64 {
        fps(self)
    }
}

pub fn average_delta(tc: &TimeContext) -> time::Duration {
    let sum: time::Duration = tc.frame_durations.contents().iter().sum();

    if tc.frame_durations.samples > tc.frame_durations.size {
        sum / (tc.frame_durations.size as u32)
    } else {
        sum / (tc.frame_durations.samples as u32)
    }
}

pub fn duration_to_f64(d: time::Duration) -> f64 {
    let seconds = d.as_secs() as f64;
    let nanos = f64::from(d.subsec_nanos());
    seconds + (nanos * 1e-9)
}

pub fn fps(tc: &TimeContext) -> f64 {
    let duration_per_frame = average_delta(tc);
    let seconds_per_frame = duration_to_f64(duration_per_frame);
    1.0 / seconds_per_frame
}
