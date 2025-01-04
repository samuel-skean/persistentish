use std::{cell::Ref, ops::Deref};

pub enum RefHack<'a, T> {
    DynamicallyChecked(Ref<'a, T>),
    CompileChecked(&'a T),
}

impl<'a, T> Deref for RefHack<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            RefHack::DynamicallyChecked(r) => &*r,
            RefHack::CompileChecked(r) => r,
        }
    }
}

impl<'a, T> From<Ref<'a, T>> for RefHack<'a, T> {
    fn from(value: Ref<'a, T>) -> Self {
        Self::DynamicallyChecked(value)
    }
}

impl<'a, T> From<&'a T> for RefHack<'a, T> {
    fn from(value: &'a T) -> Self {
        Self::CompileChecked(value)
    }
}