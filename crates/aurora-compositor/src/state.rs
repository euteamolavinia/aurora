use std::time::Instant;

use anyhow::Result;
use smithay::reexports::calloop::{EventLoop, LoopHandle, LoopSignal};

pub struct AuroraState {
    pub running: bool,
    pub start_time: Instant,
    pub loop_handle: LoopHandle<'static, Self>,
    pub loop_signal: LoopSignal,
}

impl AuroraState {
    pub fn new(event_loop: &EventLoop<'static, Self>) -> Result<Self> {
        Ok(Self {
            running: true,
            start_time: Instant::now(),
            loop_handle: event_loop.handle(),
            loop_signal: event_loop.get_signal(),
        })
    }

    pub fn uptime_ms(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }
}