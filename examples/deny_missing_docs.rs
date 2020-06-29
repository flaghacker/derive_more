#![deny(missing_docs)]
//! Some docs

#[macro_use]
extern crate derive_more;

fn main() {}

/// Some docs
pub struct MyVec(Vec<i32>);
impl<__IdxT, __IdxOutputT: ?Sized> ::core::ops::Index<__IdxT> for MyVec
where
    Vec<i32>: ::core::ops::Index<__IdxT, Output = __IdxOutputT>,
{
    type Output = __IdxOutputT;
    #[inline]
    fn index(&self, idx: __IdxT) -> &Self::Output {
        let indexable = &self.0;
        <Vec<i32> as ::core::ops::Index<__IdxT>>::index(indexable, idx)
    }
}
impl<__IdxT> ::core::ops::IndexMut<__IdxT> for MyVec
where
    Vec<i32>: ::core::ops::IndexMut<__IdxT>,
{
    #[inline]
    fn index_mut(&mut self, idx: __IdxT) -> &mut Self::Output {
        let indexable = &mut self.0;
        <Vec<i32> as ::core::ops::IndexMut<__IdxT>>::index_mut(indexable, idx)
    }
}
