use crate::ResourceSet;
use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Debug)]
pub struct IntervalSet {
    inner: BTreeSet<u32>,
}

impl IntervalSet {
    fn intervals(&self) -> impl Iterator<Item = (&u32, &u32)> {
        self.inner
            .iter()
            .step_by(2)
            .zip(self.inner
                .iter()
                .skip(1)
                .step_by(2)
            )
    }
}

impl ResourceSet for IntervalSet {
    fn new() -> Self {
        IntervalSet { inner: BTreeSet::new() }
    }

    fn singleton(e: u32) -> Self {
        IntervalSet { inner: BTreeSet::from([e, e + 1]) }
    }

    fn contains(&self, elem: u32) -> bool {
        //* Stratégie
        //* —————————
        //* recherche dichotomique pour l'interval [a, b) tel que elem in [a, b)
        //* si l'index de a est pair :
        //*     c'est un début d'interval et elem n'est pas contenu
        //* sinon :
        //*     c'est une fin d'interval et elem est contenu
        
        let mut low = 0;
        let mut high = self.inner.len();

        while low < high {
            let mid = (low + high) / 2;
            let mid_val = *self.inner.iter().nth(mid).unwrap();
            if mid_val <= elem {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        
        low % 2 == 1
    }

    fn len(&self) -> usize {
        self.intervals()
            .map(|(start, end)| (end - start) as usize).sum()
    }

    fn union(&self, other: &Self) -> Self {
        //* Stratégie
        //* —————————
        //* fusionner les deux listes d'intervalles et trier par début
        //* parcourir la liste fusionnée et fusionner les intervalles qui se chevauchent

        let mut intervals: Vec<(u32, u32)> = self.intervals()
            .chain(other.intervals())
            .map(|(&s, &e)| (s, e))
            .collect();

        intervals.sort_by_key(|&(s, _)| s);

        let mut merged: Vec<(u32, u32)> = Vec::new();
        for (start, end) in intervals {
            match merged.last_mut() {
                Some(last) if start <= last.1 => last.1 = last.1.max(end),
                _ => merged.push((start, end)),
            }
        }

        let mut inner = BTreeSet::new();
        for (start, end) in merged {
            inner.insert(start);
            inner.insert(end);
        }

        IntervalSet { inner }
    }

    fn intersection(&self, other: &Self) -> Self {
        //* Stratégie
        //* —————————
        //* parcourir les deux listes d'intervalles en parallèle et trouver les intersections
        //* si les intervalles [s1, e1) et [s2, e2) se chevauchent, alors l'intersection est [max(s1, s2), min(e1, e2))

        let mut a = self.intervals().map(|(&s, &e)| (s, e)).peekable();
        let mut b = other.intervals().map(|(&s, &e)| (s, e)).peekable();
        let mut inner = BTreeSet::new();

        while let (Some(&(s1, e1)), Some(&(s2, e2))) = (a.peek(), b.peek()) {
            let start = s1.max(s2);
            let end   = e1.min(e2);
            if start < end {
                inner.insert(start);
                inner.insert(end);
            }
            if e1 < e2 { a.next(); } else { b.next(); }
        }

        IntervalSet { inner }
    }

    fn difference(&self, other: &Self) -> Self {
        //* Stratégie
        //* —————————
        //* parcourir les intervalles de self et other en parallèle
        //* pour chaque intervalle [s1, e1) de self :
        //*     sauter les intervalles de other qui se terminent avant cur
        //*     pour chaque intervalle [s2, e2) de other qui chevauche [s1, e1) :
        //*         si s2 > cur : émettre [cur, s2)
        //*         avancer cur à max(cur, e2)
        //*         si e2 déborde au delà de e1 : garder [s2, e2) pour le prochain intervalle de self
        //*         sinon : avancer dans other
        //*     émettre le reste [cur, e1) si cur < e1

        let mut b = other.intervals().map(|(&s, &e)| (s, e)).peekable();
        let mut inner = BTreeSet::new();

        for (&s1, &e1) in self.intervals() {
            let mut cur = s1;

            while b.peek().map_or(false, |&(_, e2)| e2 <= cur) {
                b.next();
            }

            while let Some(&(s2, e2)) = b.peek() {
                if s2 >= e1 { break; }
                if s2 > cur {
                    inner.insert(cur);
                    inner.insert(s2);
                }
                cur = cur.max(e2);
                if e2 >= e1 { break; }
                else { b.next(); }
            }

            if cur < e1 {
                inner.insert(cur);
                inner.insert(e1);
            }
        }

        IntervalSet { inner }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::new(self.intervals()
            .flat_map(|(start, end)| (*start..=*end-1).into_iter())
        )
    }

    fn serialize(&self) -> String {
        self.intervals()
            .map(|(start, end)| if *start != *end - 1 { format!("{}-{}", *start, *end - 1) } else { format!("{}", *start) })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn deserialize(s: &str) -> Self {
        let mut inner = BTreeSet::new();
        for part in s.split_whitespace() {
            if let Some((start, end)) = part.split_once('-') {
                if let (Ok(s), Ok(e)) = (start.parse::<u32>(), end.parse::<u32>()) {
                    inner.insert(s);
                    inner.insert(e + 1);
                }
            } else if let Ok(e) = part.parse::<u32>() {
                inner.insert(e);
                inner.insert(e + 1);
            }
        }

        IntervalSet { inner: inner.clone() }
    }
}