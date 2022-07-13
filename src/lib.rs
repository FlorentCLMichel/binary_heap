/// A binary max-heap structure
///
/// We use an array representation for the heap, implemented as a `Vec`.
///
/// The data type must implement the `PartialOrd` trait (needed to have a partial ordering between
/// values).
pub struct BinaryHeap<T: std::cmp::PartialOrd> {
    data: Vec<T>,      // vector to store the data
}

impl<T: std::cmp::PartialOrd> BinaryHeap<T> {

    /// Create a new empty `BinaryHeap`
    ///
    /// # Example 
    ///
    /// ```
    /// use binary_heap::BinaryHeap;
    ///
    /// let heap = BinaryHeap::<isize>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        BinaryHeap::<T> {
            data: Vec::<T>::new(),
        }
    }

    /// Get the size of the heap (number of elements)
    ///
    /// # Example 
    ///
    /// ```
    /// use binary_heap::BinaryHeap;
    ///
    /// let heap = BinaryHeap::<isize>::new();
    /// let size = heap.size();
    ///
    /// assert_eq!(0, size);
    /// ```
    #[inline]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Insert an element in the heap
    ///
    /// # Example
    ///
    /// ```
    /// use binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::<isize>::new();
    /// heap.insert(0);
    ///
    /// assert_eq!(1, heap.size());
    /// ```
    pub fn insert(&mut self, x: T) {

        // push `x` in the data array
        self.data.push(x);

        // ‘bubble up’ the new element to its correct position
        let mut current_pos: usize = self.data.len();
        let mut parent_pos: usize = current_pos >> 1;
        while current_pos > 1 // stop if the element is at the root of the heap
        {
            
            // if the new element is larger than that of the parent node, swap them
            // else, the element is already at the right position and we can stop
            if self.data[parent_pos-1] < self.data[current_pos-1] {
                self.data.swap(parent_pos-1, current_pos-1);
                
                // update the current position and parent position
                current_pos = parent_pos;
                parent_pos >>= 1;

            } else {
                break;
            }
        }
    }
}

impl<T: std::cmp::PartialOrd> std::default::Default for BinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn several_inserts_1() {
        let mut heap = BinaryHeap::<isize>::new();
        assert!(heap.data.is_empty());
        heap.insert(0);
        assert_eq!(vec![0], heap.data);
        heap.insert(1);
        assert_eq!(vec![1, 0], heap.data);
        heap.insert(2);
        assert_eq!(vec![2, 0, 1], heap.data);
        heap.insert(-2);
        assert_eq!(vec![2, 0, 1, -2], heap.data);
        heap.insert(-1);
        assert_eq!(vec![2, 0, 1, -2, -1], heap.data);
        heap.insert(2);
        assert_eq!(vec![2, 0, 2, -2, -1, 1], heap.data);
        heap.insert(0);
        assert_eq!(vec![2, 0, 2, -2, -1, 1, 0], heap.data);
    }
}
