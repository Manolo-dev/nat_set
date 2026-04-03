use crate::ResourceSet;
use roaring::RoaringBitmap;

#[derive(Clone, PartialEq, Debug)]
pub struct BitSetSet {
    inner: RoaringBitmap,
}

impl ResourceSet for BitSetSet {
    fn new() -> Self {
        BitSetSet { inner: RoaringBitmap::new() }
    }

    fn singleton(e: u32) -> Self {
        let mut bitmap = RoaringBitmap::new();
        bitmap.insert(e);
        BitSetSet { inner: bitmap }
    }

    fn contains(&self, elem: u32) -> bool {
        self.inner.contains(elem)
    }

    fn len(&self) -> usize {
        self.inner.len() as usize
    }

    fn union(&self, other: &Self) -> Self {
        BitSetSet { inner: &self.inner | &other.inner }
    }

    fn intersection(&self, other: &Self) -> Self {
        BitSetSet { inner: &self.inner & &other.inner }
    }

    fn difference(&self, other: &Self) -> Self {
        BitSetSet { inner: &self.inner - &other.inner }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::new(self.inner.iter().map(|x| x as u32))
    }
}