#![unstable(feature = "os_thread_scheduling", issue = "none")]

use crate::mem;

/// Linux-specific extensions for [`thread::Builder`](crate::thread::Builder).
pub trait BuilderExt {
    /// Set the CPU affinity (which cores to run on) for the thread to be spawned.
    ///
    /// See <https://man7.org/linux/man-pages/man3/CPU_SET.3.html> for more details
    /// about how to construct the `cpu_set` parameter.
    fn affinity(self, cpu_set: libc::cpu_set_t) -> Self;
}

impl BuilderExt for crate::thread::Builder {
    fn affinity(mut self, cpu_set: libc::cpu_set_t) -> Self {
        // Initialize default attr if not already set
        self.options.native = self.options.native.or_else(Default::default);

        let attr = &mut self.options.native.as_mut().unwrap().attr;
        unsafe {
            assert_eq!(
                libc::pthread_attr_setaffinity_np(
                    attr,
                    mem::size_of::<libc::cpu_set_t>(),
                    &cpu_set
                ),
                0
            );
        }

        self
    }
}
