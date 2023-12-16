use std::collections::VecDeque;
use std::collections::HashMap;


type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph{
    n: usize, 
    outedges: AdjacencyLists,
}

//reverses the direction of the edges because the graph is undirected 
pub fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
} 
 
impl Graph{
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph{
            n, 
            outedges: vec![vec![]; n],
        };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
    //creates the undirected graph as an adjacency list with the reversed edges
    pub fn create_undirected(n:usize, edges:&ListOfEdges) -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g       
    }
}

pub fn compute_distances_bfs(start: Vertex, graph: &Graph) -> Vec<Option<u32>> {
    let mut distances: Vec<Option<u32>> = vec![None; graph.n];
    distances[start] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for &u in &graph.outedges[v] {
            if distances[u].is_none() { // consider all unprocessed neighbors of v
                distances[u] = Some(distances[v].unwrap() + 1);
                queue.push_back(u);
            }
        }
    }
    distances
}


pub fn compute_average_distance(graph: &Graph) -> f64 {
    let mut total_distance = 0;
    let mut num_pairs = 0;

    for start in 0..graph.n {
        let distances = compute_distances_bfs(start, graph);

        for end in 0..graph.n {
            if let Some(dist) = distances[end] {
                total_distance += dist;
                num_pairs += 1;
            }
        }
    }

    if num_pairs > 0 {
        total_distance as f64 / num_pairs as f64
    } else {
        0.0
    }
}

//compute the degree of each node (the number of friends each person in the network has)
pub fn compute_degree_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let mut degree_centrality: HashMap<usize, f64> = HashMap::new();
    for (i, neighbors) in graph.outedges.iter().enumerate() {
        let degree = neighbors.len() as f64;
        degree_centrality.insert(i, degree);
    }
    degree_centrality
}