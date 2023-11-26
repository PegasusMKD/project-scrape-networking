use std::iter::FusedIterator;

#[derive(Debug, Clone)]
pub enum IndexVec {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl IndexVec {
    /// Returns an iterator over the indices.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        match self {
            IndexVec::U16(vec) => IndexVecIter::U16(vec.iter()),
            IndexVec::U32(vec) => IndexVecIter::U32(vec.iter()),
        }
    }

    /// Returns the number of indices.
    pub fn len(&self) -> usize {
        match self {
            IndexVec::U16(vec) => vec.len(),
            IndexVec::U32(vec) => vec.len(),
        }
    }
}

/// An Iterator for the [`IndexVec`].
enum IndexVecIter<'a> {
    U16(std::slice::Iter<'a, u16>),
    U32(std::slice::Iter<'a, u32>),
}

impl Iterator for IndexVecIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IndexVecIter::U16(iter) => iter.next().map(|val| *val as usize),
            IndexVecIter::U32(iter) => iter.next().map(|val| *val as usize),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            IndexVecIter::U16(iter) => iter.size_hint(),
            IndexVecIter::U32(iter) => iter.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for IndexVecIter<'a> {}
impl<'a> FusedIterator for IndexVecIter<'a> {}
