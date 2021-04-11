use std::collections::{HashMap,HashSet};

pub type AttrMap = HashMap<String, String>

struct Node {
    // data common to all nodes
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData)
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap
}

// construct functions
fn text(content: String) -> Node {
    Node {
        children: Vec::new(), 
        node_type: NodeType::Text(content)
    }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
