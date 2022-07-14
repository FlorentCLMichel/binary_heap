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
    /// Worst-case complexity: $O(1)$.
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
    /// Worst-case complexity: $O(\log n)$, where $n$ is the number of elements in the heap.
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

    /// remove and return the maximum (or `None` if the heap is empty)
    ///
    /// Worst-case complexity: $O(\log n)$, where $n$ is the number of elements in the heap.
    ///
    /// # Return value
    ///
    /// * `Some(x)` where `x` is the maximum value of the heap is not empty
    /// * `None` if the heap is empty
    ///
    /// # Example
    ///
    /// ```
    /// use binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::<isize>::new();
    /// heap.insert(0);
    /// let max = heap.pop();
    ///
    /// assert_eq!(Some(0), max);
    /// assert_eq!(0, heap.size());
    /// ```
    pub fn pop(&mut self) -> Option<T> {

        // if the heap is empty, return `None`
        if self.data.is_empty() {
            return None;
        }

        let size = self.size();

        // exchange the root with the last element
        self.data.swap(0, size-1);

        // bubble down the root
        let mut current_pos: usize = 0;
        let mut pos_left_child = 1;
        let mut pos_right_child = 2;
        while pos_right_child + 1 < size // stop if the second children is the last element
        {
            let left_child_larger = self.data[pos_left_child] > self.data[current_pos];
            let right_child_larger = self.data[pos_right_child] > self.data[current_pos];
            if (left_child_larger || right_child_larger) // if the right children is larger
                && self.data[pos_left_child] < self.data[pos_right_child]
            {
                self.data.swap(current_pos, pos_right_child);
                current_pos = pos_right_child;
            } else if left_child_larger {              // if the left children is larger
                self.data.swap(current_pos, pos_left_child);
                current_pos = pos_left_child;
            } else {                                   // if no children is larger, stop
                break;
            }
            pos_left_child = (current_pos << 1) + 1;
            pos_right_child = (current_pos << 1) + 2;
        }

        // last swap if needed
        if (pos_left_child + 1 < size)
            && (self.data[pos_left_child] > self.data[current_pos])
        {
            self.data.swap(current_pos, pos_left_child);
        }

        // return the last element
        self.data.pop()
    }

    /// consume the heap and return a vectror fo all its elements
    /// # Example
    ///
    /// ```
    /// use binary_heap::BinaryHeap;
    /// 
    /// // build the heap
    /// let mut heap = BinaryHeap::<isize>::new();
    /// heap.insert(0);
    /// heap.insert(3);
    /// heap.insert(1);
    /// heap.insert(2);
    /// heap.insert(-1);
    ///
    /// // convert it to a vector
    /// let vec = heap.to_vec();
    ///
    /// assert_eq!(vec![3, 2, 1, 0, -1], vec);
    /// ```
    #[inline]
    pub fn to_vec(self) -> Vec<T> {
        // // explicit version
        // let mut res = Vec::<T>::with_capacity(self.size());
        // while let Some(x) = self.pop() {
        //     res.push(x);
        // }
        // res
        
        // version using the implementation of the `Iterator` trait
        self.collect()
    }
}

impl<T: std::cmp::PartialOrd + Clone> BinaryHeap<T> {

    /// return a copy of the maximum element if the heap is not empty
    ///
    /// Worst-case complexity: $O(1)$.
    ///
    /// # Example
    ///
    /// ```
    /// use binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::<isize>::new();
    /// heap.insert(0);
    ///
    /// assert_eq!(Some(0), heap.get_max());
    /// ```
    pub fn get_max(&self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data[0].clone())
        }
    }
}

impl<T: std::cmp::PartialOrd + std::cmp::PartialEq> BinaryHeap<T> {

    /// Search an element `x` in the heap, returning `true` if it is present and `false` if it is
    /// not.
    ///
    /// Worst-case complexity: $O(n)$, where $n$ is the number of elements in the heap.
    ///
    /// # Example
    /// ```
    /// use binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::<isize>::new();
    /// heap.insert(0);
    ///
    /// assert!(heap.search(&0));
    /// assert!(!heap.search(&1));
    /// ```
    pub fn search(&self, x: &T) -> bool {

        // queue storing the indices of elements to process
        let mut index_queue = std::collections::VecDeque::<usize>::new();

        // add the root to the queue
        index_queue.push_front(0);

        let size = self.size();

        // process the whole queue
        while let Some(current_index) = index_queue.pop_back() {

            // If the index is not smaller than `size`, we hav ereached the end of the heap.
            // If `x` is larger than the elementwith the current index, we know `x` can't be in 
            // the sub-heap.
            if (current_index < size) && !(*x > self.data[current_index]) {

                // check if the current element is equal to `x`; if yes, return `true`
                if *x == self.data[current_index] {
                    return true;
                }

                // push the indices of the two children to the queue
                index_queue.push_front((current_index << 1) + 1);
                index_queue.push_front((current_index << 1) + 2);
            }
        }

        false
    }
}

impl<T: std::cmp::PartialOrd> std::default::Default for BinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::cmp::PartialOrd> Iterator for BinaryHeap<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
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
    
    #[test]
    fn get_max_1() {
        let mut heap = BinaryHeap::<isize>::new();
        assert_eq!(None, heap.get_max());
        heap.insert(0);
        assert_eq!(Some(0), heap.get_max());
        heap.insert(1);
        assert_eq!(Some(1), heap.get_max());
        heap.insert(2);
        assert_eq!(Some(2), heap.get_max());
        heap.insert(-2);
        assert_eq!(Some(2), heap.get_max());
        heap.insert(-1);
        assert_eq!(Some(2), heap.get_max());
        heap.insert(2);
        assert_eq!(Some(2), heap.get_max());
        heap.insert(0);
        assert_eq!(Some(2), heap.get_max());
    }
    
    #[test]
    fn pop_1() {
        let mut heap = BinaryHeap::<isize>::new();
        heap.insert(0);
        heap.insert(1);
        heap.insert(2);
        heap.insert(-2);
        heap.insert(-1);
        heap.insert(2);
        heap.insert(0);
        assert_eq!(Some(2), heap.pop());
        assert_eq!(Some(2), heap.pop());
        assert_eq!(Some(1), heap.pop());
        assert_eq!(Some(0), heap.pop());
        assert_eq!(Some(0), heap.pop());
        assert_eq!(Some(-1), heap.pop());
        assert_eq!(Some(-2), heap.pop());
        assert_eq!(None, heap.pop());
    }
    
    #[test]
    fn pop_2() {
        let mut heap = BinaryHeap::<isize>::new();
        heap.insert(0);
        heap.insert(10);
        heap.insert(20);
        heap.insert(-20);
        heap.insert(-10);
        heap.insert(20);
        heap.insert(0);
        heap.insert(5);
        heap.insert(15);
        heap.insert(2);
        heap.insert(3);
        heap.insert(-3);
        heap.insert(-15);
        assert_eq!(Some(20), heap.pop());
        assert_eq!(Some(20), heap.pop());
        assert_eq!(Some(15), heap.pop());
        assert_eq!(Some(10), heap.pop());
        assert_eq!(Some(5), heap.pop());
        assert_eq!(Some(3), heap.pop());
        assert_eq!(Some(2), heap.pop());
        assert_eq!(Some(0), heap.pop());
        assert_eq!(Some(0), heap.pop());
        assert_eq!(Some(-3), heap.pop());
        assert_eq!(Some(-10), heap.pop());
        assert_eq!(Some(-15), heap.pop());
        assert_eq!(Some(-20), heap.pop());
        assert_eq!(None, heap.pop());
    }
    
    #[test]
    fn search_1() {
        let mut heap = BinaryHeap::<isize>::new();
        heap.insert(0);
        heap.insert(1);
        heap.insert(2);
        heap.insert(-2);
        heap.insert(-1);
        heap.insert(2);
        heap.insert(0);
        assert!(heap.search(&0));
        assert!(heap.search(&1));
        assert!(heap.search(&2));
        assert!(heap.search(&-1));
        assert!(heap.search(&-2));
        assert!(!heap.search(&-3));
        assert!(!heap.search(&3));
    }
    
    #[test]
    fn search_2() {
        let mut heap = BinaryHeap::<isize>::new();
        heap.insert(0);
        heap.insert(10);
        heap.insert(20);
        heap.insert(-20);
        heap.insert(-10);
        heap.insert(20);
        heap.insert(0);
        heap.insert(5);
        heap.insert(15);
        heap.insert(2);
        heap.insert(3);
        heap.insert(-3);
        heap.insert(-15);
        assert!(heap.search(&0));
        assert!(heap.search(&10));
        assert!(heap.search(&20));
        assert!(heap.search(&-20));
        assert!(heap.search(&-10));
        assert!(heap.search(&5));
        assert!(heap.search(&15));
        assert!(heap.search(&2));
        assert!(heap.search(&3));
        assert!(heap.search(&-3));
        assert!(heap.search(&-15));
        assert!(!heap.search(&1));
        assert!(!heap.search(&-1));
        assert!(!heap.search(&100));
        assert!(!heap.search(&-100));
        assert!(!heap.search(&14));
    }
}
