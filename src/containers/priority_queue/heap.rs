use std::cmp;
use std::fmt;
#[derive(Debug)]
pub struct Heap<T: std::fmt::Debug + PartialOrd + Copy>(Vec<T>);

impl<T: std::fmt::Debug + PartialOrd + Copy> Heap<T> {
    pub fn new() -> Self {
        Heap(Vec::new())
    }
 
    fn parent(&self, idx: usize) -> Option<usize> {
        assert!(idx < self.0.len(), "`parent` out of index");
        if idx == 0 {
            None
        } else {
            Some((idx-1) / 2)
        }
    }   

    fn child(&self, idx: usize) -> Option<usize> {
        let child_idx = idx * 2 + 1;

        if child_idx >= self.0.len() {
            None
        } else {
            Some(child_idx)
        }
    }   

    pub fn insert(&mut self, value: T) {
        self.0.push(value);
        self.bubble(self.0.len()-1);
    }

    // Scan until I find value, delete, then heapify
    pub fn pop(&mut self) -> Option<T> {
        if self.0.len() == 0 {
            None
        } else {
            let popped = self.0[0];
            self.heapify();
            Some(popped)
        }
    }

    pub fn peek(&self) -> Option<T> {
        if self.0.len() == 0 {
            None
        } else {
            Some(self.0[0])
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()     
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    fn bubble(&mut self, idx: usize) { 
        let mut cur_idx = idx;
        while let Some(parent_idx) = self.parent(cur_idx) {
            if self.0[parent_idx] > self.0[cur_idx] {
                let temp = self.0[parent_idx];
                self.0[parent_idx] = self.0[cur_idx];
                self.0[cur_idx] = temp;
            }
            cur_idx = parent_idx;
        }
    }

    pub fn into_vec(self) -> Vec<T> { 
        self.0
    }

    fn heapify(&mut self) { 
        let mut cur_idx = 0;
        while let Some(child_idx) = self.child(cur_idx) {
            if child_idx == self.0.len() - 1 { 
                self.0[cur_idx] = self.0[child_idx];
                break;
            } else if self.0[child_idx] <= self.0[child_idx + 1] { 
                self.0[cur_idx] = self.0[child_idx];
                cur_idx = child_idx;
            } else {
                self.0[cur_idx] = self.0[child_idx + 1];
                cur_idx = child_idx + 1;
            }
        }
        self.0[cur_idx] = self.0[self.0.len()-1];
        self.0.pop();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn basics() { 
        let mut heap = Heap::new();
        heap.insert(1);
        assert_eq!(heap.peek(), Some(1));
        heap.insert(4);
        heap.insert(2);
        heap.insert(5);
        heap.insert(3);
        assert_eq!(heap.peek(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.peek(), Some(2));
        assert_eq!(heap.peek(), Some(2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    pub fn tie_list() { 
        let mut heap = Heap::new();
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.peek(), None);
        heap.insert(1);
        assert_eq!(heap.peek(), Some(1));
        heap.insert(1);
        heap.insert(1);
        heap.insert(1);
        assert_eq!(heap.peek(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.peek(), Some(1));
        assert_eq!(heap.peek(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.pop(), None);
    }

    pub fn large() { 
        let mut heap = Heap::new();
        heap.insert(1);
        assert_eq!(heap.peek(), Some(1));
        heap.insert(4);
        heap.insert(2);
        heap.insert(2);
        heap.insert(8);
        heap.insert(6);
        heap.insert(2);
        heap.insert(2);
        heap.insert(8);
        heap.insert(5);
        heap.insert(2);
        heap.insert(8);
        heap.insert(5);
        heap.insert(3);
        heap.insert(8);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), None);
    }
}

