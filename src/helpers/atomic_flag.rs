use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Clone, Debug, Default)]
pub struct AtomicFlag(Arc<AtomicBool>);

impl AtomicFlag {
    #[must_use]
    pub fn new() -> Self {
        Self::default() // will be false-initialized, which is what we want
    }

    pub fn set(&self) {
        self.0.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn check(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    pub fn reset(&self) {
        self.0.store(false, Ordering::Release);
    }
}
