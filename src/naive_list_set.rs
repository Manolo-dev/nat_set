use crate::ResourceSet;

#[derive(Clone, PartialEq, Debug)]
pub struct NaiveListSet {
    inner: Vec<u32>,
}

impl ResourceSet for NaiveListSet {
    fn new() -> Self {
        NaiveListSet { inner: Vec::new() }
    }

    fn singleton(e: u32) -> Self {
        NaiveListSet { inner: vec![e] }
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