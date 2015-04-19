use std::collections::HashMap;
use db::node::Node;

#[derive(Clone, Debug)]
pub struct Edge {
    pub source: *mut Node,
    pub target: *mut Node,
    props: HashMap<&'static str, &'static str>
}

impl Edge {
    pub fn new(source: &mut Node, target: &mut Node) -> Edge {
        Edge {
            source: source,
            target: target,
            props: HashMap::new()
        }
    }

    pub fn has_prop(&self, key: &'static str) -> bool {
        self.props.get(key).is_some()
    } 

    pub fn get_prop(&self, key: &'static str) -> Option<&&str> {
        self.props.get(key)
    }

    pub fn set_prop(&mut self, key: &'static str, val: &'static str) {
        self.props.insert(key, val);
    }
}
