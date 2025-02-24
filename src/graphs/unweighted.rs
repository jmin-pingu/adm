use std::mem;
use std::collections;
use std::fmt;

pub struct Graph {
    edges: Vec<Option<Box<Edge>>>,
    degrees: Vec<i32>,
    nedges: usize,   
    nvert: usize,   
    directed: bool,   
}

struct Edge {
    points_to: usize, 
    next: Option<Box<Edge>>, 
}

pub struct BreadthFirstSearcher<'a> {
    graph: &'a Graph,
    parents: Vec<Option<usize>>,
    discovered: Vec<bool>,
    processed: Vec<bool>
}

pub struct DepthFirstSearcher<'a> {
    graph: &'a Graph,
    parents: Vec<Option<usize>>,
    discovered: Vec<bool>,
    processed: Vec<bool>,

    entry_time: Vec<Option<usize>>,
    exit_time: Vec<Option<usize>>,
    time: usize,
    done: bool,
}

impl Graph {
    pub fn new(vcapacity: usize, directed: bool) -> Self {
        let mut edges = Vec::with_capacity(vcapacity);
        let mut degrees = Vec::with_capacity(vcapacity);
        (0..vcapacity).for_each(|_| {
            degrees.push(0);
            edges.push(None);
        });
        Graph {
            edges,
            degrees,
            nedges: 0,
            nvert: 0,
            directed,
        }
    }
 
    pub fn insert_edge(&mut self, i: usize, j: usize) {
        assert!(j < self.edges.len() && i < self.edges.len(), "vertices `i` and `j` must be within capacity");
        self.nedges += 1;
        self.nvert += 1;
        self.degrees[i] += 1;
        match mem::replace(&mut self.edges[i], None) {
            None => self.edges[i] = Some(Box::new(Edge::new(j, None))),
            edge => self.edges[i] = Some(Box::new(Edge::new(j, edge))),
        }

        if !self.directed {
            match mem::replace(&mut self.edges[j], None) {
                None => self.edges[j] = Some(Box::new(Edge::new(i, None))),
                edge => self.edges[j] = Some(Box::new(Edge::new(i, edge))),
            }
        }
    }

    pub fn init_bfs(&self) -> BreadthFirstSearcher {
        BreadthFirstSearcher::new(self)
    }

    pub fn init_dfs(&self) -> DepthFirstSearcher {
        DepthFirstSearcher::new(self)
    }

    pub fn connected_components(&self) -> usize {
        let mut bfs = self.init_bfs();
        let mut cc = 0;
        for i in 0..bfs.discovered.len() {
            if !bfs.discovered[i] {
                cc += 1;
                bfs.search_from(i, None, None, None);
            }
        }
        cc
    }

    pub fn find_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut builder = Vec::new();
        let mut bfs = self.init_bfs();
        bfs.search_from(start, None, None, None);
        let mut cur_vertex = end;
        builder.insert(0, cur_vertex);
        while let Some(parent) = bfs.parents[cur_vertex] {
            builder.insert(0, parent);
            if parent == start { return Some(builder) }
            cur_vertex = parent;
        }
        None 
    }

    pub fn find_cycle<'a>(&'a self, start: usize) {
        let mut dfs = self.init_dfs();
        fn process_edge<'a>(searcher: &'a mut DepthFirstSearcher<'_>, origin: usize, points_to: usize) {
            if searcher.parents[points_to].is_none() || origin != searcher.parents[points_to].unwrap() {
                print!("cycle found: {} {}", points_to, origin);
                let mut cur_vertex = origin;
                while let Some(parent) = searcher.parents[cur_vertex] {
                    print!(" {}", parent);
                    if parent == points_to { break; }
                    cur_vertex = parent;
                }
                println!("");
                searcher.done = true;
            }
        }
        dfs.search_from(start, None, Some(process_edge), None);
    }
}

impl<'a> BreadthFirstSearcher<'a> {
    fn new(graph: &'a Graph) -> Self {
        let mut parents = Vec::with_capacity(graph.edges.len());
        let mut processed = Vec::with_capacity(graph.edges.len());
        let mut discovered = Vec::with_capacity(graph.edges.len());
        (0..graph.edges.len()).for_each(|_| {
            processed.push(false); 
            discovered.push(false); 
            parents.push(None); 
        });
        BreadthFirstSearcher { graph, parents, discovered, processed }
    }

    fn search_from(
        &mut self, 
        start: usize,
        preprocess: Option<fn(&mut Self, usize)>, 
        process_edge: Option<fn(&mut Self, usize, usize)>, 
        postprocess: Option<fn(&mut Self, usize)>)
    { 
        let mut queue: collections::VecDeque<usize> = collections::VecDeque::new();
        queue.push_back(start);
        self.discovered[start] = true;
        while let Some(v) = queue.pop_front() {
            preprocess.map(|f| f(self, v));
            self.processed[v] = true;
            let mut cur_edge = &self.graph.edges[v];
            while let Some(incident) = cur_edge {
                if !self.processed[incident.points_to] || self.graph.directed {
                    process_edge.map(|f| f(self, v, incident.points_to));
                }
                if !self.discovered[incident.points_to] {
                    queue.push_back(incident.points_to); 
                    self.discovered[incident.points_to] = true;
                    self.parents[incident.points_to] = Some(v);
                }
                cur_edge = &incident.next;
            }
            postprocess.map(|f| f(self, v));
        }
    }
}

impl Edge {
    fn new(points_to: usize, next: Option<Box<Edge>>) -> Self { 
        Edge {
            points_to,
            next,
        }
    }
}

impl<'a> DepthFirstSearcher<'a> {
    fn new(graph: &'a Graph) -> Self {
        let mut parents = Vec::with_capacity(graph.edges.len());
        let mut processed = Vec::with_capacity(graph.edges.len());
        let mut discovered = Vec::with_capacity(graph.edges.len());
        let mut entry_time = Vec::with_capacity(graph.edges.len());
        let mut exit_time = Vec::with_capacity(graph.edges.len());
        (0..graph.edges.len()).for_each(|_| {
            processed.push(false); 
            discovered.push(false); 
            parents.push(None); 
            entry_time.push(None); 
            exit_time.push(None); 
        });
        DepthFirstSearcher { graph, parents, discovered, processed, time: 0, entry_time, exit_time, done: false }
    }

    pub fn search_from(
        &mut self, 
        start: usize, 
        preprocess: Option<fn(usize)>, 
        process_edge: Option<fn(&mut Self, usize, usize)>, 
        postprocess: Option<fn(usize)>) 
    { 
        if self.done { return; }
        self.discovered[start] = true;
        preprocess.map(|f| f(start));
        self.time += 1;
        self.entry_time[start] = Some(self.time);
        let mut cur_edge = &self.graph.edges[start];
        while let Some(v) = cur_edge {
            let not_loop = self.parents[v.points_to].is_some() && self.parents[v.points_to].unwrap() != start;
            if !self.discovered[v.points_to] {
                self.parents[v.points_to] = Some(start);
                process_edge.map(|f| f(self, start, v.points_to));
                self.search_from(v.points_to, preprocess, process_edge, postprocess);
            } else if (!self.processed[v.points_to] && not_loop) || self.graph.directed {
                process_edge.map(|f| f(self, start, v.points_to));
            }         
            if self.done { return; }
            cur_edge = &v.next;
        }
        postprocess.map(|f| f(start));
        self.time += 1;
        self.exit_time[start] = Some(self.time);
        self.processed[start] = true;
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.points_to)?;
        let mut cur_edge = &self.next;
        while let Some(edge) = cur_edge {
            write!(f, "{} ", edge.points_to)?;
            cur_edge = &edge.next;
        };
        Ok(())
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.edges.iter().enumerate().for_each(|(i, maybe_edge)| {
            maybe_edge.as_ref().map(|edge| writeln!(f, "{}: {}", i, edge).expect("writer out of memory"));
            ()
        });
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut graph = Graph::new(5, true);
        graph.insert_edge(0, 1);
         
        graph.insert_edge(1, 0);    
        graph.insert_edge(1, 3);

        graph.insert_edge(2, 4);
        graph.insert_edge(2, 1);
        println!("{}", graph);
    }

    #[test]
    fn find_path() {
        let mut graph = Graph::new(5, true);
        graph.insert_edge(0, 1);
         
        graph.insert_edge(1, 2);    
        graph.insert_edge(1, 3);

        graph.insert_edge(3, 4);
        graph.insert_edge(3, 0);

        assert_eq!(Some(vec![0, 1, 3, 4]), graph.find_path(0, 4));
        assert_eq!(None, graph.find_path(2, 4));
    }

    #[test]
    fn connected_components() {
        let mut graph = Graph::new(5, true);
        graph.insert_edge(0, 1);
         
        graph.insert_edge(1, 2);

        graph.insert_edge(3, 4);

        assert_eq!(2, graph.connected_components());

        let graph = Graph::new(5, true);
        assert_eq!(5, graph.connected_components());

        let mut graph = Graph::new(8, true);
        graph.insert_edge(0, 1);
        graph.insert_edge(2, 3);
        graph.insert_edge(4, 5);
        graph.insert_edge(6, 7);
        assert_eq!(4, graph.connected_components());

        let mut graph = Graph::new(5, true);
        graph.insert_edge(0, 1);
        graph.insert_edge(1, 2);
        graph.insert_edge(2, 3);
        graph.insert_edge(3, 4);
        assert_eq!(1, graph.connected_components());
    }

    #[test]
    fn dfs() {
        let mut graph = Graph::new(5, true);
        graph.insert_edge(0, 1);
        graph.insert_edge(4, 1);
         
        graph.insert_edge(1, 2);    
        graph.insert_edge(1, 3);

        graph.insert_edge(3, 4);
        graph.insert_edge(3, 0);
        graph.find_cycle(0);
    }
}


