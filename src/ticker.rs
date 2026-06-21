#[cfg(feature = "std")]
#[cfg_attr(
    feature = "mirl_derive",
    mirl_derive::derive_all(
        serde = false,
        compactly = false,
        zerocopy = false,
        bitcode = false
    )
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A ticker, regulate the timing of an application
pub struct Ticker {
    /// 1/fps
    pub target_delta_time: std::time::Duration,
    /// The last time this struct got ticked
    pub last_frame: std::time::Instant,
    /// When the next frame should start
    pub next_frame: std::time::Instant,
    /// The current delta time -> Time between frame starts/ends
    pub delta_time: std::time::Duration,
}

#[cfg(feature = "std")]
impl Ticker {
    #[must_use]
    /// Create a new ticker, holds invalid data until ticked once
    ///
    /// # Errors
    /// When fps it too high/negative to fit into [`std::time::Duration`]
    pub fn new(fps: f64) -> Option<Self> {
        Some(Self {
            target_delta_time: std::time::Duration::try_from_secs_f64(
                1.0 / fps,
            )
            .ok()?,
            last_frame: std::time::Instant::now(),
            next_frame: std::time::Instant::now(),
            delta_time: std::time::Duration::new(0, 0),
        })
    }
    /// Tick the Ticker
    ///
    /// If the frame took too long, the next frame will be skipped
    /// If there is still time left, it will sleep until the desired frame time
    pub fn tick(&mut self) {
        let now = std::time::Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;
        if now > self.next_frame {
            self.next_frame = now + self.target_delta_time;
        } else {
            std::thread::sleep(self.next_frame - now);
            self.next_frame += self.target_delta_time;
        }
    }
    #[must_use]
    /// Get the delta time, use [`as_secs_f32`](std::time::Duration::as_secs_f32) or [`as_secs_64`](std::time::Duration::as_secs_f64) on that duration
    pub const fn get_delta_time(&self) -> std::time::Duration {
        self.delta_time
    }
    /// Get the delta time and the fps of the last tick
    #[must_use]
    pub const fn get_delta_time_and_fps<
        T: [const] mirl_extensions::FromPatch<u32>
            + [const] mirl_extensions::FromPatch<u64>
            + [const] core::ops::Add<Output = T>
            + [const] core::ops::Div<Output = T>
            + [const] Clone,
    >(
        &self,
    ) -> (T, T) {
        let seconds = self.delta_time.as_secs();
        let nanos = self.delta_time.subsec_nanos();
        let delta = (T::from_value(seconds))
            + (T::from_value(nanos)) / T::from_value(1_000_000_000_u32);

        (delta.clone(), T::from_value(1u32) / delta)
    }
    /// Get the delta time and the fps of the last tick
    #[must_use]
    pub const fn get_delta_time_and_fps_f32(&self) -> (f32, f32) {
        let delta = self.delta_time.as_secs_f32();
        (delta, 1.0 / delta)
    }
    /// Get the delta time and the fps of the last tick
    #[must_use]
    pub const fn get_delta_time_and_fps_f64(&self) -> (f64, f64) {
        let delta = self.delta_time.as_secs_f64();
        (delta, 1.0 / delta)
    }
}
