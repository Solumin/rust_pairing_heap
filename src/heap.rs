use std::iter::FromIterator;
use std::cmp::Ordering;

#[derive(PartialEq,Debug,Clone)]
pub enum PairingHeap<T: PartialOrd> {
    Empty,
    Tree { elem: T, subheaps: Vec<PairingHeap<T>> },
}

impl<T: PartialOrd> PairingHeap<T> {
    pub fn new() -> Self {
        PairingHeap::Empty
    }

    pub fn root(elem: T) -> Self {
        PairingHeap::Tree{elem, subheaps: Vec::new()}
    }

    pub fn is_empty(&self) -> bool {
        if let PairingHeap::Empty = self {
            true
        } else {
            false
        }
    }

    pub fn find_min(&self) -> Option<&T> {
        if let PairingHeap::Tree{elem, ..} = self {
            Some(elem)
        } else {
            None
        }
    }

    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (PairingHeap::Empty, o) => o,
            (s, PairingHeap::Empty) => s,
            (PairingHeap::Tree{elem: e1, subheaps: mut sub1},
             PairingHeap::Tree{elem: e2, subheaps: mut sub2}) => {
                 if e1 < e2 {
                     sub1.push(PairingHeap::Tree{elem: e2, subheaps: sub2});
                     PairingHeap::Tree {
                         elem: e1,
                         subheaps: sub1,
                     }
                 } else {
                     sub2.push(PairingHeap::Tree{elem: e1, subheaps: sub1});
                     PairingHeap::Tree {
                         elem: e2,
                         subheaps: sub2,
                     }
                 }
             }
        }
    }

    pub fn insert(self, elem: T) -> Self {
        let temp_heap = PairingHeap::Tree{
            elem: elem,
            subheaps: Vec::new(),
        };
        PairingHeap::merge(self, temp_heap)
    }

    pub fn pop_min(self) -> Option<(T, Self)> {
        if let PairingHeap::Tree{elem, subheaps} = self {
            let mut pair_stack: Vec<PairingHeap<T>> = Vec::new();
            let mut heap_iter = subheaps.into_iter();
            while let Some(h1) = heap_iter.next() {
                let h2 = heap_iter.next().unwrap_or_else(PairingHeap::new);
                pair_stack.push(PairingHeap::merge(h1, h2));
            }
            while pair_stack.len() > 1 {
                let h1 = pair_stack.pop().unwrap_or_else(PairingHeap::new);
                let h2 = pair_stack.pop().unwrap_or_else(PairingHeap::new);
                pair_stack.push(PairingHeap::merge(h1, h2));
            }
            Some((elem, pair_stack.pop().unwrap_or_else(PairingHeap::new)))
        } else {
            None
        }
    }
}

impl<T: PartialOrd> PartialOrd for PairingHeap<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (PairingHeap::Empty, PairingHeap::Empty) => Some(Ordering::Equal),
            (PairingHeap::Empty, PairingHeap::Tree{..}) => Some(Ordering::Less),
            (PairingHeap::Tree{..}, PairingHeap::Empty) => Some(Ordering::Greater),
            (PairingHeap::Tree{elem: e1, ..}, PairingHeap::Tree{elem: e2, ..}) => e1.partial_cmp(e2)
        }
    }
}

impl<T: PartialOrd> FromIterator<T> for PairingHeap<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        iter.into_iter().fold(PairingHeap::new(), PairingHeap::insert)
    }
}

impl<T: PartialOrd> Default for PairingHeap<T> {
    fn default() -> Self {
        PairingHeap::Empty
    }
}

#[cfg(test)]
mod test {
    use super::PairingHeap;
    #[test]
    fn test_new() {
        let heap: PairingHeap<u32> = PairingHeap::new();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_insert() {
        let heap = PairingHeap::root(1);
        assert!(!heap.is_empty());
        assert_eq!(heap.find_min(), Some(&1));
        if let PairingHeap::Tree{elem, subheaps} = heap {
            assert_eq!(elem, 1);
            assert!(subheaps.is_empty());
        }
    }

    #[test]
    fn test_insert_many() {
        // Insert adds all elements as subheaps of the root.
        let h = (1..=7).fold(PairingHeap::new(), PairingHeap::insert);
        if let PairingHeap::Tree{elem, subheaps} = h {
            assert_eq!(elem, 1);
            assert_eq!(subheaps.iter().map(|c| *c.find_min().unwrap()).collect::<Vec<u32>>(), (2..=7).collect::<Vec<_>>());
        }
    }

    #[test]
    fn test_merge_empty() {
        let heap1 = PairingHeap::root(1);
        let heap2 = PairingHeap::new();
        let heap3 = PairingHeap::merge(heap1, heap2);
        if let PairingHeap::Tree{elem, subheaps} = heap3 {
            assert_eq!(elem, 1);
            assert!(subheaps.is_empty());
        }
    }

    #[test]
    fn test_pop_min() {
        let h = PairingHeap::root(1);
        let res = h.pop_min();
        assert!(res.is_some());
        if let Some((elem, h)) = res {
            assert_eq!(elem, 1);
            assert!(h.is_empty());
        }
    }

    #[test]
    fn test_pop_min_empty() {
        let h: PairingHeap<u32> = PairingHeap::new();
        assert!(h.pop_min().is_none());
    }

    #[test]
    fn test_pop_min_rebalance() {
        let temp: PairingHeap<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7].into_iter().collect();
        let c7 = PairingHeap::root(7);
        let c6 = PairingHeap::root(6);
        let c5 = PairingHeap::Tree {
            elem: 5,
            subheaps: vec![c6, c7],
        };
        let c4 = PairingHeap::root(4);
        let c3 = PairingHeap::Tree{
            elem: 3,
            subheaps: vec![c4, c5],
        };
        let c2 = PairingHeap::root(2);
        let c1 = PairingHeap::Tree{
            elem: 1,
            subheaps: vec![c2, c3],
        };
        let res = temp.pop_min();
        assert!(res.is_some());
        if let Some((root_elem, h)) = res {
            assert_eq!(root_elem, 0);
            assert_eq!(h, c1);
        }
    }
}
