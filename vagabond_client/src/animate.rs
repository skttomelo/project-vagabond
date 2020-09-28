use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct Animator {
    current_frame: usize,
    total_frames: usize,
    current_time: Option<Instant>,
    frame_duration: Duration,
}

impl Animator {
    pub fn new(total_frames: usize, frame_duration: Duration) -> Animator {
        Animator {
            current_frame: 0,
            total_frames: total_frames,
            current_time: None,
            frame_duration: frame_duration,
        }
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    pub fn update(&mut self) {
        if self.current_time == None {
            self.current_time = Some(Instant::now());
        }
        
        let instant = Instant::now();
        let duration_since = instant.duration_since(self.current_time.unwrap());
        
        // is it time to change frames? if not return out of fn
        if &duration_since < &self.frame_duration {
            return;
        }

        self.current_time = Some(Instant::now()); // update the current time
        self.current_frame += 1;
        if self.current_frame == self.total_frames {
            self.current_frame = 0;
        }
    }

    // ending animation and resetting
    pub fn end(&mut self) {
        self.current_frame = 0;
        self.current_time = None;
    }
}