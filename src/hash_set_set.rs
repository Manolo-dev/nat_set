use crate::ResourceSet;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug)]
pub struct HashSetSet {
    inner: HashSet<u32>,
}

impl ResourceSet for HashSetSet {
    fn new() -> Self {
        HashSetSet { inner: HashSet::new() }
    }

    fn singleton(e: u32) -> Self {
        HashSetSet { inner: HashSet::from([e]) }
    }

    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        HashSetSet { inner: iter.into_iter().collect() }
    }

    fn contains(&self, elem: u32) -> bool {
        self.inner.contains(&elem)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn union(&self, other: &Self) -> Self {
        HashSetSet { inner: self.inner.union(&other.inner).cloned().collect() }
    }

    fn intersection(&self, other: &Self) -> Self {
        HashSetSet { inner: self.inner.intersection(&other.inner).cloned().collect() }
    }

    fn difference(&self, other: &Self) -> Self {
        HashSetSet { inner: self.inner.difference(&other.inner).cloned().collect() }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        let mut sorted: Vec<u32> = self.inner.iter().cloned().collect();
        sorted.sort();
        Box::new(sorted.into_iter())
    }
}