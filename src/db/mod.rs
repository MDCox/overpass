use db::node::Node;
use db::edge::Edge;

mod cypher;
pub mod edge;
pub mod node;

pub struct Graph {
    pub nodes: Vec<Node>, // SHOULD NOT BE PUB FOR LONG
    pub edges: Vec<Edge>, // SHOULD NOT BE PUB FOR LONG
}

impl Graph {
    pub fn new() -> Graph { Graph { nodes: vec!(), edges: vec!() } }

    pub fn add_node(&mut self, n: Node) { self.nodes.push(n); }
    pub fn add_edge(&mut self, e: Edge) { 
        unsafe {
            (*e.source).add_edge(&mut e.clone());
        }

        self.edges.push(e);
    }

    pub fn get_nodes(&self, filter: Box<Fn(&Node) -> bool>) -> Vec<Node> {
        let mut matches = vec!();
        for n in &self.nodes {
            if filter(&n) { matches.push(n.clone()); }
        }
        matches
    }

    pub fn get_edges(&self, filter: Box<Fn(&Edge) -> bool>) -> Vec<Edge> {
        let mut matches = vec!();
        for e in &self.edges {
            if filter(&e) { matches.push(e.clone()); }
        }
        matches
    }

    pub fn eval(&mut self,  inp: String) {
        cypher::eval(inp, self)
    }
}
