use std::{cell::{Ref, RefCell}, rc::Rc};

use crate::{internals::Link, ref_hack::RefHack};

pub struct FickleVec<T>(FickleVecImpl<T>);

enum FickleVecImpl<T> {
    Primary(Rc<RefCell<Vec<T>>>), // A tiny bit too much indirection for my tastes.
    Derived(usize, T, Link<FickleVec<T>>), // The link is only ever absent while mutating the structure.
}

impl<T> FickleVec<T> {
    pub fn new() -> Self {
        Self(FickleVecImpl::Primary(Rc::new(RefCell::new(Vec::new()))))
    }
}

impl<T> Default for FickleVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FickleVec<T> {
    fn get(&self, index: usize) -> RefHack<T> {
        match &self.0 {
            FickleVecImpl::Primary(ref_cell) => Ref::map(ref_cell.borrow(), |v| &v[index]).into(),
            FickleVecImpl::Derived(_, _, _) => {
                let mut curr = Some(self); // Backing up, it's easier this way.
                while let Some(FickleVec(FickleVecImpl::Derived(written_index, written_value, next))) = curr.as_ref() {
                    if index == *written_index {
                        return written_value.into();
                    }
                }
                let Some(FickleVec(Fickle))
                todo!()

            }
        }

    }
}