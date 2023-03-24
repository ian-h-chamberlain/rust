use crate::env;
use crate::fmt;
use crate::sync::atomic::{self, Ordering};
use crate::sys::thread as imp;

pub fn min_stack() -> usize {
    static MIN: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
    match MIN.load(Ordering::Relaxed) {
        0 => {}
        n => return n - 1,
    }
    let amt = env::var("RUST_MIN_STACK").ok().and_then(|s| s.parse().ok());
    let amt = amt.unwrap_or(imp::DEFAULT_MIN_STACK_SIZE);

    // 0 is our sentinel value, so ensure that we'll never see 0 after
    // initialization has run
    MIN.store(amt + 1, Ordering::Relaxed);
    amt
}

/// Helper struct for storing OS-specific thread spawning options
/// In the future, this could be used to pass along higher-level `Priority`
/// or `Affinity` objects (as part of [`Builder`]'s public API) which would be
/// used later in [`imp::Thread::new`].
// TODO: #84187 should this be in sys::common instead?
#[derive(Default)]
pub struct SpawnOptions {
    pub native: Option<imp::SpawnOptions>,
}

impl fmt::Debug for SpawnOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpawnOptions").finish_non_exhaustive()
    }
}
