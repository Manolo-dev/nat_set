pub mod naive_list_set;
pub mod hash_set_set;
pub mod interval_set;

pub trait ResourceSet: Clone + PartialEq + std::fmt::Debug {
    fn new() -> Self;
    fn singleton(e: u32) -> Self;
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        iter.into_iter().fold(Self::new(), |acc, e| acc.union(&Self::singleton(e)))
    }
    fn contains(&self, elem: u32) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn union(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;
    fn difference(&self, other: &Self) -> Self;
    fn iter(&self) -> Box<dyn Iterator<Item = u32> + '_>;
    fn serialize(&self) -> String {
        String::from("") + &self.iter().map(|e| format!("{}", e)).collect::<Vec<_>>().join(", ") + ""
    }
}

#[cfg(test)]
mod tests { macro_rules! tests_for { ($T:ty, $mod:ident) => { mod $mod {
    use crate::*;

    fn make_sets() -> ($T, $T, $T, $T, $T, $T) {
        (
            <$T>::from_iter([3, 1, 4, 1, 5, 9, 2, 6]),
            <$T>::from_iter([10, 7, 8, 7, 6]),
            <$T>::from_iter([100, 200, 150]),
            <$T>::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9]),
            <$T>::new(),
            <$T>::from_iter([42]),
        )
    }
    
    #[test]
    fn test_len() {
        let (a, b, c, d, e, f) = make_sets();
        assert_eq!(a.len(), 7);
        assert_eq!(b.len(), 4);
        assert_eq!(c.len(), 3);
        assert_eq!(d.len(), 9);
        assert_eq!(e.len(), 0);
        assert_eq!(f.len(), 1);
    }

    #[test]
    fn test_is_empty() {
        let (a, _, _, _, e, _) = make_sets();
        assert!( e.is_empty());
        assert!(!a.is_empty());
    }

    #[test]
    fn test_contains() {
        let (a, b, c, _, e, f) = make_sets();
        assert!( a.contains(9));
        assert!(!a.contains(7));
        assert!( b.contains(6));
        assert!(!b.contains(9));
        assert!( c.contains(150));
        assert!(!c.contains(125));
        assert!(!e.contains(0));
        assert!( f.contains(42));
        assert!(!f.contains(43));
    }

    #[test]
    fn test_union() {
        let (a, b, _, _, e, f) = make_sets();
        assert_eq!(a.union(&e), a);
        assert_eq!(e.union(&a), a);
        assert_eq!(a.union(&b), b.union(&a));
        assert_eq!(a.union(&a), a);
        let ab = a.union(&b);
        for x in [1,2,3,4,5,6,7,8,9,10] { assert!(ab.contains(x)); }
        assert!(!ab.contains(11));
        let af = a.union(&f);
        assert!(af.contains(42));
        assert_eq!(af.len(), a.len() + 1);
    }

    #[test]
    fn test_intersection() {
        let (a, b, _, d, e, _) = make_sets();
        assert_eq!(a.intersection(&e), e);
        assert_eq!(a.intersection(&b), b.intersection(&a));
        assert_eq!(a.intersection(&a), a);
        let ab = a.intersection(&b);
        assert_eq!(ab.len(), 1);
        assert!(ab.contains(6));
        assert_eq!(a.intersection(&d), a);
    }

    #[test]
    fn test_difference() {
        let (a, b, _, _, e, f) = make_sets();
        assert_eq!(a.difference(&e), a);
        assert_eq!(a.difference(&a), e);
        let ab = a.difference(&b);
        assert_eq!(ab.len(), 6);
        assert!(!ab.contains(6));
        assert!( ab.contains(9));
        assert_ne!(a.difference(&b), b.difference(&a));
        assert_eq!(a.difference(&f), a);
    }

    #[test]
    fn test_iter() {
        let (a, _, _, _, e, f) = make_sets();
        assert_eq!(e.iter().count(), 0);
        assert_eq!(a.iter().count(), a.len());
        let collected: Vec<u32> = a.iter().collect();
        for x in [1,2,3,4,5,6,9] { assert!(collected.contains(&x)); }
        assert_eq!(f.iter().collect::<Vec<_>>(), vec![42]);
    }

    }};}

    tests_for!(naive_list_set::NaiveListSet, naive_list_set);
    tests_for!(hash_set_set::HashSetSet    , hash_set_set);
    tests_for!(interval_set::IntervalSet   , interval_set);
}