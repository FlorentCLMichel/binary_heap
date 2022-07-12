/// A binary heap structure
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
}

impl<T: std::cmp::PartialOrd> std::default::Default for BinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
