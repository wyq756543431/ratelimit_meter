use {Decider, ImpliedDeciderImpl, MultiDecider, MultiDeciderImpl, NegativeMultiDecision};

use std::time::Instant;

impl Decider for Allower {}
impl MultiDecider for Allower {}

#[derive(Default, Copy, Clone)]
/// The most naive implementation of a rate-limiter ever: Always
/// allows every cell through.
/// # Example
/// ```
/// use ratelimit_meter::{Decider};
/// use ratelimit_meter::example_algorithms::Allower;
/// let mut allower = Allower::new();
/// assert!(allower.check().is_ok());
/// ```
pub struct Allower {}

impl Allower {
    pub fn new() -> Allower {
        Allower::default()
    }
}

impl MultiDeciderImpl for Allower {
    /// Allows all cells through unconditionally.
    fn test_n_and_update(&mut self, _n: u32, _t0: Instant) -> Result<(), NegativeMultiDecision> {
        Ok(())
    }
}

impl ImpliedDeciderImpl for Allower {}
