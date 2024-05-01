/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // lson+1 = (curr+1) * 2
    // rson   = lson + 1

    // lson = 2*curr + 1
    // rson = 2*curr + 2

    // (lson+1)/2 - 1 = curr
    // (rson+1)/2 - 1 = curr
    //
    // parent = (curr+1)/2 - 1

    pub fn add(&mut self, value: T) {
        self.items.push(value);

        let mut curr = self.items.len() - 1;

        while curr != 0 {
            let parent = (curr + 1) / 2 - 1;

            if !(self.comparator)(&self.items[parent], &self.items[curr]) {
                self.items.swap(curr, parent);
            } else {
                break;
            }

            curr = parent;
        }
    }

    pub fn pop_top(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }

        // Index of last element and length of items after pop.
        let n = self.items.len() - 1;

        self.items.swap(0, n);

        let result = self.items.pop();

        let mut curr = 0;
        while curr < n {
            let lson = (curr * 2) + 1;
            let rson = (curr * 2) + 2;

            if lson >= n {
                break;
            }

            // Better son might be the new parent.
            let better_son = if rson < n && !(self.comparator)(&self.items[lson], &self.items[rson])
            {
                rson
            } else {
                lson
            };

            if !(self.comparator)(&self.items[curr], &self.items[better_son]) {
                self.items.swap(curr, better_son);
                curr = better_son;
            } else {
                break;
            }
        }

        result
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop_top()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
