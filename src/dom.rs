use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    // data common to all nodes
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData)
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap
}

// construct functions
pub fn text(content: String) -> Node {
    Node {
        children: Vec::new(), 
        node_type: NodeType::Text(content)
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
