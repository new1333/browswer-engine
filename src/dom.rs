use std::collections::HashMap;

type AttrMap = HashMap<String, String>;

struct ElementData {
  tag_name: String,
  attributes: AttrMap,
}

enum NodeType {
  Text(String),
  Element(ElementData),
}

struct Node {
  children: Vec<Node>,
  node_type: NodeType,
}

fn text(data: String) -> Node {
  Node {
    node_type: NodeType::Text(data),
    children: Vec::new(),
  }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children,
    node_type: NodeType::Element(ElementData {
      tag_name: name,
      attributes: attrs,
    }),
  }
}
