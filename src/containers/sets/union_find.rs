use std::fmt;

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    nsets: usize
}

impl UnionFind {
    pub fn new(nsets: usize) -> Self {
        let mut parent = Vec::with_capacity(nsets);
        let mut size = Vec::with_capacity(nsets);
        (0..nsets).for_each(|i| {parent.push(i); size.push(1);});
        UnionFind{ parent, size, nsets}
    }

    pub fn find(&self, idx: usize) -> usize {
        assert!(idx < self.parent.len(), "`idx`: out of index");
        let mut mover_idx = idx;
        while mover_idx != self.parent[mover_idx] { mover_idx = self.parent[mover_idx]; }
        mover_idx
    }

    pub fn size(&self, idx: usize) -> usize {
        assert!(idx < self.parent.len(), "`idx`: out of index");
        self.size[self.find(idx)]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        assert!(x < self.parent.len() && y < self.parent.len(), "`x` and/or `y` out of index");
        if x == y { return }
        let x_root = self.find(x);
        let y_root = self.find(y);
        if self.size[x_root] < self.size[y_root] {
            self.parent[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
        } else {
            self.parent[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
        } 
        self.nsets -= 1;
    }
}

impl fmt::Display for UnionFind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       // TODO: need to walk up trees and print elements in each set
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.size.len(), 5);
        assert_eq!(uf.parent.len(), 5);
        assert_eq!(uf.nsets, 5);
        uf.union(0, 3);
        assert_eq!(uf.find(3), 0);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(3), 2);
        assert_eq!(uf.size(1), 1);
        uf.union(1, 0);
        assert_eq!(uf.find(1), 0);
        assert_eq!(uf.find(3), 0);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.size(1), 3);
        uf.union(4, 2);
        assert_eq!(uf.find(2), 4);
        assert_eq!(uf.find(4), 4);
        assert_eq!(uf.size(4), 2);
        assert_eq!(uf.size(4), 2);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(3), 0);
        assert_eq!(uf.find(1), 0);
        uf.union(2, 1);
        assert_eq!(uf.size(0), 5);
        assert_eq!(uf.size(1), 5);
        assert_eq!(uf.size(2), 5);
        assert_eq!(uf.size(3), 5);
        assert_eq!(uf.size(4), 5);
    }
}

