#![unstable(feature = "os_thread_scheduling", issue = "none")]

use crate::mem;

pub trait BuilderExt {
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
