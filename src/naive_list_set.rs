use crate::ResourceSet;

#[derive(Clone, Debug)]
pub struct NaiveListSet {
    inner: Vec<u32>,
}

impl PartialEq for NaiveListSet {
    fn eq(&self, other: &Self) -> bool {
        let mut sorted_self = self.inner.clone();
        let mut sorted_other = other.inner.clone();
        sorted_self.sort();
        sorted_other.sort();
        sorted_self == sorted_other
    }
}

impl ResourceSet for NaiveListSet {
    fn new() -> Self {
        NaiveListSet { inner: Vec::new() }
    }

    fn singleton(e: u32) -> Self {
        NaiveListSet { inner: vec![e] }
    }

    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        let mut set = Self::new();
        for e in iter {
            if !set.contains(e) {
                set.inner.push(e);
            }
        }
        set
    }

    fn contains(&self, elem: u32) -> bool {
        self.inner.contains(&elem)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn union(&self, other: &Self) -> Self {
        let mut result = self.inner.clone();
        for elem in &other.inner {
            if !result.contains(elem) {
                result.push(elem.clone());
            }
        }
        NaiveListSet { inner: result }
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for elem in &self.inner {
            if other.contains(elem.clone()) {
                result.push(elem.clone());
            }
        }
        NaiveListSet { inner: result }
    }

    fn difference(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for elem in &self.inner {
            if !other.contains(elem.clone())  {
                result.push(elem.clone());
            }
        }
        NaiveListSet { inner: result }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        let mut sorted = self.inner.clone();
        sorted.sort();
        Box::new(sorted.into_iter())
    }
}