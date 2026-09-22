//! # `unique_deque`
//!
//! `unique_deque` provides two [`VecDeque`]-backed collections that keep
//! themselves free of duplicates while retaining deque semantics: random
//! indexing plus `push`/`pop` at both ends.
//!
//! - [`UniqueDeque<T>`] stores `T` values and guarantees that no value appears
//!   more than once. Pushing a value that is already present is a no-op; popping
//!   a value frees it, so it may be pushed again.
//! - [`KeyDeque<K, T>`] stores `(K, T)` pairs keyed by a unique `K`. Pushing with
//!   a key that already exists overwrites the existing entry *at its current
//!   position* and returns the displaced `(K, T)`.
//!
//! Both pair a `VecDeque` with a [`HashSet`] for O(1) membership checks, so the
//! uniqueness invariant is cheap to maintain on every push and pop.
//!
//! [`VecDeque`]: std::collections::VecDeque
//! [`HashSet`]: std::collections::HashSet

mod key_deque;
mod unique_deque;

pub use key_deque::KeyDeque;
pub use unique_deque::UniqueDeque;
