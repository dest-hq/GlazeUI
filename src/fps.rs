use std::time::Instant;

pub struct FpsCounter {
    last_frame: Instant,
    frame_count: u32,
    fps: f32,
    fps_update_timer: f32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            fps_update_timer: 0.0,
        }
    }

    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        self.frame_count += 1;
        self.fps_update_timer += delta;

        if self.fps_update_timer >= 0.5 {
            self.fps = self.frame_count as f32 / self.fps_update_timer;
            self.frame_count = 0;
            self.fps_update_timer = 0.0;
        }

        self.fps
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}
