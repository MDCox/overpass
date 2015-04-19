use std::collections::HashMap;
use db::edge::Edge;

#[derive(Clone, Debug)]
pub struct Node {
    edges: Vec<*mut Edge>,
    props: HashMap<&'static str, &'static str>
}

impl Node {
    pub fn new() -> Node {
        Node {
            edges: vec!(),
            props: HashMap::new()
        }
    }

    pub fn add_edge(&mut self, e: &mut Edge) { self.edges.push(e) }
    
    pub fn has_prop(&self, key: &'static str) -> bool {
        self.props.get(key).is_some() 
    }
    
    pub fn get_prop(&self, key: &'static str) -> Option<&&str> { 
        self.props.get(key) 
    }

    pub fn set_prop(&mut self, key: &'static str, val: &'static str) {
        self.props.insert(key,val);
    }
}
