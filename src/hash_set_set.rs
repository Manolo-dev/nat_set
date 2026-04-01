use crate::ResourceSet;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug)]
pub struct HashSetSet<T: std::hash::Hash + Eq> {
    inner: HashSet<T>,
}

impl<T: Eq + std::hash::Hash + Clone + PartialEq + std::fmt::Debug + std::fmt::Display> ResourceSet<T> for HashSetSet<T> {
    fn new() -> Self {
        HashSetSet { inner: HashSet::new() }
    }

    fn singleton(e: T) -> Self {
        HashSetSet { inner: HashSet::from([e]) }
    }

    fn contains(&self, elem: T) -> bool {
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

    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        let mut sorted: Vec<_> = self.inner.iter().collect();
        sorted.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
        Box::new(sorted.into_iter())
    }
}