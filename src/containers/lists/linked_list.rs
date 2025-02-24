use std::mem;
pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T: std::cmp::PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList{
            head: None
        }
    }

    pub fn get(&self, idx: usize) -> &T {
        let mut counter: usize = 0;
        let mut cur_link = self.head.as_deref();
        while let Some(node) = cur_link {
            if counter == idx {
                return &node.item;
            }
            cur_link = node.next.as_deref();
            counter += 1;
        }
        panic!("out of index");
    }

    pub fn get_index(&self, item: T) -> Option<usize> {
        let mut counter: usize = 0;
        let mut cur_link = self.head.as_deref();
        while let Some(node) = cur_link {
            if node.item == item { return Some(counter); }
            cur_link = node.next.as_deref();
            counter += 1;
        }
        None
    }

    pub fn delete(&mut self, item: T) {
        let mut current_link = &mut self.head;
        loop {
            match current_link {
                None => {
                    break;
                },
                Some(node) if node.item == item => {
                    *current_link = node.next.take(); // new owner of the node
                },
                Some(node) => {
                    current_link = &mut node.next;
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            Some(node) => {
                self.head = node.next;
                Some(node.item)
            },
            None => None
        }
    }

    pub fn push(&mut self, item: T) {
        let tail = mem::replace(&mut self.head, None);
        self.head = Some(Box::new(Node { item, next: tail}));
    }

    pub fn insert(&mut self, idx: usize, item: T) {
        let mut current_link = &mut self.head;
        for _ in 0..idx {
            if let Some(node) = current_link {
                current_link = &mut node.next;
            }
        }
        *current_link = Some(Box::new(Node{
            item,
            next: current_link.take() 
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() { 
        let mut l = LinkedList::new();
        assert_eq!(l.pop(), None);
        l.push(0);
        l.push(1);
        l.push(2);
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), Some(0));
        assert_eq!(l.pop(), None);
    }

    #[test]
    fn indexing() { 
        let mut l = LinkedList::new();
        l.push(3);
        l.push(2);
        l.push(1);
        l.push(0);
        assert_eq!(l.get_index(0), Some(0));

        assert_eq!(l.get(l.get_index(2).unwrap()), &2);

        assert_eq!(l.get_index(3), Some(3));
        l.delete(3);
        assert_eq!(l.get_index(3), None);

        assert_eq!(l.get_index(2), Some(2));
        l.delete(2);
        assert_eq!(l.get_index(2), None);

        assert_eq!(l.get(1), &1);
    }

    #[test]
    fn insert() { 
        let mut l = LinkedList::new();
        l.push(3);
        l.push(1);
        l.push(0);
        l.insert(2, 2);
        assert_eq!(l.get_index(2), Some(2));
    }
}
