use super::{Component, Ui};
use std::marker::PhantomData;

/// A component wrapper around a primitive function.
#[derive(Debug)]
pub struct Element<P, F>
where
    F: FnMut(&Ui, &P),
{
    inner: F,
    _props: PhantomData<P>,
}

impl<P, F> Element<P, F>
where
    F: FnMut(&Ui, &P),
{
    /// Creates a new element.
    pub fn new(inner: F) -> Self {
        Self {
            inner,
            _props: PhantomData,
        }
    }
}

impl<P, F> From<F> for Element<P, F>
where
    F: FnMut(&Ui, &P),
{
    fn from(inner: F) -> Self {
        Self::new(inner)
    }
}

impl<P, F> Component for Element<P, F>
where
    F: FnMut(&Ui, &P),
{
    type Props = P;

    fn render(&mut self, ui: &Ui, props: &Self::Props) {
        (self.inner)(ui, props)
    }
}
