use std::{
    fmt, 
    iter, 
    mem,
    collections::HashSet,
    cmp::Ordering
};

use crate::containers::{
    priority_queue::heap,
    sets::union_find
};

#[derive(Debug)]
pub struct WeightedGraph {
    edges: Vec<Option<Box<WeightedEdge>>>,
    degrees: Vec<i32>,
    nedges: usize,   
    nvert: usize,   
    directed: bool,   
}

#[derive(Debug)]
struct WeightedEdge {
    weight: i32, 
    points_to: usize, 
    next: Option<Box<WeightedEdge>>, 
}
 
impl WeightedGraph {
    pub fn new(vcapacity: usize, directed: bool) -> Self {
        let mut edges = Vec::with_capacity(vcapacity);
        let mut degrees = Vec::with_capacity(vcapacity);
        (0..vcapacity).for_each(|_| {
            degrees.push(0);
            edges.push(None);
        });
        WeightedGraph {
            edges,
            degrees,
            nedges: 0,
            nvert: 0,
            directed,
        }
    }
 
    pub fn insert_edge(&mut self, i: usize, j: usize, weight: i32) {
        assert!(j < self.edges.len() && i < self.edges.len(), "vertices `i` and `j` must be within capacity");
        self.nedges += 1;
        self.nvert += 1;
        self.degrees[i] += 1;
        match mem::replace(&mut self.edges[i], None) {
            None => self.edges[i] = Some(Box::new(WeightedEdge::new(weight, j, None))),
            edge => self.edges[i] = Some(Box::new(WeightedEdge::new(weight, j, edge))),
        }

        if !self.directed {
            match mem::replace(&mut self.edges[j], None) {
                None => self.edges[j] = Some(Box::new(WeightedEdge::new(weight, i, None))),
                edge => self.edges[j] = Some(Box::new(WeightedEdge::new(weight, i, edge))),
            }
        }
    }

    pub fn prims<'a>(&'a self, start: usize) -> MinSpanTree<'a> {
        let mut distance: Vec<i32> = Vec::with_capacity(self.edges.len());
        let mut intree: Vec<bool> = Vec::with_capacity(self.edges.len());
        let mut parent: Vec<Option<usize>> = Vec::with_capacity(self.edges.len());
        let mut weight = 0;
        (0..self.edges.len()).for_each(|_| {
            distance.push(i32::MAX);
            parent.push(None);
            intree.push(false);
        });
        distance[start] = 0;
        let mut cur_vertex = start;
        while !intree[cur_vertex] {
            intree[cur_vertex] = true;
            if cur_vertex != start { weight += distance[cur_vertex] } 
            // NOTE: first, only look at neighbors and update if neighbor weight is less than
            // current smallest 
            let mut cur_edge = &self.edges[cur_vertex];
            while let Some(edge) = cur_edge.as_deref() {
                if distance[edge.points_to] > edge.weight { 
                    distance[edge.points_to] = edge.weight;
                    parent[edge.points_to] = Some(cur_vertex);
                }
                cur_edge = &edge.next;
            }
            // NOTE: choose the closest vertex NOT in our tree (where closest is guaranteed to
            // exist as we redefine all distances for immediate neighbors)
            let temp = match iter::zip(distance.iter(), intree.iter()).enumerate().filter(|(_, (_, &intree))| !intree).map(|(idx, (d, _))| (idx, d)).min_by_key(|(_, &d)| d) {
                None => break,
                Some(min) => min.0,
            };
            cur_vertex = temp;
        }
        MinSpanTree::new(self, parent, weight)
    }

    pub fn kruskals<'a>(&'a self) -> MinSpanTree<'a> {
        let mut parent: Vec<Option<usize>> = Vec::with_capacity(self.edges.len());
        let mut weight = 0;
        (0..self.edges.len()).for_each(|_| {
            parent.push(None);
        });

        let mut queue: heap::Heap<EdgePair> = heap::Heap::new();
        self.edges.iter().enumerate().for_each(|(idx, edge_list)| {
            edge_list.as_deref().map(|edge| {
                queue.insert(EdgePair::new(idx, edge.points_to, edge.weight));
                    
                let mut cur_edge = &edge.next;
                while let Some(e) = cur_edge {
                    queue.insert(EdgePair::new(idx, e.points_to, e.weight));
                    cur_edge = &e.next;
                }
            });
        });

        let mut set = union_find::UnionFind::new(queue.len());
        while let Some(edge) = queue.pop() {
            if !(set.find(edge.source) == set.find(edge.points_to)) {
                parent[edge.points_to] = Some(edge.source);
                weight += edge.weight;
                set.union(edge.source, edge.points_to);
            }
        }
        MinSpanTree::new(self, parent, weight)
    }

    pub fn dijkstras<'a>(&'a self, start: usize) -> ShortestPaths<'a> {
        let mut distance: Vec<i32> = Vec::with_capacity(self.edges.len());
        let mut intree: Vec<bool> = Vec::with_capacity(self.edges.len());
        let mut parent: Vec<Option<usize>> = Vec::with_capacity(self.edges.len());
        (0..self.edges.len()).for_each(|_| {
            distance.push(i32::MAX);
            parent.push(None);
            intree.push(false);
        });
        distance[start] = 0;
        let mut cur_vertex = start;
        while !intree[cur_vertex] {
            intree[cur_vertex] = true;
            let mut adj_v = &self.edges[cur_vertex];
            while let Some(edge) = adj_v.as_deref() {
                assert!(edge.weight > 0, "Dijkstra's algorithm does not work for graphs with negative weights");
                if distance[edge.points_to] > distance[cur_vertex] + edge.weight { 
                    distance[edge.points_to] = distance[cur_vertex] + edge.weight;
                    parent[edge.points_to] = Some(cur_vertex);
                }
                adj_v = &edge.next;
            }
            cur_vertex = match iter::zip(distance.iter(), intree.iter())
                .enumerate()
                .filter(|(_, (_, &intree))| !intree)
                .map(|(idx, (d, _))| (idx, d))
                .min_by_key(|(_, &d)| d) {
                None => break,
                Some(min) => min.0,
            };
        }
        ShortestPaths::new(self, start, parent, distance)
    }
}

// TODO: see if I can define a function to go through all incident vertices
// let mut adj_v = &self.edges[cur_vertex];
// while let Some(edge) = adj_v.as_deref() {
//     if distance[edge.points_to] > distance[cur_vertex] + edge.weight { 
//         distance[edge.points_to] = distance[cur_vertex] + edge.weight;
//         parent[edge.points_to] = Some(cur_vertex);
//     }
//     adj_v = &edge.next;
// }

#[derive(Copy, Clone, Debug)]
struct EdgePair {
    source: usize,
    points_to: usize,
    weight: i32,
}  

impl EdgePair {
    fn new(source: usize, points_to: usize, weight: i32) -> Self {
        EdgePair{ source, points_to, weight}
    }
}

#[derive(Debug)]
pub struct MinSpanTree<'a> {
    graph: &'a WeightedGraph,
    parents: Vec<Option<usize>>,
    total_weight: i32,
}

impl<'a> MinSpanTree<'a> {
    pub fn new(graph: &'a WeightedGraph, parents: Vec<Option<usize>>, total_weight: i32) -> Self {
        MinSpanTree { graph, parents, total_weight}
    }

}

#[derive(Debug)]
pub struct ShortestPaths<'a> {
    graph: &'a WeightedGraph,
    start: usize,
    parents: Vec<Option<usize>>,
    distance: Vec<i32>,
}

impl<'a> ShortestPaths<'a> {
    pub fn new(graph: &'a WeightedGraph, start: usize, parents: Vec<Option<usize>>, distance: Vec<i32>) -> Self {
        ShortestPaths { graph, start, parents, distance }
    }

    pub fn path_to(&self, end: usize) -> Option<Path> {  
        let mut cur_vertex = self.parents[end];
        let mut path = Vec::new();
        path.insert(0, end);
        while let Some(adj_v) = cur_vertex {
            path.insert(0, adj_v);
            if self.start == adj_v { return Some(Path::new(path, self.distance[end])); }
            cur_vertex = self.parents[adj_v];
        }
        None
    }
}

#[derive(Debug)]
pub struct Path {
    path: Vec<usize>,
    weight: i32,
}

impl Path {
    pub fn new(path: Vec<usize>, weight: i32) -> Self {
        Path { path, weight }
    }
}


impl WeightedEdge {
    fn new(weight: i32, points_to: usize, next: Option<Box<WeightedEdge>>) -> Self { 
        WeightedEdge {
            weight,
            points_to,
            next,
        }
    }
}

impl fmt::Display for WeightedEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[w:{}] ", self.points_to, self.weight)?;
        let mut cur_edge = &self.next;
        while let Some(edge) = cur_edge {
            write!(f, "{}[w:{}] ", edge.points_to, edge.weight)?;
            cur_edge = &edge.next;
        };
        Ok(())
    }
}

impl fmt::Display for WeightedGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.edges.iter().enumerate().for_each(|(i, maybe_edge)| {
            maybe_edge.as_ref().map(|edge| writeln!(f, "{}: {}", i, edge).expect("writer out of memory"));
            ()
        });
        Ok(())
    }
}
    

impl PartialEq for EdgePair {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight && self.source == other.source && self.points_to == other.points_to    
    }

    fn ne(&self, other: &Self) -> bool {
        !(self.weight == other.weight && self.source == other.source && self.points_to == other.points_to)
    }
}

impl PartialOrd for EdgePair {
    fn lt(&self, other: &Self) -> bool {
        self.weight < other.weight    
    }

    fn gt(&self, other: &Self) -> bool {
        self.weight > other.weight    
    }

    fn le(&self, other: &Self) -> bool {
        self.weight <= other.weight    
    }

    fn ge(&self, other: &Self) -> bool {
        self.weight >= other.weight    
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.weight <= other.weight, self.weight >= other.weight) {
            (true, true) => {
                Some(Ordering::Equal)
            },
            (false, true) => {
                Some(Ordering::Greater)
            },
            (true, false) => {
                Some(Ordering::Less)
            },
            _ => { panic!("impossible to have <= and >= both be false") }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prims() {
        let mut graph = WeightedGraph::new(7, false);
        graph.insert_edge(0, 1, 5);
        graph.insert_edge(0, 2, 7);
        graph.insert_edge(0, 3, 12);

        graph.insert_edge(1, 2, 9);
        graph.insert_edge(1, 4, 7);
         
        graph.insert_edge(2, 3, 4);
        graph.insert_edge(2, 4, 4);
        graph.insert_edge(2, 5, 3);

        graph.insert_edge(3, 5, 7);

        graph.insert_edge(4, 5, 2);
        graph.insert_edge(4, 6, 5);

        graph.insert_edge(5, 6, 2);
        (0..7).for_each(|start| assert_eq!(graph.prims(start).total_weight, 23));
    }

    #[test]
    fn kruskals() {
        let mut graph = WeightedGraph::new(7, false);
        graph.insert_edge(0, 1, 5);
        graph.insert_edge(0, 2, 7);
        graph.insert_edge(0, 3, 12);

        graph.insert_edge(1, 2, 9);
        graph.insert_edge(1, 4, 7);
         
        graph.insert_edge(2, 3, 4);
        graph.insert_edge(2, 4, 4);
        graph.insert_edge(2, 5, 3);

        graph.insert_edge(3, 5, 7);

        graph.insert_edge(4, 5, 2);
        graph.insert_edge(4, 6, 5);

        graph.insert_edge(5, 6, 2);
        assert_eq!(graph.kruskals().total_weight, 23);
    }

    #[test]
    fn dijkstras() {
        let mut graph = WeightedGraph::new(5, false);
        graph.insert_edge(0, 4, 5);
        graph.insert_edge(1, 4, 4);
        graph.insert_edge(2, 4, 3);
        graph.insert_edge(3, 4, 2);

        graph.insert_edge(0, 1, 1);
        graph.insert_edge(1, 2, 1);
        graph.insert_edge(2, 3, 1);
        graph.insert_edge(3, 4, 1);

        let shortest_paths = graph.dijkstras(0);
        assert_eq!(shortest_paths.path_to(4).unwrap().weight, 4);
        assert_eq!(shortest_paths.path_to(2).unwrap().weight, 2);
        assert_eq!(shortest_paths.path_to(1).unwrap().weight, 1);

        let shortest_paths = graph.dijkstras(1);
        assert_eq!(shortest_paths.path_to(4).unwrap().weight, 3);

        let shortest_paths = graph.dijkstras(2);
        assert_eq!(shortest_paths.path_to(4).unwrap().weight, 2);
 
        let shortest_paths = graph.dijkstras(3);
        assert_eq!(shortest_paths.path_to(4).unwrap().weight, 1);
    }
 }
