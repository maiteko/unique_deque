use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque, vec_deque::Iter},
};

/// A [`VecDeque`] that holds no duplicate values.
///
/// `UniqueDeque` keeps its elements in a `VecDeque` and mirrors them into a
/// [`HashSet`], so membership checks are O(1) and the "no duplicates" invariant
/// is maintained on every push and pop. Pushing a value that is already present
/// is ignored; popping a value frees it, so it may be pushed again.
///
/// Because each value must live in both the deque and the set, the methods that
/// own a `T` value require `T: Clone`.
///
/// # Examples
///
/// ```
/// use unique_deque::UniqueDeque;
///
/// let mut q: UniqueDeque<i32> = UniqueDeque::new();
/// q.push_back(1);
/// q.push_back(2);
/// q.push_back(1); // ignored: `1` is already present
/// q.push_front(3);
///
/// assert_eq!(q.get(0), Some(&3));
/// assert_eq!(q.get(1), Some(&1));
/// assert_eq!(q.get(2), Some(&2));
/// assert_eq!(q.get(3), None);
///
/// // Popping a value frees it, so it can be pushed again.
/// assert_eq!(q.pop_front(), Some(3));
/// q.push_front(3);
/// assert_eq!(q.get(0), Some(&3));
/// ```
pub struct UniqueDeque<T: Eq + std::hash::Hash> {
    deque: VecDeque<T>,
    set: HashSet<T>,
}

impl<T: Eq + std::hash::Hash + Clone> UniqueDeque<T> {
    /// Creates a new, empty `UniqueDeque`.
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
            set: HashSet::new(),
        }
    }

    /// Creates a new, empty `UniqueDeque` with at least `capacity` reserved for
    /// both the deque and the backing set.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(capacity),
            set: HashSet::with_capacity(capacity),
        }
    }

    /// Returns a reference to the element at `index`, or `None` if out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.deque.get(index)
    }

    /// Swaps the elements at positions `i` and `j`.
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    pub fn swap(&mut self, i: usize, j: usize) {
        self.deque.swap(i, j)
    }

    /// Returns the smaller of the underlying deque and set capacities.
    pub fn capacity(&self) -> usize {
        self.deque.capacity().min(self.set.capacity())
    }

    /// Reserves capacity for at least `additional` more elements in both the
    /// deque and the backing set.
    pub fn reserve(&mut self, additional: usize) {
        self.deque.reserve_exact(additional);
        self.set.reserve(additional);
    }

    /// Shrinks the internal buffers to fit the current number of elements.
    pub fn shrink_to_fit(&mut self) {
        self.deque.shrink_to_fit();
        self.set.shrink_to_fit();
    }

    /// Pushes `item` to the front of the deque, unless it is already present, in
    /// which case the push is ignored.
    pub fn push_front(&mut self, item: T) {
        if self.set.contains(&item) {
            return;
        }

        self.set.insert(item.clone());
        self.deque.push_front(item);
    }

    /// Pushes `item` to the back of the deque, unless it is already present, in
    /// which case the push is ignored.
    pub fn push_back(&mut self, item: T) {
        if self.set.contains(&item) {
            return;
        }

        self.set.insert(item.clone());
        self.deque.push_back(item);
    }

    /// Removes and returns the front element, if any, and removes it from the
    /// uniqueness set so it may be pushed again.
    pub fn pop_front(&mut self) -> Option<T> {
        let item = self.deque.pop_front()?;
        self.set.remove(&item);
        Some(item)
    }

    /// Removes and returns the back element, if any, and removes it from the
    /// uniqueness set so it may be pushed again.
    pub fn pop_back(&mut self) -> Option<T> {
        let item = self.deque.pop_back()?;
        self.set.remove(&item);
        Some(item)
    }

    /// Sorts the elements in place using `compare`. Sorting never introduces
    /// duplicates, so the uniqueness invariant is preserved.
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let s = self.deque.make_contiguous();
        s.sort_by(compare);
    }

    /// Sorts the elements in place using their [`Ord`] ordering.
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.sort_by(|a, b| a.cmp(b));
    }

    /// Returns `true` if the UniqueDeque contains no elements
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Returns `true` if the UniqueDeque contains `item`
    pub fn contains(&self, item: &T) -> bool {
        self.set.contains(item)
    }

    /// Returns the number of elements in the UniqueDeque
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Returns a front-to-back iterator
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        self.deque.iter()
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

#[cfg(test)]
mod tests {
    use super::UniqueDeque;

    #[test]
    fn push_back_and_front_ordering() {
        let mut q = UniqueDeque::<i32>::new();
        q.push_back(1);
        q.push_back(2);
        q.push_front(3);
        assert_eq!(q.get(0), Some(&3));
        assert_eq!(q.get(1), Some(&1));
        assert_eq!(q.get(2), Some(&2));
        assert_eq!(q.get(3), None);
    }

    #[test]
    fn duplicate_push_is_ignored() {
        let mut q = UniqueDeque::<i32>::new();
        q.push_back(1);
        q.push_back(1);
        q.push_front(1);
        assert_eq!(q.get(0), Some(&1));
        assert_eq!(q.get(1), None);
    }

    #[test]
    fn popped_value_can_be_pushed_again() {
        let mut q = UniqueDeque::<i32>::new();
        q.push_back(1);
        q.push_front(2);
        assert_eq!(q.pop_front(), Some(2));
        // `2` was removed from the set, so it may be pushed again.
        q.push_front(2);
        assert_eq!(q.get(0), Some(&2));
        assert_eq!(q.get(1), Some(&1));
    }

    #[test]
    fn pop_front_and_back() {
        let mut q = UniqueDeque::<i32>::new();
        q.push_back(1);
        q.push_back(2);
        assert_eq!(q.pop_back(), Some(2));
        assert_eq!(q.pop_front(), Some(1));
        assert_eq!(q.pop_back(), None);
        assert_eq!(q.pop_front(), None);
    }

    #[test]
    fn sort_ascending() {
        let mut q = UniqueDeque::<i32>::new();
        q.push_back(3);
        q.push_back(1);
        q.push_back(2);
        q.sort();
        assert_eq!(q.get(0), Some(&1));
        assert_eq!(q.get(1), Some(&2));
        assert_eq!(q.get(2), Some(&3));
    }

    #[test]
    fn sort_by_descending() {
        let mut q = UniqueDeque::<i32>::new();
        q.push_back(1);
        q.push_back(3);
        q.push_back(2);
        q.sort_by(|a, b| b.cmp(a));
        assert_eq!(q.get(0), Some(&3));
        assert_eq!(q.get(1), Some(&2));
        assert_eq!(q.get(2), Some(&1));
    }

    #[test]
    fn swap_exchanges_positions() {
        let mut q = UniqueDeque::<i32>::new();
        q.push_back(1);
        q.push_back(2);
        q.push_back(3);
        q.swap(0, 2);
        assert_eq!(q.get(0), Some(&3));
        assert_eq!(q.get(1), Some(&2));
        assert_eq!(q.get(2), Some(&1));
    }

    #[test]
    fn with_capacity_and_reserve() {
        let mut q = UniqueDeque::<i32>::with_capacity(4);
        assert!(q.capacity() >= 4);
        q.reserve(8);
        assert!(q.capacity() >= 8);
        q.shrink_to_fit();
    }

    #[test]
    fn unique_strings() {
        let mut q = UniqueDeque::<&str>::new();
        q.push_back("b");
        q.push_back("a");
        q.push_back("b"); // ignored: duplicate
        q.sort();
        assert_eq!(q.get(0), Some(&"a"));
        assert_eq!(q.get(1), Some(&"b"));
        assert_eq!(q.get(2), None);
    }
}
