use super::{AbstractMut, IntoAbstract, IntoIterator, Shiperator};

/// Chunk iterator over a single component.  
/// Returns slices `size` long, to get the remaining items (if any) use the `remainder` method.
pub struct ChunkExact1<T: IntoAbstract> {
    pub(crate) data: T::AbsView,
    pub(crate) current: usize,
    pub(crate) end: usize,
    pub(crate) step: usize,
}

impl<T: IntoAbstract> ChunkExact1<T> {
    pub fn remainder(&mut self) -> <T::AbsView as AbstractMut>::Slice {
        let remainder = core::cmp::min(self.end - self.current, self.end % self.step);
        let old_end = self.end;
        self.end -= remainder;
        // SAFE we checked for OOB and the lifetime is ok
        unsafe { self.data.get_data_slice(self.end..old_end) }
    }
}

impl<T: IntoAbstract> Shiperator for ChunkExact1<T> {
    type Item = <T::AbsView as AbstractMut>::Slice;

    fn first_pass(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if current + self.step <= self.end {
            self.current += self.step;
            // SAFE we checked for OOB and the lifetime is ok
            Some(unsafe { self.data.get_data_slice(current..self.current) })
        } else {
            None
        }
    }
    fn post_process(&mut self) {}
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end - self.current) / self.step;
        (len, Some(len))
    }
}

impl<I: IntoAbstract> core::iter::IntoIterator for ChunkExact1<I> {
    type IntoIter = IntoIterator<Self>;
    type Item = <Self as Shiperator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator(self)
    }
}
