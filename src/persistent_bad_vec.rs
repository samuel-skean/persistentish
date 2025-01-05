use std::{cell::RefCell, collections::VecDeque, marker::PhantomData, rc::Rc};

struct PersistentBadVec<'a, T> {
    current: Rc<RefCell<Vec<T>>>,
    history: Rc<RefCell<VecDeque<(usize, T)>>>,
    _marker: PhantomData<&'a ()>, // Borrowing the old FickleVec, so it can't be modified.
}