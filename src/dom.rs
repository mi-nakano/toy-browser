use std::collections::HashMap;

pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
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

pub fn comment(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Comment(data) }
}
