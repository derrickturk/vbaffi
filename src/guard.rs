use std::mem::ManuallyDrop;

pub struct Guard<F: FnOnce()> {
    f: ManuallyDrop<F>,
}

impl<F: FnOnce()> Guard<F> {
    #[inline]
    pub fn new(f: F) -> Self {
        Self { f: ManuallyDrop::new(f) }
    }
}

impl<F: FnOnce()> Drop for Guard<F> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ManuallyDrop::take(&mut self.f)(); }
    }
}
