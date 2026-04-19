use crate::ResourceSet;
use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Debug)]
pub struct SzemerSet {
    inner: BTreeMap<(u32, Vec<(u32, u32)>)>, // (start, steps = [(modd, reste), ...])
    // Quand Vec<(u32, u32)> est vide, c'est un trou
    // Idée :
    // E = ⋃(i ∈ {(0, 0)} ∪ inner \ last) {n ∈ [inner.i.0, inner.(i+1).0) : ∃ k ∈ inner.i.1 : k.left | n - k.right}
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { (a, 1, 0) }
    else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn intersect_classes(d1: u32, r1: u32, d2: u32, r2: u32) -> Option<(u32, u32)> {
    let g = gcd(d1 as u64, d2 as u64) as i64;
    let diff = r2 as i64 - r1 as i64;
    if diff.rem_euclid(g) != 0 { return None; }

    let l = lcm(d1 as u64, d2 as u64) as i64;
    let (_, inv, _) = extended_gcd(d1 as i64 / g, d2 as i64 / g);
    let t = (diff / g * inv).rem_euclid(d2 as i64 / g);
    let r = (r1 as i64 + d1 as i64 * t).rem_euclid(l) as u32;
    Some((l as u32, r))
}

fn count_in_region(start: u32, end: u32, modd: u32, reste: u32) -> usize {
    let first = start + ((reste as i64 - start as i64).rem_euclid(modd as i64)) as u32;
    if first >= end { 0 } else { ((end - first - 1) / modd + 1) as usize }
}

fn count_steps_in_region(start: u32, end: u32, steps: &[(u32, u32)]) -> usize {
    //* inclusion-exclusion sur tous les sous-ensembles non vides de steps :
    //* |A1 ∪ ... ∪ An| = Σ|Ai| - Σ|Ai∩Aj| + Σ|Ai∩Aj∩Ak| - ...
    //* l'intersection de k classes est une PA (ou vide) calculée par CRT itéré
    let n = steps.len();
    let mut total: i64 = 0;
    for mask in 1u32..(1 << n) {
        let indices: Vec<usize> = (0..n).filter(|&i| mask & (1 << i) != 0).collect();
        let sign = if indices.len() % 2 == 1 { 1i64 } else { -1i64 };
        let inter = indices.iter().try_fold(
            (steps[indices[0]].0, steps[indices[0]].1),
            |acc, &i| intersect_classes(acc.0, acc.1, steps[i].0, steps[i].1)
        );
        if let Some((modd, reste)) = inter {
            total += sign * count_in_region(start, end, modd, reste) as i64;
        }
    }
    total as usize
}

impl SzemerSet {
    fn intervals(&self) -> impl Iterator<Item = (&u32, &Vec<(u32, u32)>, &u32)> {
        self.inner
            .iter()
            .zip(self.inner.iter().skip(1))
            .map(|((start, steps), (next_start, _))| (start, steps, next_start))
        // (start, steps, end) où la région est [start, end)
    }

    fn simple_intervals(&self) -> impl Iterator<Item = (&u32, &u32, &u32, &u32)> {
        //* Stratégie
        //* —————————
        //* filtrer les intervalles où steps est vide (trous) et retourner (start, 1, 0, end)
        //* applatir les intervalles restants en (start, modd, reste, end) pour chaque classe)
        self.intervals()
            .flat_map(|(start, steps, end)| {
                if steps.is_empty() {
                    vec![(*start, 0, *end)]
                } else {
                    steps.iter().map(|&(modd, reste)| (*start, modd, reste, *end)).collect()
                }
            })
    }
}

impl ResourceSet for SzemerSet {
    fn new() -> Self {
        SzemerSet { inner: BTreeMap::new() }
    }

    fn singleton(e: u32) -> Self {
        SzemerSet { inner: BTreeMap::from([
            (e,     vec![(1, 0)]),  // région [e, e+1) : x ≡ 0 mod 1 → {e}
            (e + 1, vec![]),        // trou après
        ])}
    }

    fn contains(&self, elem: u32) -> bool {
        //* Stratégie
        //* —————————
        //* trouver la dernière entrée (start, steps) telle que start <= elem
        //* si aucune entrée n'existe :
        //*     elem n'est pas contenu
        //* si steps est vide :
        //*     c'est un trou et elem n'est pas contenu
        //* si ∃ (modd, reste) ∈ steps tel que modd | (elem - reste) :
        //*     elem est contenu
        //* sinon :
        //*     elem n'est pas contenu

        match self.inner.range(..=elem).next_back() {
            None | Some((_, steps)) if steps.is_empty() => false,
            Some((_, steps)) => steps.iter().any(|&(modd, reste)| (elem - reste) % modd == 0),
        }
    }

    fn len(&self) -> usize {
        //* pour chaque région [start, end) :
        //*     compter les éléments par inclusion-exclusion sur les classes
        //* sommer sur toutes les régions
        self.intervals()
            .map(|(start, steps, end)| count_steps_in_region(*start, *end, steps))
            .sum()
    }

    // fn union(&self, other: &Self) -> Self {
    //     //* Stratégie
    //     //* —————————
    // }

    // fn intersection(&self, other: &Self) -> Self {
    //     //* Stratégie
    //     //* —————————
    // }

    // fn difference(&self, other: &Self) -> Self {
    //     //* Stratégie
    //     //* —————————
    // }

    // fn iter(&self) -> Box<dyn Iterator<Item = u32> + '_> {
    // }

    fn serialize(&self) -> String {
        self.simple_intervals()
            .map(|(start, modd, reste, end)| 
                // if *steps != 0 {
                //     format!("{}-{}:{}", *start, *end - 1, *steps)
                // } else if *start != *end - 1 {
                //     format!("{}-{}", *start, *end - 1)
                // } else {
                //     format!("{}", *start)
                // })
                if *modd != 0 {
                    format!("{}-{}:{}", *start, *end - 1, *modd)
                } else if *start != *end - 1 {
                    format!("{}-{}", *start, *end - 1)
                } else {
                    format!("{}", *start)
            .collect::<Vec<_>>()
            .join(" ")
    }

    // fn deserialize(s: &str) -> Self {
    // }
}