# A simple implementation of a binary heap in Rust

This crate implements a simple binary heap from scratch. The aim is to try to keep the implementation easy to understand, even when it implies not using the most efficient algorithms or representations. For a more efficient version, use [`std::collections::BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html).

## How to use

1. Clone this repository. In the following, `PATH_BH` denotes the path to the cloned repository.
2. In a Rust project, add `binary_heap = { path = "PATH_BH" }` to the `[dependencies]` section of your `Cargo.toml` file.
3. Add `use binary_heap::BinaryHeap;` at the top of each source file where this structure is used.

## The `BinaryHeap` structure

This crate provides the `BinaryHeap` structure, which (as you have probably guessed) is a max binary heap. This structure may be seen as a binary tree which each node holding an element from a partially ordered set satisfying these two properties: 

* If a node contains data $x$ and one of its descendants contains data $y$, then $y > x$ is false. 
* At most one level of the tree is incomplete, and this level is filled from left to right. 

The `BinaryHeap` structure has a type parameter, `T`, which must implement the `std::cmp::PartialOrd` trait (so that the above inequality makes sense).

In the following, $n$ denotes the number of elements in the heap, all asymptotic complexities are understood to hold in the limit $n \to \infty$ ,and `T` is an arbitrary type implementing the `std::cmp::PartialOrd` trait.

### Main functions

The following functions are implemented for `BinaryHeap<T>`: 

| Function  | Condition on `T` | Arguments | Effect | Worst-case asymptotic complexity |
|-----------|------------------|-----------|--------|----------------------------------|
| `new`     | None             | None      | Return a new `BinaryHeap<T>`. | $\Theta(1)$ |
| `size`    | None             | None      | Return the number of elements in the heap (`usize`). | $\Theta(1)$ |
| `insert`  | None             | `x: T`    | Insert `x` in the heap. | $\Theta(\log n)$ |
| `pop`     | None             | None      | Remove the root element $x$ of the heap if it exists and return `Some(x)`; if the heap is empty, return `None`. | $\Theta(\log n)$ |
| `to_vec`  | None             | None      | Consume the heap and return a vector of all its elements in non-increasing order: if elements $x$ and $y$ are at indices $i$ and $j$ with $j > i$, then $y > x$ is false. If the type `T` implements `std::cmp::Ord`, then `to_vec` is ordered (from maximum to minimum). | $\Theta(n \log n)$ |
| `get_max` | `Clone`          | None      | Return a copy of the element at the root of the heap. | $\Theta(1)$ |
| `search`  | `PartialEq`      | `x: &T`   | Return `true` if the heap contains at least one element `y` such that `*x == y` is `true` or `false` otherwise. | $\Theta(n)$ |

### `Default` trait

The type `BinaryHeap<T>` implements the `Default` trait; the default value is an empty heap.

### `Iterator` trait

The type `BinaryHeap<T>` implements the `Iterator` trait. Elements of the heap are returned in non-increasing order: when going through the iterator, if an element $x$ comes before another element $y$, then $y > x$ is false. Getting the next element in the iterator has worst-case complexity $\Theta(\log n)$.
