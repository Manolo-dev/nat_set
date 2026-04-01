pub mod naive_list_set;
pub mod hash_set_set;
pub mod interval_set;

pub trait ResourceSet {
    fn new() -> Self;
    fn singleton(e: u32) -> Self;
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
mod tests {
    use super::*;
    use crate::naive_list_set::NaiveListSet;

    fn test_set_operations<S: ResourceSet + PartialEq + std::fmt::Debug>() {
        let mut a = S::new();
        let mut b = S::new();

        a = a.union(&S::singleton(1)).union(&S::singleton(2)).union(&S::singleton(3));
        b = b.union(&S::singleton(2)).union(&S::singleton(3)).union(&S::singleton(4));

        assert_eq!(a.len(), 3);
        assert_eq!(b.len(), 3);
        assert!(a.contains(1));
        assert!(!a.contains(4));
        assert!(b.contains(4));

        let u = a.union(&b);
        let i = a.intersection(&b);
        let d = a.difference(&b);

        let expected_u = S::singleton(1).union(&S::singleton(2)).union(&S::singleton(3)).union(&S::singleton(4));
        let expected_i = S::singleton(2).union(&S::singleton(3));
        let expected_d = S::singleton(1);

        println!("Union: {}", u.serialize());

        assert_eq!(u.difference(&expected_u), S::new());
        assert_eq!(i.difference(&expected_i), S::new());
        assert_eq!(d.difference(&expected_d), S::new());
    }

    #[test]
    fn test_implementation() {
        test_set_operations::<NaiveListSet>();
        test_set_operations::<hash_set_set::HashSetSet>();
        test_set_operations::<interval_set::IntervalSet>();
    }
}