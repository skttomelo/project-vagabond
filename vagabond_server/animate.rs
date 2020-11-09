use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Animator {
    current_frame: usize,
    total_frames: usize,
    current_time: Option<Instant>,
    frame_duration: Duration,
    pub paused: bool
}

impl Animator {
    pub fn new(total_frames: usize, frame_duration: Duration) -> Animator {
        Animator {
            current_frame: 0,
            total_frames: total_frames,
            current_time: None,
            frame_duration: frame_duration,
            paused: true,
        }
    }

    pub fn current_frame(&self) -> u16 {
        self.current_frame.clone() as u16
    }

    pub fn update(&mut self) {
        if self.paused {
            return;
        }

        if self.current_time == None {
            self.current_time = Some(Instant::now());
        }

        let instant = Instant::now();
        let duration_since = instant.duration_since(self.current_time.unwrap());

        // is it time to change frames? if not return out of fn
        if duration_since < self.frame_duration {
            return;
        }

        self.current_time = Some(Instant::now()); // update the current time
        if self.current_frame != self.total_frames {
            self.current_frame += 1;
        }
    }

    // ending animation and resetting
    pub fn end(&mut self) {
        self.current_frame = 0;
        self.current_time = None;
    }
}