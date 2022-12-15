use std::mem::ManuallyDrop;

pub struct Guard<F: FnOnce()> {
    dropfn: ManuallyDrop<F>,
}
impl<F: FnOnce()> Drop for Guard<F> {
    fn drop(&mut self) {
        let dropfn = unsafe { std::ptr::read(&*self.dropfn) };
        dropfn();
    }
}
pub fn defer<F: FnOnce()>(dropfn: F) -> Guard<F> {
    Guard {
        dropfn: ManuallyDrop::new(dropfn),
    }
}
