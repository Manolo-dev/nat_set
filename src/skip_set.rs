use crate::ResourceSet;

const MAX_LEVEL: usize = 16;
const P: f64 = 0.5;

#[derive(Clone, PartialEq, Debug)]
struct Node {
    value: u32,
    forward: Vec<Node>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SkipSet {
    head: Node,
    level: usize,
    len: usize,
}

impl SkipSet {
    fn random_level() -> usize {
        let mut lvl = 0;
        while rand::random::<f64>() < P && lvl < MAX_LEVEL - 1 {
            lvl += 1;
        }
        lvl
    }

    fn insert(&mut self, elem: u32) {
        let mut update = vec![&mut self.head; MAX_LEVEL];
        let mut current = &mut self.head;

        for i in (0..=self.level).rev() {
            while current.forward[i].value < elem {
                current = &mut current.forward[i];
            }
            update[i] = current;
        }

        if current.forward[0].value == elem {
            return; // Element already exists
        }

        let new_level = Self::random_level();
        if new_level > self.level {
            for i in self.level + 1..=new_level {
                update[i] = &mut self.head;
            }
            self.level = new_level;
        }

        let new_node = Node {
            value: elem,
            forward: vec![Node { value: 0, forward: Vec::new() }; new_level + 1],
        };

        for i in 0..=new_level {
            new_node.forward[i] = update[i].forward[i].clone();
            update[i].forward[i] = new_node.clone();
        }

        self.len += 1;
    }

    fn forward_iter(&self) -> impl Iterator<Item = u32> + '_ {
        let mut current = &self.head.forward[0];
        std::iter::from_fn(move || {
            if current.value == 0 {
                None
            } else {
                let value = current.value;
                current = &current.forward[0];
                Some(value)
            }
        })
    }
}

impl ResourceSet for SkipSet {
    fn new() -> Self {
        SkipSet {
            head: Node {
                value: 0,
                forward: vec![Node { value: 0, forward: Vec::new() }; MAX_LEVEL],
            },
            level: 0,
            len: 0,
        }
    }

    fn singleton(e: u32) -> Self {
        let mut set = Self::new();
        set.insert(e);
        set
    }

    fn contains(&self, elem: u32) -> bool {
        let mut current = &self.head;
        for i in (0..=self.level).rev() {
            while current.forward[i].value < elem {
                current = &current.forward[i];
            }
        }
        current.forward[0].value == elem
    }

    fn len(&self) -> usize {
        self.len
    }

    fn union(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for elem in self.iter().chain(other.iter()) {
            result.insert(elem);
        }
        result
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for elem in self.iter() {
            if other.contains(elem) {
                result.insert(elem);
            }
        }
        result
    }

    fn difference(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for elem in self.iter() {
            if !other.contains(elem) {
                result.insert(elem);
            }
        }
        result
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::new(self.forward_iter())
    }
}