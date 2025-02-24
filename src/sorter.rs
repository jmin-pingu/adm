use crate::containers::priority_queue::heap::Heap;

pub trait Sortable: Ord + Copy + std::fmt::Debug {}
impl<T: Ord + Copy + std::fmt::Debug> Sortable for T {}

pub trait Sorter<T: Sortable> {
    fn sort(&mut self);
}

pub struct HeapSorter<T: Sortable>(Vec<T>);
pub struct MergeSorter<T: Sortable>(Vec<T>);
pub struct QuickSorter<T: Sortable>(Vec<T>);

impl<T: Sortable> QuickSorter<T> {
    pub fn new(elements: Vec<T>) -> Self {
        QuickSorter(elements)
    }

    pub fn get(self) -> Vec<T> {
        self.0
    }
}

impl<T: Sortable> MergeSorter<T> {
    pub fn new(elements: Vec<T>) -> Self {
        MergeSorter(elements)
    }

    pub fn get(self) -> Vec<T> {
        self.0
    }

}

impl<T: Sortable> HeapSorter<T> {
    pub fn new(elements: Vec<T>) -> Self {
        HeapSorter(elements)
    }

    pub fn get(self) -> Vec<T> {
        self.0
    }
}

impl<T: Sortable> Sorter<T> for MergeSorter<T> {
    fn sort(&mut self) {
        fn merge<T: Ord + Copy + std::fmt::Debug >(left: &mut [T], right: &mut [T]) {
            let (mut queue1, mut queue2): (Heap<T>, Heap<T>) = (Heap::new(), Heap::new());
            left.iter().for_each(|item| queue1.insert(*item));
            right.iter().for_each(|item| queue2.insert(*item));
            let mut idx = 0;
            while !(queue1.is_empty() && queue2.is_empty()) {
                let val = match(queue1.peek(), queue2.peek()) {
                    (None, None) => { return; },
                    (None, Some(_)) =>  queue2.pop(),
                    (Some(_), None) => queue1.pop(),
                    (Some(l_val), Some(r_val)) => {
                        if l_val <= r_val {
                            queue1.pop()
                        } else {
                            queue2.pop()
                        }
                    },
                };
                assign(idx, val.expect("Option<T> should contain a value"), left, right);
                idx += 1;
            }
        }

        fn assign<T: Ord + Copy + std::fmt::Debug>(idx: usize, val: T, left: &mut [T], right: &mut [T]) {
            if idx >= left.len() {
                right[idx - left.len()] = val;
            } else {
                left[idx] = val;
            }
        }

        fn merge_sort<T: Ord + Copy + std::fmt::Debug>(left: &mut [T], right: &mut [T]) {
            if left.len() <= 1 && right.len() <= 1 { 
                merge(left, right);
                return ;
            }
            let (new_left, new_right) = left.split_at_mut(left.len()/ 2);
            merge_sort(new_left, new_right);
            let (new_left, new_right) = right.split_at_mut(right.len()/ 2);
            merge_sort(new_left, new_right);
            merge(left, right);
        }

        if self.0.len() <= 1 { return;}
        let middle = self.0.len()/2;
        let (left, right) = self.0.split_at_mut(middle);
        merge_sort(left, right);
    }
}

impl<T: Sortable> Sorter<T> for HeapSorter<T> {
    fn sort(&mut self) {
        let mut h: Heap<T> = Heap::new();
        let mut v = Vec::new();
        while let Some(elem) = self.0.pop() { h.insert(elem); }
        while let Some(elem) = h.pop() { v.push(elem); }
        self.0 = v;
    }
}

impl<T: Sortable> Sorter<T> for QuickSorter<T> {
    fn sort(&mut self) {
        fn partition<T: Sortable>(array: &mut [T]) -> (&mut [T], &mut [T]){
            let pivot = array[array.len()-1];
            let mut i = 0;
            for j in 1..array.len()-1 {
                if array[j] <= pivot {
                    array.swap(i, j);
                    i += 1;
                }
            }
            array.swap(array.len()-1, i);
            let (left, mut right) = array.split_at_mut(i);
            if right.len() != 0 {
                (_, right) = right.split_at_mut(1); 
            }
            (left, right)
        }

        fn quick_sort<T: Sortable>(array: &mut [T]) {
            if array.len() <= 1 {
                return;
            }
            let (left, right) = partition(array);
            quick_sort(left);
            quick_sort(right);
        }

        quick_sort(&mut self.0);
    }
}

#[cfg(test)] 
mod test {
    use super::*;
    #[test]
    fn heap_sort() {
        let mut sorter = HeapSorter::new(vec![0, 1, 2, 3, 4]);
        sorter.sort();
        sorter.0.iter().enumerate().for_each(|(idx, val)| assert_eq!(idx, *val));

        let mut sorter = HeapSorter::new(vec![4, 1, 3, 0, 2]);
        sorter.sort();
        sorter.0.iter().enumerate().for_each(|(idx, val)| assert_eq!(idx, *val));

        let mut sorter = HeapSorter::new(vec![0, 0, 0, 0, 0]);
        sorter.sort();
        sorter.0.iter().for_each(|val| assert_eq!(0, *val));
    }
 
    #[test]
    fn merge_sort() {
        let mut sorter = MergeSorter::new(vec![1, 0]);
        sorter.sort();
        sorter.0.iter().enumerate().for_each(|(idx, val)| assert_eq!(idx, *val));

        let mut sorter = MergeSorter::new(vec![4, 1, 3, 0, 2]);
        sorter.sort();
        
        sorter.0.iter().enumerate().for_each(|(idx, val)| assert_eq!(idx, *val));

        let mut sorter = MergeSorter::new(vec![9, 4, 1, 8, 5, 3, 6, 0, 10, 11, 2, 7]);
        sorter.sort();
    }

    #[test]
    fn quick_sort() {
        let mut sorter = QuickSorter::new(vec![1, 0]);
        sorter.sort();
        sorter.0.iter().enumerate().for_each(|(idx, val)| assert_eq!(idx, *val));

        let mut sorter = QuickSorter::new(vec![4, 1, 3, 0, 2]);
        sorter.sort();
        
        sorter.0.iter().enumerate().for_each(|(idx, val)| assert_eq!(idx, *val));

        let mut sorter = QuickSorter::new(vec![9, 4, 1, 8, 5, 3, 6, 0, 10, 11, 2, 7]);
        sorter.sort();
    }
}

