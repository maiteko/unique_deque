use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

/// deque structure where data is associated with a unique key.
/// if new data is pushed with the same key, the old data will be removed and returned
pub struct KeyDeque<K: Eq + std::hash::Hash + Copy, T> {
    deque: VecDeque<(K, T)>,
    set: HashSet<K>,
}

impl<K: Eq + std::hash::Hash + Copy, T> KeyDeque<K, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(capacity),
            set: HashSet::with_capacity(capacity),
        }
    }

    pub fn get(&self, index: usize) -> Option<&(K, T)> {
        self.deque.get(index)
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.deque.swap(i, j)
    }

    pub fn capacity(&self) -> usize {
        self.deque.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.deque.reserve_exact(additional);
        let difference = self.deque.capacity() as i64 - self.set.capacity() as i64;
        if difference > 0 {
            self.set.reserve(difference as usize);
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.deque.shrink_to_fit();
        self.set.shrink_to_fit();
    }

    /// pushes to the front of the queue
    /// if the key already exists, it instead replaces the value at that location
    pub fn push_front(&mut self, key: K, item: T) -> Option<(K, T)> {
        let idx = if self.set.contains(&key) {
            self.deque.iter().position(|x| x.0 == key)
        } else {
            None
        };

        self.set.insert(key);
        self.deque.push_front((key, item));

        self.deque.swap(idx?, self.deque.len() - 1);
        self.deque.pop_front()
    }

    /// pushes to the back of the queue
    /// if the key already exists, it instead replaces the value at that location
    pub fn push_back(&mut self, key: K, item: T) -> Option<(K, T)> {
        let idx = if self.set.contains(&key) {
            self.deque.iter().position(|x| x.0 == key)
        } else {
            None
        };

        self.set.insert(key);
        self.deque.push_back((key, item));

        self.deque.swap(idx?, self.deque.len() - 1);
        self.deque.pop_back()
    }

    pub fn pop_front(&mut self) -> Option<(K, T)> {
        let item = self.deque.pop_front()?;
        self.set.remove(&item.0);
        Some(item)
    }

    pub fn pop_back(&mut self) -> Option<(K, T)> {
        let item = self.deque.pop_back()?;
        self.set.remove(&item.0);
        Some(item)
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&(K, T), &(K, T)) -> Ordering,
    {
        let s = self.deque.make_contiguous();
        s.sort_by(compare);
    }

    pub fn sort_by_keys(&mut self)
    where
        K: Ord,
    {
        self.sort_by(|a, b| a.0.cmp(&b.0));
    }

    pub fn sort_by_values(&mut self)
    where
        T: Ord,
    {
        self.sort_by(|a, b| a.1.cmp(&b.1));
    }
}

impl<K, T> Default for KeyDeque<K, T>
where
    K: Eq + std::hash::Hash + Copy,
{
    fn default() -> Self {
        Self {
            deque: Default::default(),
            set: Default::default(),
        }
    }
}
