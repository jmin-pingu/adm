use std::fmt::Display;
use std::cmp;
use std::mem;

type Downlink<T> = Option<Box<Node<T>>>;

struct Node<T: PartialOrd + Display> {
    item: T,
    left: Downlink<T>,
    right: Downlink<T>,
}

impl<T: PartialOrd + Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => write!(f, "({}: {}, {})", self.item, left, right),
            (None, Some(right)) => write!(f, "({}: None, {})", self.item, right),
            (Some(left), None) => write!(f, "({}: {}, None)", self.item, left),
            (None, None) => write!(f, "[{}]", self.item),
        } 
    }
}

impl<T: PartialOrd + Display> Node<T> {
    fn is_leaf(&self) -> bool {
        match (self.left.as_deref(), self.right.as_deref()) {
            (None, None) => false,
            _ => true,
        }
    }

    fn is_full(&self) -> bool {
        match (self.left.as_deref(), self.right.as_deref()) {
            (Some(_), Some(_)) => true,
            _ => false,
        }
    }
 
    fn max_height(&self) -> usize {
        match (self.left.as_deref(), self.right.as_deref()) {
            (None, None) => 0,
            (Some(left), None) => 1 + left.max_height(),
            (None, Some(right)) => 1 + right.max_height(),
            (Some(left), Some(right)) => 1 + cmp::max(left.max_height(), right.max_height()),
        }
    }

    fn min_height(&self) -> usize {
        match (self.left.as_deref(), self.right.as_deref()) {
            (None, None) => 0,
            (Some(_), None) => 0,
            (None, Some(_)) => 0,
            (Some(left), Some(right)) => 1 + cmp::min(left.min_height(), right.min_height()),
        }
    }
}

pub struct Bst<T: PartialOrd + Display> {
    root: Downlink<T>,
}

impl<T: PartialOrd + Display> Display for Bst<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = &self.root {
            write!(f, "{}", root)
        } else {
            write!(f, "{}", "None")
        }
    }
}


impl<T: PartialOrd + Display + Clone> Bst<T> {
    pub fn new() -> Self {
        Bst { root: None }
    }

    pub fn insert(&mut self, item: T) 
    {
        let mut cur_link = &mut self.root;
        loop {
            match cur_link {
                None => {
                    *cur_link = Some(Box::new(Node{item, left: None, right: None}));
                    return;
                },
                Some(node) => {
                    if node.item < item {
                        cur_link = &mut node.right;
                    } else if node.item > item {
                        cur_link = &mut node.left;
                    } else {
                        return;
                    }
                },
            }
        }
    }

    // NOTE: my old version of insert, understand the issue
    // pub fn insert(&mut self, item: T) {
    //     let cur_link = &mut self.root;
    //     loop {
    //         match cur_link {
    //             None => {
    //                 *cur_link = Some(Box::new(Node{item, left: None, right: None}));
    //                 break;
    //             },
    //             Some(node) if node.item < item => {
    //                 *cur_link = node.right.take();
    //             },
    //             Some(node) if node.item > item => {
    //                 *cur_link = node.left.take();
    //             },
    //             _ => { break; }
    //         }
    //     }
    // }
    // The first iteration (insert) would replace root with a new Downlink<item>
    // Any iteration after the initial state would trigger the second/third prong,
    // and then we would redefine cur_links REFERENCED DATA to None (since `right`/`left`)
    // Therefore, we would loop one more time, hit the first match arm and set the new item
    // The issue was not ownership (with &mut) but rather incorrect logic 
    //
    // NOTE: what I needed to do was for the steps where I want to "change the pointer", 
    // I should not have referenced cur_link but instead kept it as cur_link so that I could
    // change the pointer to either left or right

    pub fn delete(&mut self, item: T) -> Option<T> {
        let mut cur_link = &mut self.root;
        loop {
            match cur_link {
                Some(node) if node.item == item => {
                    let popped = node.item.clone();
                    *cur_link = match (node.left.take(), node.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => Some(left),
                        (Some(left), Some(right)) => {
                            Some(Box::new(Node{item: left.item, left: None, right: Some(right)}))
                        },
                        (None, Some(right)) => Some(right),
                        _ => { panic!("All values of `left` and `right` should have been exhausted") },
                    };
                    return Some(popped);
                }, 
                Some(node) => {
                    cur_link = if node.item < item {
                        &mut node.right
                    } else if node.item > item { 
                        &mut node.left
                    } else {
                        panic!("Already should have checked node.item == item case");
                    }
                },
                None => {
                    return None;
                },
            }
        }
    }

    pub fn contains(&self, item: T) -> bool 
    {
        let mut cur_link = &self.root;
        loop {
            match cur_link {
                Some(node) if node.item < item => {
                    cur_link = &node.right;
                },
                Some(node) if node.item > item => {
                    cur_link = &node.left;
                },
                Some(node) if node.item == item => return true,
                None => return false,
                _ => return false,
            }
        }
    }

    fn into_linked_list(&mut self) { 
        let mut cur_link = &mut self.root;

        while let Some(cur_node) = cur_link {
            let left_link = cur_node.left.take();
            if left_link.is_none() {
                cur_link = if let Some(node) = cur_link.as_deref_mut() { 
                    &mut node.right
                } else {
                    break;
                }
            } else {
                let mut right_tail = cur_link.take();
                let mut left_tail = left_link;

                right_tail.as_deref_mut().map(|node| {
                    node.left = if let Some(left_node) = left_tail.as_deref_mut() {
                        left_node.right.take()
                    } else { 
                        None 
                    };
                    node
                });

                left_tail.as_deref_mut().map(|node| {
                    node.right = right_tail;               
                    node
                });
                *cur_link = left_tail;
            }
        }
    }

    // NOTE: How do you convert the recursive approach in place
    pub fn rebalance(&mut self) { }
 
    pub fn max_height(&self) -> usize { 
        match &self.root {
            Some(node) => 1 + node.max_height(),
            None => 0 
        }
    }

    pub fn min_height(&self) -> usize { 
        match &self.root {
            Some(node) => 1 + node.min_height(),
            None => 0 
        }
    }

    pub fn is_balanced(&self) -> bool { 
        match &self.root {
            Some(node) => node.max_height() - node.min_height() <=  1,
            None => true 
        }
    }

    pub fn min(&self) -> Option<&T> {
        let mut cur_link = &self.root; 
        loop {
            match cur_link {
                Some(node) if node.left.is_some() => { cur_link = &node.left; },
                Some(node) if !node.left.is_some() => { return Some(&node.item) },
                None => { return None }
                _ => panic!("Should never reach"),
            }
        }
    }

    pub fn max(&self) -> Option<&T> {
        let mut cur_link = &self.root; 
        loop {
            match cur_link {
                Some(node) if node.right.is_some() => { cur_link = &node.right; },
                Some(node) if !node.right.is_some() => { return Some(&node.item) },
                None => { return None }
                _ => panic!("Should never reach"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut bst: Bst<i32> = Bst::new();
        bst.insert(3);
        bst.insert(1);
        bst.insert(7);
        bst.insert(2);
        bst.insert(-2);
        println!("{}", bst);
        assert_eq!(bst.contains(1), true);
        assert_eq!(bst.contains(2), true);
        assert_eq!(bst.contains(100), false);
    }

    #[test]
    fn delete() {
        let mut bst: Bst<i32> = Bst::new();
        bst.insert(3);
        bst.insert(1);
        bst.insert(7);
        bst.insert(2);
        bst.insert(-2);
        assert_eq!(bst.delete(2), Some(2));
        assert_eq!(bst.delete(2), None);
        assert_eq!(bst.delete(7), Some(7));
        assert_eq!(bst.contains(7), false);
        bst.delete(1);
        assert_eq!(bst.contains(1), false);
        assert_eq!(bst.contains(3), true);
    }

    #[test]
    fn minmax() {
        let mut bst: Bst<i32> = Bst::new();
        bst.insert(3);
        bst.insert(1);
        bst.insert(7);
        bst.insert(2);
        bst.insert(-2);
        assert_eq!(bst.max(), Some(&7));
        assert_eq!(bst.min(), Some(&-2));
        assert_eq!(bst.delete(2), Some(2));
        assert_eq!(bst.contains(2), false);
        bst.delete(7);
        assert_eq!(bst.contains(7), false);
        assert_eq!(bst.max(), Some(&3));
        bst.delete(1);
        assert_eq!(bst.contains(1), false);
        assert_eq!(bst.contains(3), true);
        assert_eq!(bst.min(), Some(&-2));
    }

    #[test]
    fn height() {
        let mut bst: Bst<i32> = Bst::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        assert_eq!(bst.max_height(), 5);
        assert_eq!(bst.min_height(), 1);
        assert_eq!(bst.delete(2), Some(2));
        assert_eq!(bst.max_height(), 4);
        assert_eq!(bst.is_balanced(), false);

        let mut bst1: Bst<i32> = Bst::new();
        assert_eq!(bst1.max_height(), 0);
        bst1.insert(3);
        assert_eq!(bst1.max_height(), 1);
        bst1.insert(2);
        bst1.insert(1);
        assert_eq!(bst1.max_height(), 3);
        bst1.insert(4);
        bst1.insert(5);
        assert_eq!(bst1.max_height(), 3);
        assert_eq!(bst1.is_balanced(), true);

        let mut bst2: Bst<i32> = Bst::new();
        bst2.insert(3);
        bst2.insert(2);
        bst2.insert(5);
        bst2.insert(4);
        bst2.insert(6);
        assert_eq!(bst2.max_height(), 3);
        assert_eq!(bst2.min_height(), 2);
        assert_eq!(bst2.is_balanced(), true);
        bst2.insert(7);
        assert_eq!(bst2.max_height(), 4);
        assert_eq!(bst2.min_height(), 2);
        assert_eq!(bst2.is_balanced(), false);
    }

    #[test]
    fn into_linked_list() {
        let mut bst: Bst<i32> = Bst::new();
        bst.insert(3);
        bst.insert(2);
        bst.insert(1);
        bst.insert(4);
        bst.insert(5);
        bst.into_linked_list();
        println!("{}", bst);
        assert_eq!(bst.max_height(), 5);
    }
}
