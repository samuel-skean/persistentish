use std::{cell::Cell, rc::Rc};


// Creating one of these that does have cycles may be unsafe, I'm not sure. See
// https://ryhl.io/blog/temporary-shared-mutation/, specifically the section on
// why it isn't enough for the contents of Cell to be Clone. Rc's clone does
// involve modifying its contents, but not the thing whose reference we leak in
// this case.
//
// NOTES: Even when I thought I needed this, I don't know why it contained Option when
// it avoids the need for `take`.
//
// Regardless, I should probably have used UnsafeCell for clarity simply because
// I wasn't taking any advantage of Cell's safety, just presenting a slightly
// Cell-like interface.
pub(crate) struct Link<T>(pub(crate) Cell<Option<Rc<T>>>);

impl<T> Link<T> {
    // At some point I thought this was only safe if it returned Cell<T>. I
    // don't think so now, but I'm not quite sure why.
    pub(crate) fn as_ref(&self) -> Option<&T> {
        let ptr = self.0.as_ptr();
        unsafe {
            // SAFETY: This materializes a reference, but it's the only
            // reference to the interior.
            (*ptr).as_deref()
        }
    }
}