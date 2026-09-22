use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque, vec_deque::Iter},
};

/// A [`VecDeque`] of `(K, T)` pairs keyed by a unique `K`.
///
/// `KeyDeque` keeps its `(K, T)` entries in a `VecDeque` and mirrors the keys
/// into a [`HashSet`], so a push can detect a key collision in O(1); locating the
/// slot to overwrite is O(n).
///
/// - Pushing with a **new** key inserts the pair at the front or back (depending
///   on the method) and returns `None`.
/// - Pushing with a key that **already exists** overwrites the existing entry in
///   place, preserving the relative order of the other entries, and returns the
///   displaced `(K, T)`.
/// - Popping removes the pair from both the deque and the key set, so the key
///   may be reused later.
///
/// # Examples
///
/// ```
/// use unique_deque::KeyDeque;
///
/// let mut q: KeyDeque<usize, &str> = KeyDeque::new();
/// q.push_back(1, "a");
/// q.push_back(2, "b");
/// assert_eq!(q.get(0), Some(&(1, "a")));
/// assert_eq!(q.get(1), Some(&(2, "b")));
///
/// // Re-pushing key `1` overwrites in place and returns the old pair.
/// assert_eq!(q.push_back(1, "c"), Some((1, "a")));
/// assert_eq!(q.get(0), Some(&(1, "c")));
/// assert_eq!(q.get(1), Some(&(2, "b")));
/// ```
pub struct KeyDeque<K: Eq + std::hash::Hash + Copy, T> {
    deque: VecDeque<(K, T)>,
    set: HashSet<K>,
}

impl<K: Eq + std::hash::Hash + Copy, T> KeyDeque<K, T> {
    /// Creates a new, empty `KeyDeque`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new, empty `KeyDeque` with at least `capacity` reserved.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(capacity),
            set: HashSet::with_capacity(capacity),
        }
    }

    /// Returns a reference to the `(K, T)` pair at `index`, or `None` if out of
    /// bounds.
    pub fn get(&self, index: usize) -> Option<&(K, T)> {
        self.deque.get(index)
    }

    /// Swaps the entries at positions `i` and `j`.
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    pub fn swap(&mut self, i: usize, j: usize) {
        self.deque.swap(i, j)
    }

    /// Returns the underlying deque capacity.
    pub fn capacity(&self) -> usize {
        self.deque.capacity()
    }

    /// Reserves capacity for at least `additional` more entries, growing the
    /// backing key set to match.
    pub fn reserve(&mut self, additional: usize) {
        self.deque.reserve_exact(additional);
        let difference = self.deque.capacity() as i64 - self.set.capacity() as i64;
        if difference > 0 {
            self.set.reserve(difference as usize);
        }
    }

    /// Shrinks the internal buffers to fit the current number of entries.
    pub fn shrink_to_fit(&mut self) {
        self.deque.shrink_to_fit();
        self.set.shrink_to_fit();
    }

    /// Pushes `(key, item)` to the front of the deque.
    ///
    /// If `key` is new, the pair is inserted at the front and `None` is
    /// returned. If `key` already exists, the existing entry is overwritten in
    /// place and the displaced `(K, T)` is returned.
    pub fn push_front(&mut self, key: K, item: T) -> Option<(K, T)> {
        let idx = if self.set.contains(&key) {
            self.deque.iter().position(|x| x.0 == key)
        } else {
            None
        };

        self.set.insert(key);
        self.deque.push_front((key, item));

        // For a brand new key `idx` is `None` and we short-circuit to returning
        // `None`. Otherwise the old entry shifted one slot back by virtue of the
        // `push_front`, so swap it to the front (where the new entry now sits)
        // and pop it, returning the old value.
        self.deque.swap(idx? + 1, 0);
        self.deque.pop_front()
    }

    /// Pushes `(key, item)` to the back of the deque.
    ///
    /// If `key` is new, the pair is inserted at the back and `None` is
    /// returned. If `key` already exists, the existing entry is overwritten in
    /// place and the displaced `(K, T)` is returned.
    pub fn push_back(&mut self, key: K, item: T) -> Option<(K, T)> {
        let idx = if self.set.contains(&key) {
            self.deque.iter().position(|x| x.0 == key)
        } else {
            None
        };

        self.set.insert(key);
        self.deque.push_back((key, item));

        // For a brand new key `idx` is `None` and we short-circuit to returning
        // `None`. Otherwise swap the old entry to the back (where the new entry
        // now sits) and pop it, returning the old value while the new entry
        // stays in place.
        self.deque.swap(idx?, self.deque.len() - 1);
        self.deque.pop_back()
    }

    /// Removes and returns the front `(K, T)` pair, if any, and removes its key
    /// from the key set so it may be reused.
    pub fn pop_front(&mut self) -> Option<(K, T)> {
        let item = self.deque.pop_front()?;
        self.set.remove(&item.0);
        Some(item)
    }

    /// Removes and returns the back `(K, T)` pair, if any, and removes its key
    /// from the key set so it may be reused.
    pub fn pop_back(&mut self) -> Option<(K, T)> {
        let item = self.deque.pop_back()?;
        self.set.remove(&item.0);
        Some(item)
    }

    /// Sorts the entries in place using `compare`.
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&(K, T), &(K, T)) -> Ordering,
    {
        let s = self.deque.make_contiguous();
        s.sort_by(compare);
    }

    /// Sorts the entries in place by their keys.
    pub fn sort_by_keys(&mut self)
    where
        K: Ord,
    {
        self.sort_by(|a, b| a.0.cmp(&b.0));
    }

    /// Sorts the entries in place by their values.
    pub fn sort_by_values(&mut self)
    where
        T: Ord,
    {
        self.sort_by(|a, b| a.1.cmp(&b.1));
    }

    /// Returns `true` if the UniqueDeque contains no elements
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Returns `true` if the UniqueDeque contains `item`
    pub fn contains_key(&self, key: &K) -> bool {
        self.set.contains(key)
    }

    /// Returns the number of elements in the UniqueDeque
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Returns a front-to-back iterator
    pub fn iter<'a>(&'a self) -> Iter<'a, (K, T)> {
        self.deque.iter()
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

#[cfg(test)]
mod tests {
    use super::KeyDeque;

    #[test]
    fn push_back_new_keys() {
        let mut q = KeyDeque::<usize, &str>::new();
        assert_eq!(q.push_back(1, "a"), None);
        assert_eq!(q.push_back(2, "b"), None);
        assert_eq!(q.get(0), Some(&(1, "a")));
        assert_eq!(q.get(1), Some(&(2, "b")));
    }

    #[test]
    fn push_front_new_keys() {
        let mut q = KeyDeque::<usize, &str>::new();
        assert_eq!(q.push_front(1, "a"), None);
        assert_eq!(q.push_front(2, "b"), None);
        assert_eq!(q.get(0), Some(&(2, "b")));
        assert_eq!(q.get(1), Some(&(1, "a")));
    }

    #[test]
    fn push_back_duplicate_replaces_in_place() {
        let mut q = KeyDeque::<usize, &str>::new();
        q.push_back(1, "a");
        q.push_back(2, "b");
        assert_eq!(q.push_back(1, "c"), Some((1, "a")));
        assert_eq!(q.get(0), Some(&(1, "c")));
        assert_eq!(q.get(1), Some(&(2, "b")));
    }

    #[test]
    fn push_front_duplicate_replaces_in_place() {
        let mut q = KeyDeque::<usize, &str>::new();
        q.push_back(1, "a");
        q.push_back(2, "b");
        assert_eq!(q.push_front(1, "c"), Some((1, "a")));
        assert_eq!(q.get(0), Some(&(1, "c")));
        assert_eq!(q.get(1), Some(&(2, "b")));
    }

    #[test]
    fn duplicate_key_does_not_duplicate() {
        let mut q = KeyDeque::<usize, &str>::new();
        q.push_back(1, "a");
        q.push_back(1, "b");
        q.push_back(1, "c");
        assert_eq!(q.get(0), Some(&(1, "c")));
        assert_eq!(q.get(1), None);
    }

    #[test]
    fn pop_front_removes_key() {
        let mut q = KeyDeque::<usize, &str>::new();
        q.push_back(1, "a");
        q.push_back(2, "b");
        assert_eq!(q.pop_front(), Some((1, "a")));
        // Key `1` was removed from the set, so pushing it again is a new key.
        assert_eq!(q.push_back(1, "z"), None);
        assert_eq!(q.get(1), Some(&(1, "z")));
        assert_eq!(q.get(0), Some(&(2, "b")));
    }

    #[test]
    fn pop_back_removes_key() {
        let mut q = KeyDeque::<usize, &str>::new();
        q.push_back(1, "a");
        q.push_back(2, "b");
        assert_eq!(q.pop_back(), Some((2, "b")));
        assert_eq!(q.push_front(2, "z"), None);
        assert_eq!(q.get(0), Some(&(2, "z")));
        assert_eq!(q.get(1), Some(&(1, "a")));
    }

    #[test]
    fn sort_by_keys() {
        let mut q = KeyDeque::<usize, &str>::new();
        q.push_back(3, "c");
        q.push_back(1, "a");
        q.push_back(2, "b");
        q.sort_by_keys();
        assert_eq!(q.get(0).map(|(k, _)| *k), Some(1));
        assert_eq!(q.get(1).map(|(k, _)| *k), Some(2));
        assert_eq!(q.get(2).map(|(k, _)| *k), Some(3));
    }

    #[test]
    fn sort_by_values() {
        let mut q = KeyDeque::<usize, i32>::new();
        q.push_back(3, 30);
        q.push_back(1, 10);
        q.push_back(2, 20);
        q.sort_by_values();
        assert_eq!(q.get(0).map(|(_, v)| *v), Some(10));
        assert_eq!(q.get(1).map(|(_, v)| *v), Some(20));
        assert_eq!(q.get(2).map(|(_, v)| *v), Some(30));
    }

    #[test]
    fn with_capacity_and_reserve() {
        let mut q = KeyDeque::<usize, u32>::with_capacity(4);
        assert!(q.capacity() >= 4);
        q.reserve(8);
        assert!(q.capacity() >= 8);
        q.shrink_to_fit();
    }
}
