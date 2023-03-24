#![unstable(feature = "os_thread_scheduling", issue = "none")]

pub trait BuilderExt {
    fn priority(self, prio: libc::c_int) -> Self;
}

impl BuilderExt for crate::thread::Builder {
    fn priority(mut self, sched_priority: libc::c_int) -> Self {
        // Initialize default attr if not already set
        self.options.native = self.options.native.or_else(Default::default);

        let attr = &mut self.options.native.as_mut().unwrap().attr;
        let param = libc::sched_param { sched_priority };
        unsafe {
            assert_eq!(libc::pthread_attr_setschedparam(attr, &param), 0);
        }

        self
    }
}
