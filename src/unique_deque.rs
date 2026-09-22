use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

pub struct UniqueDeque<T: Eq + std::hash::Hash> {
    deque: VecDeque<T>,
    set: HashSet<T>,
}

impl<T: Eq + std::hash::Hash + Clone> UniqueDeque<T> {
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
            set: HashSet::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(capacity),
            set: HashSet::with_capacity(capacity),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.deque.get(index)
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.deque.swap(i, j)
    }

    pub fn capacity(&self) -> usize {
        self.deque.capacity().min(self.set.capacity())
    }

    pub fn reserve(&mut self, additional: usize) {
        self.deque.reserve_exact(additional);
        self.set.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.deque.shrink_to_fit();
        self.set.shrink_to_fit();
    }

    pub fn push_front(&mut self, item: T) {
        if self.set.contains(&item) {
            return;
        }

        self.set.insert(item.clone());
        self.deque.push_front(item);
    }

    pub fn push_back(&mut self, item: T) {
        if self.set.contains(&item) {
            return;
        }

        self.set.insert(item.clone());
        self.deque.push_back(item);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let item = self.deque.pop_front()?;
        self.set.remove(&item);
        Some(item)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let item = self.deque.pop_back()?;
        self.set.remove(&item);
        Some(item)
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let s = self.deque.make_contiguous();
        s.sort_by(compare);
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.sort_by(|a, b| a.cmp(b));
    }
}

impl<T: Eq + std::hash::Hash> Default for UniqueDeque<T> {
    fn default() -> Self {
        Self {
            deque: Default::default(),
            set: Default::default(),
        }
    }
}
