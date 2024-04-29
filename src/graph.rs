// initialize type Vertex, ListOfEdges, and AdjacencyList
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

// declare struct Graph with properties n and outedges
#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
}

// implement functions for Graph
impl Graph {
    // add more directed edges to outedges given a ListOfEdges
    pub fn add_directed_edges(&mut self,
                            edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    // sort the graph
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    // create a new Graph given a size and a ListOfEdges
    pub fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }

    // checks if a node exists in the outedges list.
    pub fn has_edges(&self, node: usize) -> bool {
        if let edges = &self.outedges[node] {
            !edges.is_empty()
        } else {
            false // Node not found in the adjacency list
        }
    }
}
