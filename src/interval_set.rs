use crate::ResourceSet;
use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Debug)]
pub struct IntervalSet {
    inner: BTreeSet<(u32, u32)>,
}

impl ResourceSet for IntervalSet {
    fn new() -> Self {
        IntervalSet { inner: BTreeSet::new() }
    }

    fn singleton(e: u32) -> Self {
        IntervalSet { inner: BTreeSet::from([(e, e)])
    }

    fn contains(&self, elem: u32) -> bool {
        self.inner.range(..=(n, u32::MAX))
            .next_back()
            .map(|(start, end)| n >= *start && n <= *end)
            .unwrap_or(false)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn union(&self, other: &Self) -> Self {
        IntervalSet { inner: self.inner.union(&other.inner).cloned().collect() }
    }

    fn intersection(&self, other: &Self) -> Self {
        IntervalSet { inner: self.inner.intersection(&other.inner).cloned().collect() }
    }

    fn difference(&self, other: &Self) -> Self {
        IntervalSet { inner: self.inner.difference(&other.inner).cloned().collect() }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &u32> + '_> {
        let mut sorted: Vec<_> = self.inner.iter().collect();
        sorted.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
        Box::new(sorted.into_iter())
    }
}