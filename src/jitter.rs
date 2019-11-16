use crate::lib::*;

/// An interval specification for deviating from the nominal wait time.
///
/// Jitter can be added to wait time `Duration`s to ensure that multiple tasks waiting on the same
/// rate limit don't wake up at the same time and attempt to measure at the same time.
///
/// Methods on rate limiters that work asynchronously like
/// [`DirectRateLimiter.until_ready_with_jitter`](struct.DirectRateLimiter.html#method.until_ready_with_jitter)
/// exist to automatically apply jitter to wait periods, thereby reducing the chance of a
/// thundering herd problem.  
///
/// # Examples
///
/// Jitter can be added manually to a `Duration`:
///
/// ```rust
/// # use governor::Jitter;
/// # use std::time::Duration;
/// let reference = Duration::from_secs(24);
/// let jitter = Jitter::new(Duration::from_secs(1), Duration::from_secs(1));
/// let result = jitter + reference;
/// assert!(result >= reference + Duration::from_secs(1));
/// assert!(result < reference + Duration::from_secs(2))
/// ```
///
/// In a `std` build (the default), Jitter can also be added to an `Instant`:
///
/// ```rust
/// # use governor::Jitter;
/// # use std::time::{Duration, Instant};
/// # #[cfg(feature = "std")]
/// # fn main() {
/// let reference = Instant::now();
/// let jitter = Jitter::new(Duration::from_secs(1), Duration::from_secs(1));
/// let result = jitter + reference;
/// assert!(result >= reference + Duration::from_secs(1));
/// assert!(result < reference + Duration::from_secs(2))
/// # }
/// # #[cfg(not(feature = "std"))] fn main() {}
/// ```
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Jitter {
    min: Duration,
    interval: Duration,
}

impl Jitter {
    #[cfg(feature = "std")]
    /// The "empty" jitter interval - no jitter at all.
    pub(crate) const NONE: Jitter = Jitter {
        min: Duration::from_secs(0),
        interval: Duration::from_secs(0),
    };

    /// Constructs a new Jitter interval, waiting at most a duration of `max`.
    pub fn up_to(max: Duration) -> Jitter {
        Jitter {
            min: Duration::new(0, 0),
            interval: max,
        }
    }

    /// Constructs a new Jitter interval, waiting at least `min` and at most `min+interval`.
    pub const fn new(min: Duration, interval: Duration) -> Jitter {
        Jitter { min, interval }
    }

    /// Returns a random amount of jitter within the configured interval.
    pub(crate) fn get(&self) -> Duration {
        let range = rand::random::<f32>();
        self.min + self.interval.mul_f32(range)
    }
}

impl Add<Duration> for Jitter {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        self.get() + rhs
    }
}

#[cfg(feature = "std")]
impl Add<Instant> for Jitter {
    type Output = Instant;

    fn add(self, rhs: Instant) -> Instant {
        rhs + self.get()
    }
}
