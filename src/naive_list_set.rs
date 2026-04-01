use crate::ResourceSet;

#[derive(Clone, PartialEq, Debug)]
pub struct NaiveListSet<T> {
    inner: Vec<T>,
}

impl<T: Clone + PartialEq + std::fmt::Debug + std::fmt::Display> ResourceSet<T> for NaiveListSet<T> {
    fn new() -> Self {
        NaiveListSet { inner: Vec::new() }
    }

    fn singleton(e: T) -> Self {
        NaiveListSet { inner: vec![e] }
    }

    fn contains(&self, elem: T) -> bool {
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

    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        // need to sort
        let mut sorted: Vec<_> = self.inner.iter().collect();
        sorted.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
        Box::new(sorted.into_iter())
    }
}