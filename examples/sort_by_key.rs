//! This example shows how to use a binary heap to sort (key, value) pairs according to their keys. 

use binary_heap::BinaryHeap;

// A (key, value) structure
#[derive(PartialEq, Clone)]
struct KeyValuePair<K: PartialOrd + Clone, V: PartialEq + Clone> {
    key: K, 
    value: V,
}

impl<K: PartialOrd + Clone, V: PartialEq + Clone> KeyValuePair<K, V> {
    pub fn new(key: K, value: V) -> Self {
        KeyValuePair { key, value }
    }
}

impl<K: PartialOrd + Clone, V: PartialEq + Clone> PartialOrd for KeyValuePair<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}


fn main() {

    // List of tasks with level of priority (higher ↔ more urgent)
    let tasks = vec![
        KeyValuePair::new(4, "Clean the kitchen table"),
        KeyValuePair::new(2, "Read ‘The Lord of the Rings’"),
        KeyValuePair::new(8, "Proofread the new draft for the paper"),
        KeyValuePair::new(6, "Update documentation in the Git repository"),
    ];
    
    // Sort them using a binary heap
    let tasks_sorted = BinaryHeap::sort(&tasks);

    // Print the result
    for KeyValuePair { key: k, value: v } in tasks_sorted.iter() {
        println!("Task: {}, priority level: {}", v, k);
    }
}
