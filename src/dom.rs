use std::collections::HashMap;

struct Node {
    // data common to all nodes
    children: Vec<None>,

    // data specific to each node type:
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData)
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap
}

type AttrMap = HashMap<String, String>