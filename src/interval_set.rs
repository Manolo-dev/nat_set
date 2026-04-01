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
        IntervalSet { inner: BTreeSet::from([(e, e)]) }
    }

    fn contains(&self, elem: u32) -> bool {
        self.inner.range(..=(elem, u32::MAX))
            .next_back()
            .map(|(start, end)| elem >= *start && elem <= *end)
            .unwrap_or(false)
    }

    fn len(&self) -> usize {
        self.inner.iter().map(|(start, end)| (end - start + 1) as usize).sum()
    
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

    fn iter(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::new(self.inner.iter().flat_map(|(start, end)| (*start..=*end).into_iter()))
    }
}