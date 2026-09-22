# unique_deque

Two `VecDeque`-backed collections that keep themselves free of duplicates while
retaining deque semantics — random indexing plus `push`/`pop` at both ends.

- **`UniqueDeque<T>`** stores `T` values and guarantees no value appears more
  than once. Pushing a value that is already present is a no-op; popping a value
  frees it, so it can be pushed again.
- **`KeyDeque<K, T>`** stores `(K, T)` pairs keyed by a unique `K`. Pushing with
  a key that already exists overwrites the existing entry *in place* and returns
  the displaced `(K, T)`.

Both pair a `VecDeque` with a `HashSet` for O(1) membership checks, so the
uniqueness invariant is cheap to maintain on every push and pop.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
unique_deque = "0.1"
```

## Usage

### `UniqueDeque`

```rust
use unique_deque::UniqueDeque;

let mut q = UniqueDeque::<i32>::new();
q.push_back(1);
q.push_back(2);
q.push_back(1); // ignored: already present
q.push_front(3);

assert_eq!(q.get(0), Some(&3));
assert_eq!(q.get(1), Some(&1));
assert_eq!(q.get(2), Some(&2));

// Popping frees the value, so it may be pushed again.
assert_eq!(q.pop_front(), Some(3));
q.push_front(3);
```

### `KeyDeque`

```rust
use unique_deque::KeyDeque;

let mut q = KeyDeque::<usize, &str>::new();
q.push_back(1, "a");
q.push_back(2, "b");

// Re-pushing key `1` overwrites in place and returns the old pair.
assert_eq!(q.push_back(1, "c"), Some((1, "a")));
assert_eq!(q.get(0), Some(&(1, "c")));
assert_eq!(q.get(1), Some(&(2, "b")));
```

## API

### `UniqueDeque<T: Eq + Hash + Clone>`

| Method | Description |
| --- | --- |
| `new()` / `with_capacity(n)` | Create an empty deque, optionally pre-sized. |
| `get(i)` | Element at `i`, or `None`. |
| `swap(i, j)` | Swap the elements at `i` and `j` (panics if out of bounds). |
| `capacity()` | Smaller of the deque and set capacities. |
| `reserve(n)` | Reserve capacity for `n` more elements in both buffers. |
| `shrink_to_fit()` | Shrink the internal buffers to the current size. |
| `push_front(x)` / `push_back(x)` | Push `x` unless it is already present. |
| `pop_front()` / `pop_back()` | Remove and return an end element, freeing it for reuse. |
| `sort()` / `sort_by(f)` | Sort in place (`sort()` requires `T: Ord`). |

Because each value lives in both the deque and the backing set, the methods that
own a `T` require `T: Clone`.

### `KeyDeque<K: Eq + Hash + Copy, T>`

| Method | Description |
| --- | --- |
| `new()` / `with_capacity(n)` | Create an empty deque, optionally pre-sized. |
| `get(i)` | The `(K, T)` pair at `i`, or `None`. |
| `swap(i, j)` | Swap the entries at `i` and `j` (panics if out of bounds). |
| `capacity()` | Underlying deque capacity. |
| `reserve(n)` | Reserve capacity, growing the key set to match. |
| `shrink_to_fit()` | Shrink the internal buffers to the current size. |
| `push_front(k, v)` / `push_back(k, v)` | Push `(k, v)`; if `k` exists, overwrite in place and return the old pair. |
| `pop_front()` / `pop_back()` | Remove and return an end pair, freeing its key. |
| `sort_by(f)` / `sort_by_keys()` / `sort_by_values()` | Sort in place. |

## Performance

Detecting a key collision on push is O(1); locating the slot to overwrite on
`KeyDeque` is O(n). `UniqueDeque` push and pop are O(1) amortized.

## Testing

```sh
cargo test
```

This runs the unit tests in `src/` as well as the doctests embedded in the
documentation.
