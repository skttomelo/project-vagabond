use std::time::{Duration, Instant};

use crate::server_data::ServerAnimator;

#[derive(Clone, Debug)]
pub struct Animator {
    current_frame: usize,
    total_frames: usize,
    current_time: Option<Instant>,
    frame_duration: Duration,
    current_repeat: i8,
    repeat: i8, // any negative number means to repeat indefinitely
}

impl Animator {
    pub fn new(total_frames: usize, frame_duration: Duration, repeat: i8) -> Animator {
        Animator {
            current_frame: 0,
            total_frames: total_frames,
            current_time: None,
            frame_duration: frame_duration,
            current_repeat: 0,
            repeat: repeat,
        }
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    pub fn current_repeat(&self) -> i8 {
        self.current_repeat
    }

    pub fn max_repeats(&self) -> i8 {
        self.repeat
    }

    pub fn update(&mut self) {
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
        if self.current_frame != self.total_frames
            && (self.current_repeat <= self.repeat || self.repeat < 0)
        {
            self.current_frame += 1;
        }

        if self.current_frame == self.total_frames {
            if self.repeat >= 0 {
                self.current_repeat += 1;
            }

            if self.current_repeat <= self.repeat || self.repeat < 0 {
                self.current_frame = 0;
            }
        }
    }

    // ending animation and resetting
    pub fn end(&mut self) {
        self.current_frame = 0;
        self.current_time = None;
        self.current_repeat = 0;
    }
}

// deserializing from ServerAnimator
impl Animator {
    pub fn update_from_server_animator(&mut self, server_animator: &ServerAnimator) {
        self.current_frame = server_animator.current_frame();
        self.current_repeat = server_animator.current_repeat();
    } 
}
